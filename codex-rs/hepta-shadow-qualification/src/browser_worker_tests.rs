use tokio::io::AsyncWriteExt;

use super::*;

fn session_id() -> BrowserSessionId {
    BrowserSessionId::from_seed("browser-worker-tests").expect("valid session id")
}

fn request(
    request_id: u64,
    session_id: BrowserSessionId,
    generation: u64,
    owner_epoch: u64,
    page_revision: u64,
    command: BrowserCommand,
) -> BrowserRequest {
    BrowserRequest::new(
        request_id,
        session_id,
        BrowserActorKind::Agent,
        generation,
        owner_epoch,
        page_revision,
        command,
    )
}

fn handshake(
) -> Result<(BrowserWorkerParentSession, BrowserWorkerServerSession), BrowserWorkerProtocolError> {
    let session_id = session_id();
    let capability = BrowserWorkerStartupCapability::from_bytes([0x41; 32]);
    let capability_sha256 = capability.digest();
    let (mut parent, hello) = BrowserWorkerParentSession::begin(
        session_id.clone(),
        7,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )?;
    let mut server = BrowserWorkerServerSession::new(
        session_id,
        7,
        capability_sha256,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )?;
    let ready = match server.accept(hello)? {
        BrowserWorkerServerEvent::HandshakeAccepted { ready, parent_pid } => {
            assert_ne!(parent_pid, 0);
            ready
        }
        _ => panic!("expected handshake acceptance"),
    };
    assert!(matches!(
        parent.accept(ready)?,
        BrowserWorkerParentEvent::Ready {
            transport: BrowserWorkerTransportKind::QualificationStdioPipe,
            ..
        }
    ));
    assert!(parent.is_ready());
    assert!(server.is_ready());
    Ok((parent, server))
}

#[tokio::test]
async fn browser_worker_frame_round_trips_as_bounded_canonical_json() {
    let capability = BrowserWorkerStartupCapability::from_bytes([0x42; 32]);
    let (_, frame) = BrowserWorkerParentSession::begin(
        session_id(),
        9,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )
    .expect("valid parent session");
    let (mut writer, mut reader) = tokio::io::duplex(MAX_BROWSER_WORKER_FRAME_BYTES * 2);
    write_browser_worker_frame(&mut writer, &frame)
        .await
        .expect("write canonical frame");
    let decoded = read_browser_worker_frame(&mut reader)
        .await
        .expect("read canonical frame");
    assert_eq!(decoded, frame);
}

#[tokio::test]
async fn browser_worker_rejects_noncanonical_and_oversized_frames() {
    let capability = BrowserWorkerStartupCapability::from_bytes([0x43; 32]);
    let (_, frame) = BrowserWorkerParentSession::begin(
        session_id(),
        10,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )
    .expect("valid parent session");
    let mut pretty = serde_json::to_vec_pretty(&frame).expect("serialize pretty frame");
    let (mut writer, mut reader) = tokio::io::duplex(MAX_BROWSER_WORKER_FRAME_BYTES * 2);
    writer
        .write_all(&(pretty.len() as u32).to_be_bytes())
        .await
        .expect("write length");
    writer.write_all(&pretty).await.expect("write pretty frame");
    pretty.fill(0);
    assert!(matches!(
        read_browser_worker_frame(&mut reader).await,
        Err(BrowserWorkerProtocolError::NonCanonicalFrame)
    ));

    let (mut writer, mut reader) = tokio::io::duplex(16);
    writer
        .write_all(&((MAX_BROWSER_WORKER_FRAME_BYTES as u32) + 1).to_be_bytes())
        .await
        .expect("write oversized length");
    assert!(matches!(
        read_browser_worker_frame(&mut reader).await,
        Err(BrowserWorkerProtocolError::FrameTooLarge)
    ));
}

#[test]
fn browser_worker_rejects_wrong_or_replayed_startup_capability() {
    let session_id = session_id();
    let expected = BrowserWorkerStartupCapability::from_bytes([0x44; 32]);
    let expected_sha256 = expected.digest();
    drop(expected);
    let wrong = BrowserWorkerStartupCapability::from_bytes([0x45; 32]);
    let (_, wrong_hello) = BrowserWorkerParentSession::begin(
        session_id.clone(),
        11,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        wrong,
    )
    .expect("valid parent session");
    let mut server = BrowserWorkerServerSession::new(
        session_id.clone(),
        11,
        expected_sha256,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )
    .expect("valid server session");
    assert!(matches!(
        server.accept(wrong_hello),
        Err(BrowserWorkerProtocolError::WrongStartupCapability)
    ));
    assert!(server.is_closed());

    let capability = BrowserWorkerStartupCapability::from_bytes([0x46; 32]);
    let digest = capability.digest();
    let (_, hello) = BrowserWorkerParentSession::begin(
        session_id.clone(),
        12,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )
    .expect("valid parent session");
    let encoded = serde_json::to_vec(&hello).expect("serialize hello");
    let mut server = BrowserWorkerServerSession::new(
        session_id,
        12,
        digest,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )
    .expect("valid server session");
    let first: BrowserWorkerFrame = serde_json::from_slice(&encoded).expect("decode first hello");
    let second: BrowserWorkerFrame = serde_json::from_slice(&encoded).expect("decode replay hello");
    assert!(matches!(
        server.accept(first),
        Ok(BrowserWorkerServerEvent::HandshakeAccepted { .. })
    ));
    assert!(matches!(
        server.accept(second),
        Err(BrowserWorkerProtocolError::WrongSequence)
            | Err(BrowserWorkerProtocolError::UnexpectedFrame)
    ));
    assert!(server.is_closed());
}

#[test]
fn browser_worker_sequence_violation_fences_the_channel() {
    let (mut parent, mut server) = handshake().expect("handshake");
    let mut frame = parent
        .next_request(request(
            1,
            session_id(),
            7,
            1,
            0,
            BrowserCommand::Navigate {
                url: "fixture://shared-form".to_string(),
            },
        ))
        .expect("request frame");
    frame.sequence += 1;
    assert!(matches!(
        server.accept(frame),
        Err(BrowserWorkerProtocolError::WrongSequence)
    ));
    assert!(server.is_closed());
}

#[test]
fn browser_worker_transports_actor_request_and_response_with_closed_authority() {
    let (mut parent, mut server) = handshake().expect("handshake");
    let session_id = session_id();
    let mut actor = BrowserActor::new(session_id.clone(), 7, FixtureBrowserEngine::default())
        .expect("actor");
    let frame = parent
        .next_request(request(
            1,
            session_id,
            7,
            1,
            0,
            BrowserCommand::Navigate {
                url: "fixture://shared-form".to_string(),
            },
        ))
        .expect("request frame");
    let request = match server.accept(frame).expect("server accepts request") {
        BrowserWorkerServerEvent::Request(request) => request,
        _ => panic!("expected request"),
    };
    let response = actor.handle(request, 1).expect("actor response");
    assert!(response.authority.is_closed());
    let frame = server.next_response(response).expect("response frame");
    let response = match parent.accept(frame).expect("parent accepts response") {
        BrowserWorkerParentEvent::Response(response) => response,
        _ => panic!("expected response"),
    };
    assert_eq!(response.page_revision, 1);
    assert!(response.authority.is_closed());

    let shutdown = parent
        .next_shutdown(BrowserWorkerShutdownReason::QualificationComplete)
        .expect("shutdown frame");
    assert!(matches!(
        server.accept(shutdown).expect("server accepts shutdown"),
        BrowserWorkerServerEvent::Shutdown(BrowserWorkerShutdownReason::QualificationComplete)
    ));
    let acknowledgement = server.next_shutdown_ack().expect("shutdown acknowledgement");
    assert!(matches!(
        parent
            .accept(acknowledgement)
            .expect("parent accepts acknowledgement"),
        BrowserWorkerParentEvent::ShutdownAck
    ));
    assert!(parent.is_closed());
    assert!(server.is_closed());
}

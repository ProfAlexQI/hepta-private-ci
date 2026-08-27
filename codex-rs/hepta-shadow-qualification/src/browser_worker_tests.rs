use std::error::Error;
use std::io::Error as IoError;

use tokio::io::AsyncWriteExt;

use super::*;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

fn session_id() -> Result<BrowserSessionId, QualificationError> {
    BrowserSessionId::from_seed("browser-worker-tests")
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

fn handshake() -> TestResult<(BrowserWorkerParentSession, BrowserWorkerServerSession)> {
    let session_id = session_id()?;
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
        other => panic!("expected handshake acceptance, found {other:?}"),
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
async fn browser_worker_frame_round_trips_as_bounded_canonical_json() -> TestResult {
    let capability = BrowserWorkerStartupCapability::from_bytes([0x42; 32]);
    let (_, frame) = BrowserWorkerParentSession::begin(
        session_id()?,
        9,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )?;
    let (mut writer, mut reader) = tokio::io::duplex(MAX_BROWSER_WORKER_FRAME_BYTES * 2);
    write_browser_worker_frame(&mut writer, &frame).await?;
    let decoded = read_browser_worker_frame(&mut reader).await?;
    assert_eq!(decoded, frame);
    Ok(())
}

#[tokio::test]
async fn browser_worker_rejects_noncanonical_and_oversized_frames() -> TestResult {
    let capability = BrowserWorkerStartupCapability::from_bytes([0x43; 32]);
    let (_, frame) = BrowserWorkerParentSession::begin(
        session_id()?,
        10,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        capability,
    )?;
    let mut pretty = serde_json::to_vec_pretty(&frame)?;
    let pretty_length = u32::try_from(pretty.len())?;
    let (mut writer, mut reader) = tokio::io::duplex(MAX_BROWSER_WORKER_FRAME_BYTES * 2);
    writer.write_all(&pretty_length.to_be_bytes()).await?;
    writer.write_all(&pretty).await?;
    pretty.fill(0);
    assert!(matches!(
        read_browser_worker_frame(&mut reader).await,
        Err(BrowserWorkerProtocolError::NonCanonicalFrame)
    ));

    let oversized_length = u32::try_from(MAX_BROWSER_WORKER_FRAME_BYTES)?
        .checked_add(1)
        .ok_or_else(|| IoError::other("browser worker frame bound overflowed"))?;
    let (mut writer, mut reader) = tokio::io::duplex(16);
    writer.write_all(&oversized_length.to_be_bytes()).await?;
    assert!(matches!(
        read_browser_worker_frame(&mut reader).await,
        Err(BrowserWorkerProtocolError::FrameTooLarge)
    ));
    Ok(())
}

#[test]
fn browser_worker_rejects_wrong_or_replayed_startup_capability() -> TestResult {
    let session_id = session_id()?;
    let expected = BrowserWorkerStartupCapability::from_bytes([0x44; 32]);
    let expected_sha256 = expected.digest();
    drop(expected);
    let wrong = BrowserWorkerStartupCapability::from_bytes([0x45; 32]);
    let (_, wrong_hello) = BrowserWorkerParentSession::begin(
        session_id.clone(),
        11,
        BrowserWorkerTransportKind::QualificationStdioPipe,
        wrong,
    )?;
    let mut server = BrowserWorkerServerSession::new(
        session_id.clone(),
        11,
        expected_sha256,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )?;
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
    )?;
    let encoded = serde_json::to_vec(&hello)?;
    let mut server = BrowserWorkerServerSession::new(
        session_id,
        12,
        digest,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )?;
    let first: BrowserWorkerFrame = serde_json::from_slice(&encoded)?;
    let second: BrowserWorkerFrame = serde_json::from_slice(&encoded)?;
    assert!(matches!(
        server.accept(first),
        Ok(BrowserWorkerServerEvent::HandshakeAccepted { .. })
    ));
    assert!(matches!(
        server.accept(second),
        Err(BrowserWorkerProtocolError::WrongSequence)
    ));
    assert!(server.is_closed());
    Ok(())
}

#[test]
fn browser_worker_sequence_violation_fences_the_channel() -> TestResult {
    let (mut parent, mut server) = handshake()?;
    let mut frame = parent.next_request(request(
        1,
        session_id()?,
        7,
        1,
        0,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    ))?;
    frame.sequence = frame
        .sequence
        .checked_add(1)
        .ok_or_else(|| IoError::other("browser worker test sequence overflowed"))?;
    assert!(matches!(
        server.accept(frame),
        Err(BrowserWorkerProtocolError::WrongSequence)
    ));
    assert!(server.is_closed());
    Ok(())
}

#[test]
fn browser_worker_transports_actor_request_and_response_with_closed_authority() -> TestResult {
    let (mut parent, mut server) = handshake()?;
    let session_id = session_id()?;
    let mut actor = BrowserActor::new(session_id.clone(), 7, FixtureBrowserEngine::default())?;
    let frame = parent.next_request(request(
        1,
        session_id,
        7,
        1,
        0,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    ))?;
    let request = match server.accept(frame)? {
        BrowserWorkerServerEvent::Request(request) => request,
        other => panic!("expected request, found {other:?}"),
    };
    let response = actor.handle(request, 1)?;
    assert!(response.authority.is_closed());
    let frame = server.next_response(response)?;
    let response = match parent.accept(frame)? {
        BrowserWorkerParentEvent::Response(response) => response,
        other => panic!("expected response, found {other:?}"),
    };
    assert_eq!(response.page_revision, 1);
    assert!(response.authority.is_closed());

    let shutdown = parent.next_shutdown(BrowserWorkerShutdownReason::QualificationComplete)?;
    assert!(matches!(
        server.accept(shutdown)?,
        BrowserWorkerServerEvent::Shutdown(BrowserWorkerShutdownReason::QualificationComplete)
    ));
    let acknowledgement = server.next_shutdown_ack()?;
    assert!(matches!(
        parent.accept(acknowledgement)?,
        BrowserWorkerParentEvent::ShutdownAck
    ));
    assert!(parent.is_closed());
    assert!(server.is_closed());
    Ok(())
}

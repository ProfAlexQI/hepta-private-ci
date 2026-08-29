use std::error::Error;
use std::io::Cursor;

use hepta_browser_worker_protocol_qualification::AuthorityPosture;
use hepta_browser_worker_protocol_qualification::BrowserSessionId;
use hepta_browser_worker_protocol_qualification::CommandFrame;
use hepta_browser_worker_protocol_qualification::CommandKind;
use hepta_browser_worker_protocol_qualification::EstablishedBinding;
use hepta_browser_worker_protocol_qualification::FramedChannel;
use hepta_browser_worker_protocol_qualification::HostExpectedWorker;
use hepta_browser_worker_protocol_qualification::MAX_FRAME_BYTES;
use hepta_browser_worker_protocol_qualification::Message;
use hepta_browser_worker_protocol_qualification::OutcomeFrame;
use hepta_browser_worker_protocol_qualification::OutcomeStatus;
use hepta_browser_worker_protocol_qualification::ProtocolError;
use hepta_browser_worker_protocol_qualification::SourcePin;
use hepta_browser_worker_protocol_qualification::StartupCapability;
use hepta_browser_worker_protocol_qualification::WorkerIdentity;
use hepta_browser_worker_protocol_qualification::decode_message;
use hepta_browser_worker_protocol_qualification::encode_message;
use hepta_browser_worker_protocol_qualification::host_handshake;
use hepta_browser_worker_protocol_qualification::read_message;
use hepta_browser_worker_protocol_qualification::worker_handshake;
use hepta_browser_worker_protocol_qualification::write_message;

const SERVO_COMMIT: &str = "0a48e298482659817eb50097df23841f2b8e3044";
const SERVO_TREE: &str = "b04d2f75b3217374d079d579c270177b57fa1389";

fn expected_worker() -> Result<HostExpectedWorker, ProtocolError> {
    HostExpectedWorker::new(
        WorkerIdentity::new(BrowserSessionId::new([0x11; 32])?, 7, 9)?,
        SourcePin::new(SERVO_COMMIT, SERVO_TREE)?,
        StartupCapability::new([0x22; 32])?,
        [0x33; 32],
    )
}

#[test]
fn hello_round_trip_is_exact_and_negative_authority() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let message = Message::WorkerHello(expected.worker_hello()?);
    let encoded = encode_message(&message)?;
    assert_eq!(decode_message(&encoded)?, message);
    let Message::WorkerHello(decoded) = decode_message(&encoded)? else {
        return Err(std::io::Error::other("decoded a non-hello message").into());
    };
    assert_eq!(decoded.authority, AuthorityPosture::qualification_only());
    Ok(())
}

#[test]
fn command_and_outcome_round_trip() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let command = CommandFrame::new(
        41,
        expected.identity,
        3,
        CommandKind::TypeText {
            semantic_ref: "browser-ref:v1:3:submit".to_string(),
            text: "bounded fixture input".to_string(),
        },
    )?;
    let outcome = OutcomeFrame::new(
        41,
        expected.identity,
        4,
        OutcomeStatus::Completed,
        "fixture_type_completed",
    )?;
    for message in [Message::Command(command), Message::Outcome(outcome)] {
        assert_eq!(decode_message(&encode_message(&message)?)?, message);
    }
    Ok(())
}

#[test]
fn length_prefixed_io_round_trip() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let message = Message::Command(CommandFrame::new(
        9,
        expected.identity,
        2,
        CommandKind::Observe { limit: 32 },
    )?);
    let mut bytes = Vec::new();
    write_message(&mut bytes, &message)?;
    assert_eq!(read_message(&mut Cursor::new(bytes))?, message);
    Ok(())
}

#[test]
fn oversized_frame_is_rejected_before_allocation() -> Result<(), Box<dyn Error>> {
    let encoded_length = u32::try_from(MAX_FRAME_BYTES + 1)?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&encoded_length.to_be_bytes());
    let error = read_message(&mut Cursor::new(bytes))
        .err()
        .ok_or_else(|| std::io::Error::other("oversized frame unexpectedly decoded"))?;
    assert!(matches!(error, ProtocolError::FrameTooLarge { .. }));
    Ok(())
}

#[test]
fn truncated_and_trailing_frames_are_rejected() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let mut encoded = encode_message(&Message::WorkerHello(expected.worker_hello()?))?;
    let truncated_length = encoded.len().saturating_sub(1);
    assert!(decode_message(&encoded[..truncated_length]).is_err());
    encoded.push(0);
    assert!(decode_message(&encoded).is_err());
    Ok(())
}

#[test]
fn unknown_message_and_authority_bits_are_rejected() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let mut encoded = encode_message(&Message::WorkerHello(expected.worker_hello()?))?;
    encoded[10] = 0xff;
    assert!(decode_message(&encoded).is_err());
    assert!(AuthorityPosture::from_wire_bits(0).is_err());
    assert!(AuthorityPosture::from_wire_bits(3).is_err());
    assert!(AuthorityPosture::from_wire_bits(u16::MAX).is_err());
    Ok(())
}

#[test]
fn source_pin_and_startup_secrets_are_strict() -> Result<(), Box<dyn Error>> {
    assert!(SourcePin::new(&SERVO_COMMIT.to_uppercase(), SERVO_TREE).is_err());
    assert!(SourcePin::new("deadbeef", SERVO_TREE).is_err());
    assert!(StartupCapability::new([0; 32]).is_err());
    assert!(BrowserSessionId::new([0; 32]).is_err());
    let capability = StartupCapability::new([0x5a; 32])?;
    let rendered = format!("{capability:?}");
    assert_eq!(rendered, "StartupCapability(<redacted>)");
    assert!(!rendered.contains("5a5a"));
    Ok(())
}

#[test]
fn command_vocabulary_rejects_external_or_unbounded_inputs() {
    assert!(
        CommandKind::NavigateLocal {
            fixture_id: "https://example.com".to_string(),
        }
        .validate()
        .is_err()
    );
    assert!(
        CommandKind::NavigateLocal {
            fixture_id: "/etc/passwd".to_string(),
        }
        .validate()
        .is_err()
    );
    assert!(CommandKind::Observe { limit: 0 }.validate().is_err());
    assert!(
        CommandKind::HumanTakeover { lease_ms: 0 }
            .validate()
            .is_err()
    );
    assert!(
        CommandKind::TypeText {
            semantic_ref: "browser-ref:v1:1:field".to_string(),
            text: "\0".to_string(),
        }
        .validate()
        .is_err()
    );
}

#[cfg(unix)]
#[test]
fn inherited_unix_channel_completes_mutual_startup_handshake() -> Result<(), Box<dyn Error>> {
    use std::os::unix::net::UnixStream;

    let expected = expected_worker()?;
    let worker_expected = expected.clone();
    let (mut host_io, mut worker_io) = UnixStream::pair()?;
    let worker = std::thread::spawn(move || worker_handshake(&mut worker_io, &worker_expected));
    let host_binding = host_handshake(&mut host_io, &expected)?;
    let worker_binding = worker
        .join()
        .map_err(|_| std::io::Error::other("worker handshake thread panicked"))??;
    assert_eq!(host_binding, worker_binding);
    assert_eq!(host_binding.identity, expected.identity);
    assert_eq!(
        host_binding.authority,
        AuthorityPosture::qualification_only()
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn wrong_startup_capability_fails_closed() -> Result<(), Box<dyn Error>> {
    use std::os::unix::net::UnixStream;

    let host_expected = expected_worker()?;
    let worker_expected = HostExpectedWorker::new(
        host_expected.identity,
        host_expected.source_pin,
        StartupCapability::new([0x44; 32])?,
        *host_expected.host_nonce(),
    )?;
    let (mut host_io, mut worker_io) = UnixStream::pair()?;
    let worker = std::thread::spawn(move || worker_handshake(&mut worker_io, &worker_expected));
    let error = host_handshake(&mut host_io, &host_expected)
        .err()
        .ok_or_else(|| std::io::Error::other("wrong capability was accepted"))?;
    assert!(matches!(error, ProtocolError::AuthenticationFailed));
    drop(host_io);
    let _worker_result = worker
        .join()
        .map_err(|_| std::io::Error::other("worker handshake thread panicked"))?;
    Ok(())
}

#[test]
fn framed_channel_rejects_a_stale_identity_before_writing() -> Result<(), Box<dyn Error>> {
    let expected = expected_worker()?;
    let stale_identity = WorkerIdentity::new(expected.identity.session_id, 8, 9)?;
    let binding = EstablishedBinding {
        identity: expected.identity,
        source_pin: expected.source_pin,
        authority: AuthorityPosture::qualification_only(),
    };
    let message = Message::Command(CommandFrame::new(1, stale_identity, 1, CommandKind::Ping)?);
    let mut channel = FramedChannel::new(Cursor::new(Vec::<u8>::new()), binding);
    let error = channel
        .send(&message)
        .err()
        .ok_or_else(|| std::io::Error::other("stale command was written"))?;
    assert!(matches!(error, ProtocolError::StaleFence));
    assert!(channel.into_inner().into_inner().is_empty());
    Ok(())
}

use hepta_browser_worker_protocol_qualification::BrowserSessionId;
use hepta_browser_worker_protocol_qualification::HostAck;
use hepta_browser_worker_protocol_qualification::HostExpectedWorker;
use hepta_browser_worker_protocol_qualification::SourcePin;
use hepta_browser_worker_protocol_qualification::StartupCapability;
use hepta_browser_worker_protocol_qualification::WorkerConfirm;
use hepta_browser_worker_protocol_qualification::WorkerIdentity;

const COMMIT: &str = "0a48e298482659817eb50097df23841f2b8e3044";
const TREE: &str = "b04d2f75b3217374d079d579c270177b57fa1389";

fn identity() -> WorkerIdentity {
    WorkerIdentity::new(
        BrowserSessionId::new([0x33; 32]).expect("valid session"),
        1,
        1,
    )
    .expect("valid identity")
}

#[test]
fn startup_capability_and_nonce_debug_output_is_redacted() {
    let capability_pattern = "4141414141414141";
    let nonce_pattern = "4242424242424242";
    let expected = HostExpectedWorker::new(
        identity(),
        SourcePin::new(COMMIT, TREE).expect("valid pin"),
        StartupCapability::new([0x41; 32]).expect("valid capability"),
        [0x42; 32],
    )
    .expect("valid expected worker");
    let acknowledgement = HostAck::accepted(identity(), [0x42; 32]).expect("valid ack");
    let confirmation = WorkerConfirm::new(identity(), [0x42; 32]).expect("valid confirm");

    for rendered in (
        format!("{expected:?}"),
        format!("{acknowledgement:?}"),
        format!("{confirmation:?}"),
    ) {
        assert!(rendered.contains("<redacted>"));
        assert!(!rendered.contains(capability_pattern));
        assert!(!rendered.contains(nonce_pattern));
    }
}

#[test]
fn capability_is_cloneable_but_not_required_to_be_copy() {
    fn require_clone<T: Clone>(_value: &T) {}
    let capability = StartupCapability::new([0x55; 32]).expect("valid capability");
    require_clone(&capability);
    let clone = capability.clone();
    assert!(capability.matches(&clone));
}

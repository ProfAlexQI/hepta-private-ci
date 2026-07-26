use hepta_runtime::EffectBroker;
use hepta_runtime::EffectPlan;
use hepta_runtime::ExactExecutionAuthority;
use hepta_runtime::ExecutionAdmission;
use hepta_runtime::ExecutionIngress;
use hepta_runtime::ProviderEffectAck;
use hepta_runtime::TerminalEffectReceipt;

fn hex(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn admission() -> ExecutionAdmission {
    ExecutionAdmission::new(
        ExecutionIngress::NativeGateway,
        "operator-note-write",
        ExactExecutionAuthority::new(hex('a'), hex('b'), hex('c')).unwrap(),
        hex('d'),
        format!("sha256:{}", hex('e')),
    )
    .unwrap()
}

#[test]
fn lifecycle_requires_admission_plan_ack_and_terminal_order() {
    let admission = admission();
    let mut broker = EffectBroker::admit(admission.clone());
    let plan = EffectPlan::new(
        admission.admission_hash(),
        "write_file",
        "fixed-local-operator-note",
        format!("sha256:{}", hex('f')),
        hex('1'),
    )
    .unwrap();
    broker.record_effect_plan(plan.clone()).unwrap();
    let ack = ProviderEffectAck::new(
        plan.effect_plan_hash(),
        "runtime-kernel:write_file",
        format!("sha256:{}", hex('2')),
    )
    .unwrap();
    broker.record_provider_ack(ack.clone()).unwrap();
    let receipt = TerminalEffectReceipt::succeeded(
        ack.ack_hash(),
        format!("sha256:{}", hex('3')),
        format!("sha256:{}", hex('4')),
    )
    .unwrap();
    broker.record_terminal_receipt(receipt).unwrap();
    assert!(
        broker
            .completed_receipt_hash()
            .unwrap()
            .starts_with("sha256:")
    );
}

#[test]
fn mismatched_effect_plan_is_denied() {
    let mut broker = EffectBroker::admit(admission());
    let plan = EffectPlan::new(
        format!("sha256:{}", hex('9')),
        "write_file",
        "fixed-local-operator-note",
        format!("sha256:{}", hex('f')),
        hex('1'),
    )
    .unwrap();
    assert!(broker.record_effect_plan(plan).is_err());
}

#[test]
fn provider_ack_before_effect_plan_is_denied() {
    let mut broker = EffectBroker::admit(admission());
    let ack = ProviderEffectAck::new(
        format!("sha256:{}", hex('2')),
        "runtime-kernel:write_file",
        format!("sha256:{}", hex('3')),
    )
    .unwrap();
    assert!(broker.record_provider_ack(ack).is_err());
}

#[test]
fn terminal_receipt_before_provider_ack_is_denied() {
    let mut broker = EffectBroker::admit(admission());
    let receipt = TerminalEffectReceipt::succeeded(
        format!("sha256:{}", hex('2')),
        format!("sha256:{}", hex('3')),
        format!("sha256:{}", hex('4')),
    )
    .unwrap();
    assert!(broker.record_terminal_receipt(receipt).is_err());
}

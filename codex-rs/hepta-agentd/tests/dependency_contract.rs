use codex_hepta_agentd::AgentdRequest;
use pretty_assertions::assert_eq;

#[test]
fn public_agentd_reexport_preserves_protocol_round_trip() {
    let request = AgentdRequest::health(73, 5);
    let encoded = serde_json::to_vec(&request).expect("serialize agentd request");
    let decoded =
        serde_json::from_slice::<AgentdRequest>(&encoded).expect("parse agentd request");

    assert_eq!(decoded, request);
}

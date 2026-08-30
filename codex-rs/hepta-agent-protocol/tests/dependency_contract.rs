use codex_hepta_agent_protocol::AgentdRequest;
use pretty_assertions::assert_eq;

#[test]
fn health_request_round_trip_keeps_exact_control_identity() {
    let request = AgentdRequest::health(41, 7);
    let encoded = serde_json::to_vec(&request).expect("serialize health request");
    let decoded =
        serde_json::from_slice::<AgentdRequest>(&encoded).expect("parse health request");

    assert_eq!(decoded, request);
}

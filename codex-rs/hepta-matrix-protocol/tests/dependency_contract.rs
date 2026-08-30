use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use pretty_assertions::assert_eq;

#[test]
fn matrix_identifiers_round_trip_as_exact_wire_strings() {
    let room = MatrixRoomId::parse("!room:example.org").expect("valid room id");
    let event = MatrixEventId::parse("$event:example.org").expect("valid event id");

    let room_json = serde_json::to_string(&room).expect("serialize room id");
    let event_json = serde_json::to_string(&event).expect("serialize event id");

    assert_eq!(
        serde_json::from_str::<MatrixRoomId>(&room_json).expect("parse room id"),
        room
    );
    assert_eq!(
        serde_json::from_str::<MatrixEventId>(&event_json).expect("parse event id"),
        event
    );
}

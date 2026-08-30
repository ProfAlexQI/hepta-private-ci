use codex_hepta_matrix_protocol::MatrixEventId;
use futures::StreamExt;
use futures::stream;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn timeline_test_stream_preserves_matrix_event_order() {
    let events = vec![
        MatrixEventId::parse("$event-1:example.org").expect("first event id"),
        MatrixEventId::parse("$event-2:example.org").expect("second event id"),
        MatrixEventId::parse("$event-3:example.org").expect("third event id"),
    ];
    let expected = events.iter().map(ToString::to_string).collect::<Vec<_>>();

    let observed = stream::iter(events)
        .map(|event| event.to_string())
        .collect::<Vec<_>>()
        .await;

    assert_eq!(observed, expected);
}

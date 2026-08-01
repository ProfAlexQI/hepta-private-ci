fn route_contract_body(endpoint: &str) -> String {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request("GET", endpoint, &options);
    assert_eq!(status, "200 OK", "{endpoint}");
    assert_eq!(content_type, "application/json; charset=utf-8", "{endpoint}");
    body
}

const CONTRACT_FIXTURE_SOURCES: [&str; 14] = [
    include_str!("../tests.rs"),
    include_str!("contract_fixture.rs"),
    include_str!("contract_part_01.rs"),
    include_str!("contract_part_02.rs"),
    include_str!("contract_part_03.rs"),
    include_str!("contract_part_04.rs"),
    include_str!("contract_part_05.rs"),
    include_str!("contract_part_06.rs"),
    include_str!("contract_part_07.rs"),
    include_str!("contract_part_08.rs"),
    include_str!("contract_part_09.rs"),
    include_str!("contract_part_10.rs"),
    include_str!("contract_part_11.rs"),
    include_str!("contract_part_12.rs"),
];

#[test]
fn gateway_contract_fixture_family_stays_within_line_budget() {
    let lines = CONTRACT_FIXTURE_SOURCES
        .iter()
        .map(|source| source.lines().count())
        .sum::<usize>();
    assert!(
        lines <= 48_534,
        "gateway contract fixture family has {lines} lines, max 48534"
    );
}

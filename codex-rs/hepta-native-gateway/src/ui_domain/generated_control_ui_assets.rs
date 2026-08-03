pub(crate) const NATIVE_GATEWAY_BINARY_ASSET_PATHS: &[&str] = &["/assets/hepta-agent-logo.png", "/control-ui.js"];

#[rustfmt::skip]
pub(super) fn native_gateway_binary_asset(path: &str) -> Option<NativeGatewayBinaryAsset> {
    match path {
        "/assets/hepta-agent-logo.png" => Some(NativeGatewayBinaryAsset {
    content_type: "image/png",
    cache_control: "public, max-age=3600, must-revalidate",
    etag: "\"sha256-eeaa030d321f566157055ae07d5f540bf06baeaaa6a2e6209227eb1bf6d3f41c\"",
    body: hepta_core::control_ui::CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
}),
        "/control-ui.js" => Some(NativeGatewayBinaryAsset {
    content_type: "text/javascript; charset=utf-8",
    cache_control: "public, max-age=3600, must-revalidate",
    etag: "\"sha256-026cc7b50e3be8173b50dfb6fcf38b45b7cebd31b1f22b22ae99910ddef5b1a9\"",
    body: hepta_core::control_ui::CONTROL_UI_JS,
}),
        _ => None,
    }
}

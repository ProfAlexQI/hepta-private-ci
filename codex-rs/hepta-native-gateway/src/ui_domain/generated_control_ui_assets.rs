pub(crate) const NATIVE_GATEWAY_BINARY_ASSET_PATHS: &[&str] = &["/assets/hepta-agent-logo.png", "/assets/k.png"];

#[rustfmt::skip]
pub(super) fn native_gateway_binary_asset(path: &str) -> Option<NativeGatewayBinaryAsset> {
    match path {
        "/assets/hepta-agent-logo.png" => Some(NativeGatewayBinaryAsset {
    content_type: "image/png",
    cache_control: "public, max-age=3600, must-revalidate",
    etag: "\"sha256-eeaa030d321f566157055ae07d5f540bf06baeaaa6a2e6209227eb1bf6d3f41c\"",
    body: hepta_core::control_ui::CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
}),
        "/assets/k.png" => Some(NativeGatewayBinaryAsset {
    content_type: "image/png",
    cache_control: "public, max-age=3600, must-revalidate",
    etag: "\"sha256-a54bc0d6352c3130d2d22b7df80f1fabaa94f5098fec12046e4f262e6d0d7c28\"",
    body: hepta_core::control_ui::CONTROL_UI_GLASS_K_PNG,
}),
        _ => None,
    }
}

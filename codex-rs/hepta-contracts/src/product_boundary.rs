/// Stable product role for the formal Hepta service.
pub const OPENCLAW_GOVERNED_BACKEND_ROLE: &str = "openclaw_governed_backend";
/// Product-boundary disposition for retired Control UI mutation surfaces.
pub const LEGACY_CONTROL_UI_MUTATION_DISPOSITION: &str =
    "compatibility_plan_only_retired_for_real_effects";
/// Product-boundary disposition for the legacy Telegram replacement surface.
pub const LEGACY_TELEGRAM_REPLACEMENT_DISPOSITION: &str =
    "compatibility_readiness_only_owner_remains_openclaw";
/// Canonical next action for retired Control UI mutation surfaces.
pub const LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION: &str = "keep legacy Control UI POST routes plan-only; use governed plugin, preference, operator-note, or Telegram authority entrypoints for real effects";
/// Canonical next action for the legacy Telegram replacement surface.
pub const LEGACY_TELEGRAM_REPLACEMENT_NEXT_ACTION: &str = "keep legacy OpenClaw as Telegram owner; treat this endpoint as compatibility readiness only unless a separate controlled-live ownership migration is approved";
/// Stable default-off product boundary shared by reports, watchdogs, and tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductBoundarySpec {
    /// Product role of the formal Hepta service.
    pub product_role: &'static str,
    /// Channel owner under the accepted boundary.
    pub channel_owner: &'static str,
    /// Whether Telegram external reads are enabled by default.
    pub telegram_external_read: bool,
    /// Whether Telegram external sends are enabled by default.
    pub telegram_external_send: bool,
    /// Whether Hepta owns the Telegram polling loop by default.
    pub telegram_poll_loop_owner: bool,
    /// Whether native real mutation is enabled by default.
    pub native_real_mutation: bool,
    /// Whether the formal service links the full provider/tool runner.
    pub formal_service_links_full_codex_provider_tool_runner: bool,
}
/// Accepted product boundary for Hepta.
pub const PRODUCT_BOUNDARY: ProductBoundarySpec = ProductBoundarySpec {
    product_role: OPENCLAW_GOVERNED_BACKEND_ROLE,
    channel_owner: "legacy_openclaw",
    telegram_external_read: false,
    telegram_external_send: false,
    telegram_poll_loop_owner: false,
    native_real_mutation: false,
    formal_service_links_full_codex_provider_tool_runner: false,
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn governed_backend_boundary_is_default_off() {
        assert_eq!(
            PRODUCT_BOUNDARY.product_role,
            OPENCLAW_GOVERNED_BACKEND_ROLE
        );
        assert_eq!(PRODUCT_BOUNDARY.channel_owner, "legacy_openclaw");
        const {
            assert!(!PRODUCT_BOUNDARY.telegram_external_read);
            assert!(!PRODUCT_BOUNDARY.telegram_external_send);
            assert!(!PRODUCT_BOUNDARY.telegram_poll_loop_owner);
            assert!(!PRODUCT_BOUNDARY.native_real_mutation);
            assert!(!PRODUCT_BOUNDARY.formal_service_links_full_codex_provider_tool_runner);
        }
        assert!(LEGACY_CONTROL_UI_MUTATION_DISPOSITION.contains("retired"));
        assert!(LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION.contains("plan-only"));
        assert!(LEGACY_TELEGRAM_REPLACEMENT_DISPOSITION.contains("owner_remains_openclaw"));
    }
}

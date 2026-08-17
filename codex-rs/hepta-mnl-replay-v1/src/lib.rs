//! Linux replay-store and pre-run clock-order publisher for the scoped MNL
//! successor.
//!
//! Production policy is deliberately absent. This crate does not grant live
//! execution authority; it will only expose durable publication inspections
//! after the filesystem implementation and qualification gates are complete.

#![forbid(unsafe_code)]

#[cfg(target_os = "linux")]
mod boot_id;
#[cfg(target_os = "linux")]
mod clock;
mod error;
#[cfg(target_os = "linux")]
mod secure_fs;
#[cfg(target_os = "linux")]
mod store;
#[cfg(target_os = "linux")]
mod supervisor;

#[cfg(target_os = "linux")]
pub use boot_id::derive_linux_boot_id_sha256;
pub use error::ReplayStoreErrorV1;
pub use error::ReplayStoreResultV1;
#[cfg(target_os = "linux")]
pub use store::DurableReplayPublicationInspectionV1;
#[cfg(target_os = "linux")]
pub use store::ReplayStoreAnchorV1;
#[cfg(target_os = "linux")]
pub use store::ReplayStorePolicyV1;
#[cfg(target_os = "linux")]
pub use store::open_production_replay_store;
#[cfg(target_os = "linux")]
pub use store::open_replay_store;
#[cfg(target_os = "linux")]
pub use store::publish_copy_ack_claim_once;
#[cfg(target_os = "linux")]
pub use store::publish_pre_run_claim_once;
#[cfg(target_os = "linux")]
pub use supervisor::PRODUCTION_WALL_CLOCK_SUPERVISOR_POLICY_AVAILABLE;
#[cfg(target_os = "linux")]
pub use supervisor::PreRunClockPublicationInspectionV1;
#[cfg(target_os = "linux")]
pub use supervisor::inspect_pre_run_clock_publication_sequence;
#[cfg(target_os = "linux")]
pub use supervisor::require_production_wall_clock_supervisor_policy;

pub const PRODUCTION_REPLAY_STORE_POLICY_AVAILABLE: bool = false;

pub fn require_production_replay_store_policy() -> ReplayStoreResultV1<()> {
    Err(ReplayStoreErrorV1::Blocked(
        "compiled production replay-store policy is absent",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_absent() {
        let error = require_production_replay_store_policy()
            .expect_err("production replay-store policy must remain absent");
        assert!(matches!(error, ReplayStoreErrorV1::Blocked(_)));
    }
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests;
#[cfg(all(test, target_os = "linux"))]
mod supervisor_tests;

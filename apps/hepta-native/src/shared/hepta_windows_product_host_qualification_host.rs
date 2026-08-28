//! Fail-closed, qualification-only Windows material host.
//!
//! This module is split into deterministic repository-tracked source parts to
//! keep the qualification state machine reviewable while preserving one
//! canonical compilation surface.

include!("hepta_windows_product_host_qualification_host/part1.rs");
include!("hepta_windows_product_host_qualification_host/part2.rs");
include!("hepta_windows_product_host_qualification_host/part3.rs");
include!("hepta_windows_product_host_qualification_host/part4.rs");

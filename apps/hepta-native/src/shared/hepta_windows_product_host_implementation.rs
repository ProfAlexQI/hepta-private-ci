//! Disabled-by-default Windows product-material host candidate.
//!
//! This module is intentionally not registered in the product Script graph or
//! lifecycle. It models the explicit activation, verified binding, rollback,
//! suspend, and shutdown transaction that a future product tranche may invoke
//! only after a sealed review envelope, operator acceptance, physical-device
//! validation, and rollback drill all exist for the same candidate.

include!("hepta_windows_product_host_implementation/part1.rs");
include!("hepta_windows_product_host_implementation/part2.rs");
include!("hepta_windows_product_host_implementation/part3.rs");

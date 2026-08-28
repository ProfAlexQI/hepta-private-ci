//! Default-off Windows product-host wiring core.
//!
//! This module closes the source-level seam between the qualified review envelope,
//! the explicit product-host transaction candidate, and the existing audited DWM
//! set/readback surface. It never activates automatically and grants no authority.

include!("hepta_windows_product_host_wiring/part1.rs");
include!("hepta_windows_product_host_wiring/part2.rs");
include!("hepta_windows_product_host_wiring/part3.rs");
include!("hepta_windows_product_host_wiring/part4.rs");

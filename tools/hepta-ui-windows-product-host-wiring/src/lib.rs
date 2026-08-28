#![deny(unsafe_op_in_unsafe_fn)]

#[path = "../../../../apps/hepta-native/src/shared/hepta_platform_material.rs"]
pub mod hepta_platform_material;
#[path = "../../../../apps/hepta-native/src/shared/hepta_platform_material_runtime.rs"]
pub mod hepta_platform_material_runtime;
#[path = "../../../../apps/hepta-native/src/shared/hepta_windows_material_adapter.rs"]
pub mod hepta_windows_material_adapter;
#[path = "../../../../apps/hepta-native/src/shared/hepta_windows_product_host_integration_review.rs"]
pub mod hepta_windows_product_host_integration_review;
#[path = "../../../../apps/hepta-native/src/shared/hepta_windows_product_host_implementation.rs"]
pub mod hepta_windows_product_host_implementation;
#[path = "../../../../apps/hepta-native/src/shared/hepta_windows_product_host_wiring.rs"]
pub mod hepta_windows_product_host_wiring;

// Compatibility namespace for canonical source tests that address modules via
// `crate::shared::*` in the full Native product.
pub mod shared {
    pub use crate::hepta_platform_material;
    pub use crate::hepta_platform_material_runtime;
    pub use crate::hepta_windows_material_adapter;
    pub use crate::hepta_windows_product_host_implementation;
    pub use crate::hepta_windows_product_host_integration_review;
    pub use crate::hepta_windows_product_host_wiring;
}

#[path = "../../../apps/hepta-native/src/shared/hepta_windows_product_host_integration_review.rs"]
mod hepta_windows_product_host_integration_review;

#[path = "../../../apps/hepta-native/src/shared/hepta_windows_product_host_qualification_host.rs"]
mod hepta_windows_product_host_qualification_host;

#[cfg(not(all(target_os = "windows", feature = "hepta_ui_windows_system_material_v4")))]
fn main() {
    println!(
        "hepta-ui-windows-product-host-device-drill: Windows explicit-feature runtime required"
    );
}

#[cfg(all(target_os = "windows", feature = "hepta_ui_windows_system_material_v4"))]
mod windows_drill {
    include!("windows_drill/part1.rs");
    include!("windows_drill/part2.rs");
    include!("windows_drill/part3.rs");
    include!("windows_drill/part4.rs");
}

#[cfg(all(target_os = "windows", feature = "hepta_ui_windows_system_material_v4"))]
fn main() {
    windows_drill::main();
}

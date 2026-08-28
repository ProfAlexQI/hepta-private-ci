#[cfg(not(all(target_os = "windows", feature = "hepta_ui_windows_system_material_v4")))]
fn main() {
    eprintln!(
        "hepta-ui-windows-product-host-device-drill requires Windows and the explicit feature"
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

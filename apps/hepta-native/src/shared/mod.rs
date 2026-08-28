use makepad_widgets::ScriptVm;

pub mod attachment_download;
pub mod avatar;
pub mod bouncing_dots;
pub mod collapsible_header;
pub mod confirmation_modal;
pub mod expand_arrow;
pub mod file_upload_modal;
pub mod helpers;
pub mod hepta_makepad_window_material;
pub mod hepta_material_app_lifecycle;
pub mod hepta_platform_material;
pub mod hepta_platform_material_host;
pub mod hepta_platform_material_runtime;
pub mod hepta_system_preferences;
pub mod hepta_theme;
pub mod hepta_v4;
pub mod hepta_v4_controls;
pub mod hepta_v4_layout;
pub mod hepta_windows_material_adapter;
pub mod html_or_plaintext;
pub mod icon_button;
pub mod image_viewer;
pub mod jump_to_bottom_button;
pub mod mention_popup;
pub mod mentionable_text_input;
pub mod navigation_bar_button;
pub mod popup_list;
pub mod progress_bar;
pub mod restore_status_view;
pub mod room_filter_input_bar;
pub mod room_input_popup_menu;
pub mod slash_commands;
pub mod styles;
pub mod text_or_image;
pub mod timestamp;
pub mod unread_badge;
pub mod verification_badge;

pub fn script_mod(vm: &mut ScriptVm) {
    // Order matters here, as some widget definitions depend on others.
    hepta_theme::script_mod(vm);
    styles::script_mod(vm);
    hepta_v4::script_mod(vm);
    // Concrete migration templates load after legacy styles and semantic v4
    // overrides, before downstream consumers instantiate compatibility names.
    hepta_v4_controls::script_mod(vm);
    hepta_v4_layout::script_mod(vm);
    // The lifecycle module rebinds the canonical Window prototype before the
    // App root is evaluated. Full-profile OS material remains explicitly
    // unbound; its Makepad WindowVisuals controller can only queue a partial
    // persistent-chrome request for an exact framework WindowId.
    hepta_material_app_lifecycle::script_mod(vm);
    helpers::script_mod(vm);
    icon_button::script_mod(vm);
    navigation_bar_button::script_mod(vm);
    expand_arrow::script_mod(vm);
    unread_badge::script_mod(vm);
    collapsible_header::script_mod(vm);
    timestamp::script_mod(vm);
    room_filter_input_bar::script_mod(vm);
    room_input_popup_menu::script_mod(vm);
    avatar::script_mod(vm);
    text_or_image::script_mod(vm);
    html_or_plaintext::script_mod(vm);
    bouncing_dots::script_mod(vm);
    jump_to_bottom_button::script_mod(vm);
    popup_list::script_mod(vm);
    verification_badge::script_mod(vm);
    mention_popup::script_mod(vm);
    mentionable_text_input::script_mod(vm);
    restore_status_view::script_mod(vm);
    confirmation_modal::script_mod(vm);
    image_viewer::script_mod(vm);
    progress_bar::script_mod(vm);
    file_upload_modal::script_mod(vm);
}

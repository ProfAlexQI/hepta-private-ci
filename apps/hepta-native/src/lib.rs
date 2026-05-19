#![recursion_limit = "256"]

use std::{path::Path, sync::OnceLock};

use makepad_widgets::ScriptNew;
use robius_directories::ProjectDirs;

pub use makepad_widgets;

#[macro_export]
macro_rules! live {
    ($($tt:tt)*) => {
        makepad_widgets::script! { $($tt)* }
    };
}

pub type LivePtr = makepad_widgets::ScriptValue;


pub fn widget_ref_from_live_ptr(
    cx: &mut makepad_widgets::Cx,
    ptr: Option<LivePtr>,
) -> makepad_widgets::WidgetRef {
    ptr.map_or_else(makepad_widgets::WidgetRef::empty, |value| {
        cx.with_vm(|vm| makepad_widgets::WidgetRef::script_from_value(vm, value))
    })
}

pub fn view_from_live_ptr(
    cx: &mut makepad_widgets::Cx,
    ptr: Option<LivePtr>,
) -> makepad_widgets::View {
    cx.with_vm(|vm| match ptr {
        Some(value) => makepad_widgets::View::script_from_value(vm, value),
        None => makepad_widgets::View::script_new(vm),
    })
}

/// The top-level main application module.
pub mod app;
/// Function for loading and saving persistent application/session state.
pub mod persistence;
/// The settings screen and settings-related content/widgets.
pub mod settings;

/// Login screen
pub mod login;
/// Logout confirmation and state management
pub mod logout;
/// Core UI content: the main home screen (rooms list), room screen.
pub mod home;
/// User profile info and a user profile sliding pane.
pub mod profile;
/// A modal/dialog popup for interactive verification of users/devices.
mod verification_modal;
/// A modal/dialog popup for joining/leaving rooms, including confirming invite accept/reject.
mod join_leave_room_modal;
/// Shared UI components.
pub mod shared;
/// Generating text previews of timeline events/messages.
mod event_preview;
/// Local Hepta fixture mode for Matrix-heart UI development.
pub mod hepta_fixture;
/// Local bounded fixture smoke report.
pub mod hepta_fixture_smoke;
/// Local Hepta composer command parser for the Matrix-heart action bridge.
pub mod hepta_composer;
/// Local validated quick-command templates.
pub mod hepta_command_templates;
/// Local read-only context snapshot model.
pub mod hepta_context_snapshot;
/// Local side-effect-free action bridge policy.
pub mod hepta_action_bridge;
/// Local side-effect-free action outbox model.
pub mod hepta_action_queue;
/// Local read-only runtime status model.
pub mod hepta_runtime_status;
/// Local mobile packaging gate status model.
pub mod hepta_mobile_packaging;
/// Local Hepta Native productization status model.
pub mod hepta_productization;
/// Local Hepta-runtime-to-Matrix event shape helpers.
pub mod hepta_bridge;
/// Local bridge to the current codex-rs/hepta-* runtime crates.
pub mod hepta_runtime_bridge;
/// Hepta custom Matrix-style event helpers.
pub mod hepta_event;
pub mod room;


/// All content related to TSP (Trust Spanning Protocol) wallets/identities.
#[cfg(feature = "tsp")]
pub mod tsp;
/// Dummy TSP module with placeholder widgets, for builds without TSP.
#[cfg(not(feature = "tsp"))]
pub mod tsp_dummy;


// Matrix stuff
pub mod sliding_sync;
pub mod space_service_sync;
pub mod avatar_cache;
pub mod room_preview_cache;
pub mod media_cache;
pub mod verification;

pub mod utils;
pub mod temp_storage;
pub mod location;

pub const APP_QUALIFIER: &str = "ai";
pub const APP_ORGANIZATION: &str = "hepta";
pub const APP_NAME: &str = "hepta-native";

pub fn project_dir() -> &'static ProjectDirs {
    static ROBRIX_PROJECT_DIRS: OnceLock<ProjectDirs> = OnceLock::new();

    ROBRIX_PROJECT_DIRS.get_or_init(|| {
        ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
            .expect("Failed to obtain Hepta Native project directory")
    })
}

pub fn app_data_dir() -> &'static Path {
    project_dir().data_dir()
}

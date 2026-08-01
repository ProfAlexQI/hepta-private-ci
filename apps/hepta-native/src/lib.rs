#![recursion_limit = "256"]

use std::{path::{Path, PathBuf}, sync::OnceLock};

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

/// Side-effect-free contract boundary for optional Hepta runtime integration.
#[cfg(feature = "hepta-bridge")]
pub mod hepta_bridge;

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
pub mod image_utils;

pub const APP_QUALIFIER: &str = "ai";
pub const APP_ORGANIZATION: &str = "hepta";
// Keep the historical ProjectDirs component stable across the upstream-first
// migration so existing encrypted Matrix state is neither orphaned nor paired
// with a newly-written credential generation after rollback.
pub const APP_NAME: &str = "hepta-native";

pub fn project_dir() -> &'static ProjectDirs {
    static HEPTA_PROJECT_DIRS: OnceLock<ProjectDirs> = OnceLock::new();

    HEPTA_PROJECT_DIRS.get_or_init(|| {
        ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
            .expect("Failed to obtain Hepta project directory")
    })
}

pub fn app_data_dir() -> &'static Path {
    static HEPTA_APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

    HEPTA_APP_DATA_DIR
        .get_or_init(|| {
            // Test and explicit diagnostics builds may isolate local state.
            // Product builds ignore this environment variable so an injected
            // process environment cannot silently redirect credential metadata.
            #[cfg(any(test, feature = "developer-diagnostics"))]
            if let Some(path) = std::env::var_os("HEPTA_NATIVE_APP_DATA_DIR") {
                return PathBuf::from(path);
            }
            project_dir().data_dir().to_path_buf()
        })
        .as_path()
}

pub fn cache_dir() -> &'static Path {
    project_dir().cache_dir()
}

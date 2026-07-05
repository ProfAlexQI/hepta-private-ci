//! The `LocationPreview` is a small view that shows the current location
//! and allows the user to send their location to a room.
//!
//! This view is not visible by default, only when the user requests it
//! by clicking on the location button in the message input bar.
//! The `RoomScreen` widget then shows this view above the message input bar.

use std::time::SystemTime;

use makepad_widgets::*;
use robius_location::Coordinates;

use crate::location::{
    get_latest_location, request_location_update, LocationAction, LocationRequest, LocationUpdate,
};

pub const LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE: &str = "LocationPreview now exposes explicit local Start and Stop device-update controls while preserving the existing one-shot location send path. Opening the preview requests LocationRequest::UpdateOnce; Start Device Updates submits only LocationRequest::StartUpdates, Stop Device Updates and Cancel submit only LocationRequest::StopUpdates when local continuous updates are active. These controls do not create a live-location Matrix event, do not submit SendMessage, and do not emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_LABEL: &str =
    "Live location: device updates off; one-time send remains guarded.";
pub const LIVE_LOCATION_CONTINUOUS_UPDATES_ACTIVE_LABEL: &str =
    "Live location: device updates on locally; no Matrix live event.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.LocationPreview = set_type_default() do #(LocationPreview::register_widget(vm)) {
        ..mod.widgets.RoundedView

        visible: false
        width: Fill
        height: Fit
        flow: Down
        // to align this view just below the RoomInputBar's curved border
        margin: Inset{top: 1}
        padding: Inset{left: 12, top: 10, bottom: 10, right: 10}
        spacing: 8

        show_bg: true,
        draw_bg +: {
            color: (COLOR_BG_PREVIEW)
            border_radius: 5.0
            border_size: 2.0
        }

        Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: (MESSAGE_TEXT_COLOR),
                text_style: MESSAGE_TEXT_STYLE { font_size: 10.0 },
            }
            text: "Send your location to this room?"
        }

        location_label := Label {
            width: Fill,
            height: Fit,
            align: Align{x: 0.0, y: 0.5},
            padding: Inset{left: 10, bottom: 7}
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: (MESSAGE_TEXT_COLOR),
                text_style: MESSAGE_TEXT_STYLE {},
            }
            text: "➡ Fetching current location..."
        }

        live_location_boundary_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            padding: Inset{left: 10, bottom: 2}
            draw_text +: {
                color: (MESSAGE_TEXT_COLOR),
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.0 },
            }
            text: "Live location: device updates off; one-time send remains guarded."
        }

        View {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            spacing: 12
            align: Align{x: 0.0, y: 0.5}

            start_live_location_button := RobrixIconButton {
                align: Align{x: 0.5, y: 0.5}
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: 0
                draw_icon.svg: (ICON_CHECKMARK)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                text: "Start Device Updates"
            }

            stop_live_location_button := RobrixNegativeIconButton {
                visible: false,
                align: Align{x: 0.5, y: 0.5}
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: 0
                draw_icon.svg: (ICON_FORBIDDEN)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                text: "Stop Device Updates"
            }
        }

        View {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            spacing: 15
            align: Align{x: 0.0, y: 0.5}

            cancel_location_button := RobrixNegativeIconButton {
                align: Align{x: 0.5, y: 0.5}
                padding: 15,
                margin: 0
                draw_icon.svg: (ICON_FORBIDDEN)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1, top: -1} }
                text: "Cancel"
            }

            send_location_button := RobrixPositiveIconButton {
                // disabled by default; will be enabled upon receiving valid location update.
                enabled: false,
                align: Align{x: 0.5, y: 0.5}
                padding: 15,
                margin: 0
                draw_icon.svg: (ICON_SEND)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                text: "Yes"
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
struct LocationPreview {
    #[deref]
    view: View,
    #[rust]
    coords: Option<Result<Coordinates, robius_location::Error>>,
    #[rust]
    timestamp: Option<SystemTime>,
    #[rust]
    continuous_updates_active: bool,
}

impl Widget for LocationPreview {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let mut needs_redraw = false;
        if let Event::Actions(actions) = event {
            for action in actions {
                match action.downcast_ref() {
                    Some(LocationAction::Update(LocationUpdate { coordinates, time })) => {
                        self.coords = Some(Ok(*coordinates));
                        self.timestamp = *time;
                        self.button(cx, ids!(send_location_button))
                            .set_enabled(cx, true);
                        needs_redraw = true;
                    }
                    Some(LocationAction::Error(e)) => {
                        self.coords = Some(Err(*e));
                        self.timestamp = None;
                        self.button(cx, ids!(send_location_button))
                            .set_enabled(cx, false);
                        needs_redraw = true;
                    }
                    _ => {}
                }
            }

            // NOTE: the send location button click event is handled
            //       in the RoomScreen handle_event function.

            // Handle the cancel location button being clicked.
            if self
                .button(cx, ids!(cancel_location_button))
                .clicked(actions)
            {
                self.clear();
                needs_redraw = true;
            }

            if self
                .button(cx, ids!(start_live_location_button))
                .clicked(actions)
            {
                request_location_update(LocationRequest::StartUpdates);
                self.continuous_updates_active = true;
                needs_redraw = true;
            }

            if self
                .button(cx, ids!(stop_live_location_button))
                .clicked(actions)
            {
                self.stop_continuous_updates();
                needs_redraw = true;
            }
        }

        if needs_redraw {
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let text = match self.coords {
            Some(Ok(c)) => {
                format!("➡ Current location: {:.6}, {:.6}", c.latitude, c.longitude)
            }
            Some(Err(e)) => format!("➡ Error getting location: {e:?}"),
            None => String::from("➡ Current location is not yet available."),
        };
        self.label(cx, ids!(location_label)).set_text(cx, &text);
        let live_location_label = if self.continuous_updates_active {
            LIVE_LOCATION_CONTINUOUS_UPDATES_ACTIVE_LABEL
        } else {
            LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_LABEL
        };
        self.label(cx, ids!(live_location_boundary_label))
            .set_text(cx, live_location_label);
        self.button(cx, ids!(start_live_location_button))
            .set_visible(cx, !self.continuous_updates_active);
        self.button(cx, ids!(stop_live_location_button))
            .set_visible(cx, self.continuous_updates_active);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl LocationPreview {
    fn show(&mut self) {
        request_location_update(LocationRequest::UpdateOnce);
        if let Some(loc) = get_latest_location() {
            self.coords = Some(Ok(loc.coordinates));
            self.timestamp = loc.time;
        }
        self.visible = true;
    }

    fn clear(&mut self) {
        self.stop_continuous_updates();
        self.coords = None;
        self.timestamp = None;
        self.visible = false;
    }

    fn stop_continuous_updates(&mut self) {
        if self.continuous_updates_active {
            request_location_update(LocationRequest::StopUpdates);
            self.continuous_updates_active = false;
        }
    }

    pub fn get_current_data(&self) -> Option<(Coordinates, Option<SystemTime>)> {
        self.coords
            .as_ref()
            .and_then(|res| res.ok())
            .map(|c| (c, self.timestamp))
    }
}

impl LocationPreviewRef {
    pub fn show(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show();
        }
    }

    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
        }
    }

    pub fn get_current_data(&self) -> Option<(Coordinates, Option<SystemTime>)> {
        self.borrow().and_then(|inner| inner.get_current_data())
    }
}

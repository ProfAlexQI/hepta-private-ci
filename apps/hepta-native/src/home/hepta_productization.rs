use makepad_widgets::*;

use crate::hepta_productization::{sample_productization_snapshot, HeptaProductizationItem};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaProductizationRow = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 3.0,
        padding: Inset{top: 8.0, bottom: 8.0, left: 9.0, right: 9.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF14
            border_color: #xFFFFFF24
            border_size: 1.0
            border_radius: 6.0
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 11.5 }
            }
            text: "Product lane"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xDDE7FF,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaProductizationPane = set_type_default() do #(HeptaProductizationPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 7.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x101B31
            border_color: #x5F7EDD
            border_size: 1.0
            border_radius: 8.0
        }

        title := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 13.0 }
            }
            text: "Hepta Native productization"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Matrix-heart baseline absorbed; productization tracks branding, native capability parity gates, and release evidence."
        }

        matrix_heart := mod.widgets.HeptaProductizationRow {}
        cockpit := mod.widgets.HeptaProductizationRow {}
        branding := mod.widgets.HeptaProductizationRow {}
        native_runtime_parity := mod.widgets.HeptaProductizationRow {}
        mobile_release := mod.widgets.HeptaProductizationRow {}
        release_candidate := mod.widgets.HeptaProductizationRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaProductizationPane {
    #[deref]
    view: View,
}

impl Widget for HeptaProductizationPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaProductizationPane {
    fn populate(&mut self, cx: &mut Cx) {
        let snapshot = sample_productization_snapshot();
        self.view
            .label(cx, ids!(title))
            .set_text(cx, snapshot.title);
        self.view.label(cx, ids!(subtitle)).set_text(
            cx,
            &format!(
                "{} · {} complete / {} gated · package {}",
                snapshot.summary,
                snapshot.completed_count(),
                snapshot.gated_count(),
                snapshot.android_package,
            ),
        );
        self.set_row(cx, ids!(matrix_heart), snapshot.item("matrix_heart"));
        self.set_row(cx, ids!(cockpit), snapshot.item("hepta_cockpit"));
        self.set_row(cx, ids!(branding), snapshot.item("branding_metadata"));
        self.set_row(
            cx,
            ids!(native_runtime_parity),
            snapshot.item("native_runtime_parity"),
        );
        self.set_row(cx, ids!(mobile_release), snapshot.item("mobile_release"));
        self.set_row(
            cx,
            ids!(release_candidate),
            snapshot.item("release_candidate"),
        );
    }

    fn set_row(&mut self, cx: &mut Cx, row_id: &[LiveId], item: Option<&HeptaProductizationItem>) {
        let row = self.view.widget(cx, row_id);
        if let Some(item) = item {
            row.label(cx, ids!(title)).set_text(
                cx,
                &format!(
                    "{} · {}{}",
                    item.status.label(),
                    item.label,
                    if item.blocking { " · blocker" } else { "" },
                ),
            );
            row.label(cx, ids!(body)).set_text(cx, item.detail);
        } else {
            row.label(cx, ids!(title)).set_text(cx, "unknown lane");
            row.label(cx, ids!(body))
                .set_text(cx, "No productization status available.");
        }
    }
}

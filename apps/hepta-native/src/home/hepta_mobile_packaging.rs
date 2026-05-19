use makepad_widgets::*;

use crate::hepta_mobile_packaging::{sample_mobile_packaging_status, HeptaPackagingGate};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaMobilePackagingRow = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 3.0,
        padding: Inset{top: 7.0, bottom: 7.0, left: 9.0, right: 9.0},
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
                text_style: theme.font_bold { font_size: 11.0 }
            }
            text: "Gate"
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

    mod.widgets.HeptaMobilePackagingPane = set_type_default() do #(HeptaMobilePackagingPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 7.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x102032
            border_color: #x4DBB9E
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
            text: "Mobile packaging"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Phase 6 gates are visible in-client but never trigger installs, adb, simulator, or signing actions."
        }

        summary := mod.widgets.HeptaMobilePackagingRow {}
        gate0 := mod.widgets.HeptaMobilePackagingRow {}
        gate1 := mod.widgets.HeptaMobilePackagingRow {}
        gate2 := mod.widgets.HeptaMobilePackagingRow {}
        gate3 := mod.widgets.HeptaMobilePackagingRow {}
        gate4 := mod.widgets.HeptaMobilePackagingRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaMobilePackagingPane {
    #[deref]
    view: View,
}

impl Widget for HeptaMobilePackagingPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaMobilePackagingPane {
    fn populate(&mut self, cx: &mut Cx) {
        let snapshot = sample_mobile_packaging_status();
        self.set_row(
            cx,
            self.view.widget(cx, ids!(summary)),
            snapshot.title,
            format!("{} · {}", snapshot.summary_line(), snapshot.subtitle),
        );
        self.populate_gate(cx, ids!(gate0), snapshot.gates.get(0));
        self.populate_gate(cx, ids!(gate1), snapshot.gates.get(1));
        self.populate_gate(cx, ids!(gate2), snapshot.gates.get(2));
        self.populate_gate(cx, ids!(gate3), snapshot.gates.get(3));
        self.populate_gate(cx, ids!(gate4), snapshot.gates.get(4));
    }

    fn populate_gate(&mut self, cx: &mut Cx, row_id: &[LiveId], gate: Option<&HeptaPackagingGate>) {
        let row = self.view.widget(cx, row_id);
        let Some(gate) = gate else {
            row.set_visible(cx, false);
            return;
        };
        row.set_visible(cx, true);
        self.set_row(
            cx,
            row,
            &format!("{} · {}", gate.status.label(), gate.label),
            gate.operator_line(),
        );
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

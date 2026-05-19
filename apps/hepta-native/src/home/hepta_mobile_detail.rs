use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaMobileDetailRow = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4.0,
        padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF18
            border_color: #xFFFFFF24
            border_size: 1.0
            border_radius: 6.0
        }

        title := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 12.0 }
            }
            text: "Detail"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xE7ECFF,
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaMobileDetailPane = set_type_default() do #(HeptaMobileDetailPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 8.0,
        padding: Inset{top: 14.0, bottom: 14.0, left: 14.0, right: 14.0},
        show_bg: true,
        draw_bg +: {
            color: #x16213C
            border_color: #x5877D6
            border_size: 1.0
            border_radius: 8.0
        }

        title := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 15.0 }
            }
            text: "Mobile drill-down detail"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xDDE7FF,
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: "Single-column cockpit summary for the same workspace event substrate. Mobile mutations remain draft-first."
        }

        runtime_status := mod.widgets.HeptaRuntimeStatusPane {}

        productization := mod.widgets.HeptaProductizationPane {}

        timeline := mod.widgets.HeptaMobileDetailRow {}

        composer := mod.widgets.HeptaMobileDetailRow {}

        confirmation := mod.widgets.HeptaMobileDetailRow {}

        action_outbox := mod.widgets.HeptaActionOutboxPane {}

        action_detail := mod.widgets.HeptaActionDetailPane {}

        command_templates := mod.widgets.HeptaCommandTemplatesPane {}

        context_snapshot := mod.widgets.HeptaContextSnapshotPane {}

        mobile_packaging := mod.widgets.HeptaMobilePackagingPane {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaMobileDetailPane {
    #[deref]
    view: View,
}

impl Widget for HeptaMobileDetailPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaMobileDetailPane {
    fn populate(&mut self, cx: &mut Cx) {
        self.set_row(
            cx,
            self.view.widget(cx, ids!(timeline)),
            "Timeline",
            "Runtime, task, tool, approval, agent, and memory cards share the desktop renderer contract, with payload inspection available from each Hepta card.",
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(composer)),
            "Composer",
            "/task /agent /tool /approve /reject /status now show an inline dry-run preview in the composer and suppress Matrix typing notices for reserved Hepta commands.",
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(confirmation)),
            "Confirmation",
            "Approval buttons now open an exact local payload preview through the shared confirmation modal; accepting only records a local preview, never a live mutation.",
        );
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: &str) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, body);
    }
}

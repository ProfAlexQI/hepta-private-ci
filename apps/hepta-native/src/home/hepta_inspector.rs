use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaInspectorSection = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF16
            border_color: #xFFFFFF26
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
            text: "Section"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xDDE7FF,
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaInspectorPane = set_type_default() do #(HeptaInspectorPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 10.0,
        padding: Inset{top: 14.0, bottom: 14.0, left: 12.0, right: 12.0},
        show_bg: true,
        draw_bg +: {
            color: #x111A30
            border_color: #x344D8E
            border_size: 1.0
            border_radius: 0.0
        }

        inspector_title := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 15.0 }
            }
            text: "Inspector / Control"
        }

        inspector_subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: "Read-only runtime context for the selected Hepta workspace. Mutations stay gated until explicit confirmation wiring is enabled."
        }

        runtime_status := mod.widgets.HeptaRuntimeStatusPane {}

        productization := mod.widgets.HeptaProductizationPane {}

        runtime := mod.widgets.HeptaInspectorSection {}

        tasks := mod.widgets.HeptaInspectorSection {}

        approvals := mod.widgets.HeptaInspectorSection {}

        action_outbox := mod.widgets.HeptaActionOutboxPane {}

        action_detail := mod.widgets.HeptaActionDetailPane {}

        command_templates := mod.widgets.HeptaCommandTemplatesPane {}

        context_snapshot := mod.widgets.HeptaContextSnapshotPane {}

        mobile_packaging := mod.widgets.HeptaMobilePackagingPane {}

        context := mod.widgets.HeptaInspectorSection {}

        mobile := mod.widgets.HeptaInspectorSection {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaInspectorPane {
    #[deref]
    view: View,
}

impl Widget for HeptaInspectorPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaInspectorPane {
    fn populate(&mut self, cx: &mut Cx) {
        self.set_section(
            cx,
            self.view.widget(cx, ids!(runtime)),
            "Runtime status",
            "Fixture/native parity planner active · Matrix-heart events render as m.hepta.* cards · payload inspection available · OpenClaw Gateway dependency disabled",
        );
        self.set_section(
            cx,
            self.view.widget(cx, ids!(tasks)),
            "Tasks",
            "Draft task plans from /task are previewed inline in the composer and locally staged before any task-registry mutation is allowed.",
        );
        self.set_section(
            cx,
            self.view.widget(cx, ids!(approvals)),
            "Approvals",
            "Approval buttons and /approve or /reject commands now surface exact local payload previews; confirmation records preview evidence only.",
        );
        self.set_section(
            cx,
            self.view.widget(cx, ids!(context)),
            "Context chips",
            "Composer parser recognizes @agents, #tasks, session:, memory:, and artifact: chips for dry-run planning.",
        );
        self.set_section(
            cx,
            self.view.widget(cx, ids!(mobile)),
            "Mobile policy",
            "Mobile remains read-only/draft-first until large confirmation screens and keyboard-safe mutation UX are proven.",
        );
    }

    fn set_section(&mut self, cx: &mut Cx, section: WidgetRef, title: &str, body: &str) {
        section.label(cx, ids!(title)).set_text(cx, title);
        section.label(cx, ids!(body)).set_text(cx, body);
    }
}

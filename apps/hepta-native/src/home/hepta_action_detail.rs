use makepad_widgets::*;

use crate::hepta_action_queue::{sample_action_queue_items, selected_action_detail, HeptaActionDetail};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaActionDetailRow = RoundedView {
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
            text: "Detail"
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

    mod.widgets.HeptaActionDetailPane = set_type_default() do #(HeptaActionDetailPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 7.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x111A2D
            border_color: #x6F86D8
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
            text: "Action drill-down"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Selected staged action with exact local payload preview, confirmation copy, and execution guard."
        }

        selected := mod.widgets.HeptaActionDetailRow {}
        target := mod.widgets.HeptaActionDetailRow {}
        payload := mod.widgets.HeptaActionDetailRow {}
        confirmation := mod.widgets.HeptaActionDetailRow {}
        guard := mod.widgets.HeptaActionDetailRow {}
        readback := mod.widgets.HeptaActionDetailRow {}
        evidence := mod.widgets.HeptaActionDetailRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaActionDetailPane {
    #[deref]
    view: View,
}

impl Widget for HeptaActionDetailPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaActionDetailPane {
    fn populate(&mut self, cx: &mut Cx) {
        let detail = selected_action_detail(&sample_action_queue_items());
        let Some(detail) = detail else {
            self.set_row(
                cx,
                self.view.widget(cx, ids!(selected)),
                "Selected action",
                "No staged Hepta action is available in the local preview queue.".to_string(),
            );
            return;
        };

        self.populate_detail(cx, &detail);
    }

    fn populate_detail(&mut self, cx: &mut Cx, detail: &HeptaActionDetail) {
        self.set_row(
            cx,
            self.view.widget(cx, ids!(selected)),
            "Selected action",
            detail.operator_line(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(payload)),
            "Exact payload preview",
            detail.payload_preview.clone(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(target)),
            "Target display",
            detail.target_display.clone(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(confirmation)),
            "Confirmation copy",
            detail.confirmation_summary.clone(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(guard)),
            "Execution guard",
            detail.execution_guard.clone(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(readback)),
            "Result readback",
            detail.result_readback.clone(),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(evidence)),
            "Redacted evidence",
            format!(
                "{} · {}",
                detail.redacted_evidence, detail.payload_inspection_badge
            ),
        );
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

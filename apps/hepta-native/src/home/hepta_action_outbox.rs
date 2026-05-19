use makepad_widgets::*;

use crate::hepta_action_queue::{
    inspect_action_outbox, sample_action_queue_items, summarize_action_queue, HeptaActionQueueItem,
    HeptaActionQueueStage,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaActionOutboxRow = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 3.0,
        padding: Inset{top: 8.0, bottom: 8.0, left: 9.0, right: 9.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF14
            border_color: #xFFFFFF22
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
            text: "Queue item"
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

    mod.widgets.HeptaActionOutboxPane = set_type_default() do #(HeptaActionOutboxPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 7.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x0F1830
            border_color: #x4C67B2
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
            text: "Action outbox"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Side-effect-free staging lane for composer drafts, approval previews, and blocked mutation classes."
        }

        staged := mod.widgets.HeptaActionOutboxRow {}
        confirmation := mod.widgets.HeptaActionOutboxRow {}
        blocked := mod.widgets.HeptaActionOutboxRow {}
        evidence := mod.widgets.HeptaActionOutboxRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaActionOutboxPane {
    #[deref]
    view: View,
}

impl Widget for HeptaActionOutboxPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaActionOutboxPane {
    fn populate(&mut self, cx: &mut Cx) {
        let items = sample_action_queue_items();
        let summary = summarize_action_queue(&items);
        let local = first_with_stage(&items, HeptaActionQueueStage::LocalPreview);
        let confirmation = first_with_stage(&items, HeptaActionQueueStage::AwaitingConfirmation);
        let blocked = first_with_stage(&items, HeptaActionQueueStage::PolicyBlocked);
        let inspections = inspect_action_outbox(&items);
        let first_badge = inspections
            .first()
            .map(|inspection| inspection.badge_line())
            .unwrap_or_else(|| "payload badge unavailable".to_string());

        self.set_row(
            cx,
            self.view.widget(cx, ids!(staged)),
            &format!("1 · Staged preview · {}", summary.local_preview),
            local
                .map(action_item_brief)
                .unwrap_or_else(|| "No read-only preview queued in the sample lane.".to_string()),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(confirmation)),
            &format!("2 · Exact confirmation · {}", summary.awaiting_confirmation),
            confirmation.map(action_item_brief).unwrap_or_else(|| {
                "No confirmation-required draft queued in the sample lane.".to_string()
            }),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(blocked)),
            &format!("3 · Policy blocked · {}", summary.policy_blocked),
            blocked.map(action_item_brief).unwrap_or_else(|| {
                "No policy-blocked live mutation queued in the sample lane.".to_string()
            }),
        );
        self.set_row(
            cx,
            self.view.widget(cx, ids!(evidence)),
            &format!("4 · Evidence · {} pending", summary.pending_total()),
            format!(
                "All sample queue items keep external_mutation_enabled=false and carry bridge_policy + exact_payload for desktop/mobile inspection. First badge: {first_badge}"
            ),
        );
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

fn first_with_stage(
    items: &[HeptaActionQueueItem],
    stage: HeptaActionQueueStage,
) -> Option<&HeptaActionQueueItem> {
    items.iter().find(|item| item.stage == stage)
}

fn action_item_brief(item: &HeptaActionQueueItem) -> String {
    format!(
        "{} · {} · {} · external_mutation_enabled={}",
        item.title, item.event_kind, item.mutation_class, item.external_mutation_enabled,
    )
}

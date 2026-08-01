use makepad_widgets::*;

use crate::hepta_runtime_status::{
    sample_runtime_status_snapshot, HeptaRuntimeStatusItem, HeptaRuntimeStatusKind,
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaRuntimeStatusRow = RoundedView {
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
            text: "Runtime status"
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

    mod.widgets.HeptaRuntimeStatusPane = set_type_default() do #(HeptaRuntimeStatusPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 7.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x101B34
            border_color: #x5877D6
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
            text: "Runtime event plane"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Local read-only runtime snapshot. No OpenClaw Gateway status call is made."
        }

        summary := mod.widgets.HeptaRuntimeStatusRow {}
        fixture := mod.widgets.HeptaRuntimeStatusRow {}
        fixture_smoke := mod.widgets.HeptaRuntimeStatusRow {}
        m2 := mod.widgets.HeptaRuntimeStatusRow {}
        ledgers := mod.widgets.HeptaRuntimeStatusRow {}
        composer := mod.widgets.HeptaRuntimeStatusRow {}
        approval := mod.widgets.HeptaRuntimeStatusRow {}
        outbox := mod.widgets.HeptaRuntimeStatusRow {}
        m3 := mod.widgets.HeptaRuntimeStatusRow {}
        m4 := mod.widgets.HeptaRuntimeStatusRow {}
        mobile := mod.widgets.HeptaRuntimeStatusRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaRuntimeStatusPane {
    #[deref]
    view: View,
}

impl Widget for HeptaRuntimeStatusPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaRuntimeStatusPane {
    fn populate(&mut self, cx: &mut Cx) {
        let snapshot = sample_runtime_status_snapshot();
        self.set_row(
            cx,
            self.view.widget(cx, ids!(summary)),
            &snapshot.summary_line(),
            format!(
                "{} {}",
                snapshot.subtitle, snapshot.capability_maturity_summary
            ),
        );
        self.populate_item(
            cx,
            ids!(fixture),
            item_by_id(&snapshot.items, "matrix-heart-fixture"),
        );
        self.populate_item(
            cx,
            ids!(fixture_smoke),
            item_by_id(&snapshot.items, "current-codex-fixture-smoke"),
        );
        self.populate_item(
            cx,
            ids!(m2),
            item_by_id(&snapshot.items, "runtime-m2-coverage"),
        );
        self.populate_item(
            cx,
            ids!(ledgers),
            item_by_id(&snapshot.items, "session-task-tool-ledgers"),
        );
        self.populate_item(
            cx,
            ids!(composer),
            item_by_id(&snapshot.items, "composer-action-bridge"),
        );
        self.populate_item(
            cx,
            ids!(approval),
            item_by_id(&snapshot.items, "approval-confirmation"),
        );
        self.populate_item(
            cx,
            ids!(outbox),
            item_by_id(&snapshot.items, "action-outbox"),
        );
        self.populate_item(cx, ids!(m3), item_by_id(&snapshot.items, "m3-live-gates"));
        self.set_row(
            cx,
            self.view.widget(cx, ids!(m4)),
            "M4 product readiness",
            snapshot.m4_readiness_line(),
        );
        self.populate_item(
            cx,
            ids!(mobile),
            item_by_id(&snapshot.items, "mobile-packaging"),
        );
    }

    fn populate_item(
        &mut self,
        cx: &mut Cx,
        row_id: &[LiveId],
        item: Option<&HeptaRuntimeStatusItem>,
    ) {
        let row = self.view.widget(cx, row_id);
        let Some(item) = item else {
            row.set_visible(cx, false);
            return;
        };
        row.set_visible(cx, true);
        self.set_row(cx, row, &status_title(item), item.operator_line());
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

fn item_by_id<'a>(
    items: &'a [HeptaRuntimeStatusItem],
    id: &str,
) -> Option<&'a HeptaRuntimeStatusItem> {
    items.iter().find(|item| item.id == id)
}

fn status_title(item: &HeptaRuntimeStatusItem) -> String {
    let marker = match item.kind {
        HeptaRuntimeStatusKind::Ready => "ready",
        HeptaRuntimeStatusKind::PreviewOnly => "preview-only",
        HeptaRuntimeStatusKind::Gated => "gated",
        HeptaRuntimeStatusKind::Blocked => "blocked",
    };
    format!("{} · {marker}", item.label)
}

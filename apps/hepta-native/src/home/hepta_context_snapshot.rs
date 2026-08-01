use makepad_widgets::*;

use crate::hepta_context_snapshot::{sample_context_snapshot, HeptaContextChip, HeptaContextKind};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaContextSnapshotRow = RoundedView {
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
            text: "Context"
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

    mod.widgets.HeptaContextSnapshotPane = set_type_default() do #(HeptaContextSnapshotPane::register_widget(vm)) {
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
            text: "Context snapshot"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Read-only context chips used by staged Hepta composer actions."
        }

        summary := mod.widgets.HeptaContextSnapshotRow {}
        agent := mod.widgets.HeptaContextSnapshotRow {}
        task := mod.widgets.HeptaContextSnapshotRow {}
        session := mod.widgets.HeptaContextSnapshotRow {}
        memory := mod.widgets.HeptaContextSnapshotRow {}
        artifact := mod.widgets.HeptaContextSnapshotRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaContextSnapshotPane {
    #[deref]
    view: View,
}

impl Widget for HeptaContextSnapshotPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaContextSnapshotPane {
    fn populate(&mut self, cx: &mut Cx) {
        let snapshot = sample_context_snapshot();
        self.set_row(
            cx,
            self.view.widget(cx, ids!(summary)),
            &snapshot.summary_line(),
            snapshot.tokens_line(),
        );
        self.populate_chip(
            cx,
            ids!(agent),
            chip_by_kind(&snapshot.chips, HeptaContextKind::Agent),
        );
        self.populate_chip(
            cx,
            ids!(task),
            chip_by_kind(&snapshot.chips, HeptaContextKind::Task),
        );
        self.populate_chip(
            cx,
            ids!(session),
            chip_by_kind(&snapshot.chips, HeptaContextKind::Session),
        );
        self.populate_chip(
            cx,
            ids!(memory),
            chip_by_kind(&snapshot.chips, HeptaContextKind::Memory),
        );
        self.populate_chip(
            cx,
            ids!(artifact),
            chip_by_kind(&snapshot.chips, HeptaContextKind::Artifact),
        );
    }

    fn populate_chip(&mut self, cx: &mut Cx, row_id: &[LiveId], chip: Option<&HeptaContextChip>) {
        let row = self.view.widget(cx, row_id);
        let Some(chip) = chip else {
            row.set_visible(cx, false);
            return;
        };
        row.set_visible(cx, true);
        self.set_row(cx, row, &chip.token(), chip.operator_line());
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

fn chip_by_kind(chips: &[HeptaContextChip], kind: HeptaContextKind) -> Option<&HeptaContextChip> {
    chips.iter().find(|chip| chip.kind == kind)
}

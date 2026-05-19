use makepad_widgets::*;

use crate::hepta_command_templates::{sample_command_templates, template_summary_line};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaCommandTemplateRow = RoundedView {
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
            text: "Command"
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

    mod.widgets.HeptaCommandTemplatesPane = set_type_default() do #(HeptaCommandTemplatesPane::register_widget(vm)) {
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
            text: "Quick commands"
        }

        subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Suggested Hepta composer commands validated through the dry-run planner."
        }

        summary := mod.widgets.HeptaCommandTemplateRow {}
        template0 := mod.widgets.HeptaCommandTemplateRow {}
        template1 := mod.widgets.HeptaCommandTemplateRow {}
        template2 := mod.widgets.HeptaCommandTemplateRow {}
        template3 := mod.widgets.HeptaCommandTemplateRow {}
        template4 := mod.widgets.HeptaCommandTemplateRow {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaCommandTemplatesPane {
    #[deref]
    view: View,
}

impl Widget for HeptaCommandTemplatesPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaCommandTemplatesPane {
    fn populate(&mut self, cx: &mut Cx) {
        let templates = sample_command_templates();
        self.set_row(
            cx,
            self.view.widget(cx, ids!(summary)),
            &template_summary_line(),
            "Every template stays external_mutation_enabled=false until policy gates open."
                .to_string(),
        );
        self.populate_index(cx, ids!(template0), templates.get(0));
        self.populate_index(cx, ids!(template1), templates.get(1));
        self.populate_index(cx, ids!(template2), templates.get(2));
        self.populate_index(cx, ids!(template3), templates.get(3));
        self.populate_index(cx, ids!(template4), templates.get(4));
    }

    fn populate_index(
        &mut self,
        cx: &mut Cx,
        row_id: &[LiveId],
        template: Option<&crate::hepta_command_templates::HeptaCommandTemplate>,
    ) {
        let row = self.view.widget(cx, row_id);
        let Some(template) = template else {
            row.set_visible(cx, false);
            return;
        };
        row.set_visible(cx, true);
        self.set_row(cx, row, template.label, template.operator_line());
    }

    fn set_row(&mut self, cx: &mut Cx, row: WidgetRef, title: &str, body: String) {
        row.label(cx, ids!(title)).set_text(cx, title);
        row.label(cx, ids!(body)).set_text(cx, &body);
    }
}

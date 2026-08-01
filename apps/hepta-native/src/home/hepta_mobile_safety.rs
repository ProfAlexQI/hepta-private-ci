use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaMobileSafetyPill = RoundedView {
        width: Fit,
        height: Fit,
        padding: Inset{top: 5.0, bottom: 5.0, left: 8.0, right: 8.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF18
            border_color: #xFFFFFF28
            border_size: 1.0
            border_radius: 12.0
        }

        label := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 10.0 }
            }
            text: "safe"
        }
    }

    mod.widgets.HeptaMobileSafetyBar = set_type_default() do #(HeptaMobileSafetyBar::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 6.0,
        padding: Inset{top: 8.0, bottom: 8.0, left: 12.0, right: 12.0},
        show_bg: true,
        draw_bg +: {
            color: #x0E172B
            border_color: #x4C67B2
            border_size: 1.0
            border_radius: 0.0
        }

        header := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 6.0,
            align: Align{y: 0.5}

            title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: #xFFFFFFFF,
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Hepta mobile safe-action mode"
            }

            mode := mod.widgets.HeptaMobileSafetyPill {}
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xDDE7FF,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Workspace timeline keeps the Matrix-heart substrate; Hepta actions are previewed locally, exact payloads are inspectable, and live mutation classes remain disabled on mobile."
        }

        pills := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 6.0,

            inspect := mod.widgets.HeptaMobileSafetyPill {}
            confirm := mod.widgets.HeptaMobileSafetyPill {}
            blocked := mod.widgets.HeptaMobileSafetyPill {}
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaMobileSafetyBar {
    #[deref]
    view: View,
}

impl Widget for HeptaMobileSafetyBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaMobileSafetyBar {
    fn populate(&mut self, cx: &mut Cx) {
        self.set_pill(cx, self.view.widget(cx, ids!(mode)), "draft-first");
        self.set_pill(cx, self.view.widget(cx, ids!(inspect)), "inspect payload");
        self.set_pill(cx, self.view.widget(cx, ids!(confirm)), "confirm preview");
        self.set_pill(cx, self.view.widget(cx, ids!(blocked)), "live blocked");
    }

    fn set_pill(&mut self, cx: &mut Cx, pill: WidgetRef, text: &str) {
        pill.label(cx, ids!(label)).set_text(cx, text);
    }
}

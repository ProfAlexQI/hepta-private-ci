use makepad_widgets::*;

use crate::{
    hepta_event::{card_text_for_event, HeptaEventEnvelope},
    hepta_fixture::{sample_matrix_timeline_events, HeptaFixtureMatrixEvent},
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaFixtureMiniCard = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 12.0, right: 12.0},
        show_bg: true,
        draw_bg +: {
            color: #xFFFFFF24
            border_color: #xFFFFFF30
            border_size: 1.0
            border_radius: 6.0
        }

        header := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            align: Align{y: 0.5}

            eyebrow := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: #xFFFFFFFF,
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Hepta event"
            }
            status := Label {
                width: Fit,
                height: Fit,
                draw_text +: {
                    color: #xDDE7FF,
                    text_style: theme.font_bold { font_size: 10.0 }
                }
                text: "running"
            }
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 13.0 }
            }
            text: "Runtime event"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xE7ECFF,
                text_style: theme.font_regular { font_size: 11.5 }
            }
            text: ""
        }

        meta := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xBFC9E8,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaFixtureCockpit = set_type_default() do #(HeptaFixtureCockpit::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 9.0,
        padding: Inset{top: 14.0, bottom: 14.0, left: 14.0, right: 14.0},
        show_bg: true,
        draw_bg +: {
            color: #x1A2440
            border_color: #x526EBA
            border_size: 1.0
            border_radius: 8.0
        }

        cockpit_title := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #xFFFFFFFF,
                text_style: theme.font_bold { font_size: 16.0 }
            }
            text: "Local Hepta Runtime Cockpit fixture"
        }

        cockpit_subtitle := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true},
            draw_text +: {
                color: #xDDE7FF,
                text_style: theme.font_regular { font_size: 11.5 }
            }
            text: "Renders local m.hepta.* collaboration events without requiring a homeserver login."
        }

        event0 := mod.widgets.HeptaFixtureMiniCard {}
        event1 := mod.widgets.HeptaFixtureMiniCard {}
        event2 := mod.widgets.HeptaFixtureMiniCard {}
        event3 := mod.widgets.HeptaFixtureMiniCard {}
        event4 := mod.widgets.HeptaFixtureMiniCard {}
        event5 := mod.widgets.HeptaFixtureMiniCard {}
        event6 := mod.widgets.HeptaFixtureMiniCard {}
        event7 := mod.widgets.HeptaFixtureMiniCard {}
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaFixtureCockpit {
    #[deref]
    view: View,
}

impl Widget for HeptaFixtureCockpit {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.populate(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaFixtureCockpit {
    fn populate(&mut self, cx: &mut Cx) {
        let events = sample_matrix_timeline_events();
        populate_fixture_card(cx, self.view.widget(cx, ids!(event0)), events.get(0));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event1)), events.get(1));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event2)), events.get(2));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event3)), events.get(3));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event4)), events.get(4));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event5)), events.get(5));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event6)), events.get(6));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event7)), events.get(7));
    }
}

fn populate_fixture_card(cx: &mut Cx, card: WidgetRef, event: Option<&HeptaFixtureMatrixEvent>) {
    let Some(event) = event else {
        card.set_visible(cx, false);
        return;
    };
    card.set_visible(cx, true);
    let envelope = HeptaEventEnvelope::from_content_value(&event.content).ok();
    let text = card_text_for_event(event.event_type, envelope.as_ref());
    card.label(cx, ids!(header.eyebrow))
        .set_text(cx, &text.eyebrow);
    card.label(cx, ids!(header.status))
        .set_text(cx, &text.status);
    card.label(cx, ids!(title)).set_text(cx, &text.title);
    card.label(cx, ids!(body)).set_text(cx, &text.body);
    card.label(cx, ids!(meta))
        .set_text(cx, &format!("{} · {}", event.sender, text.meta));
}

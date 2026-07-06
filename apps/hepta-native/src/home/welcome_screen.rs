use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.WELCOME_TEXT_COLOR = #x4

    mod.widgets.WelcomeScreen = SolidView {
        width: Fill, height: Fill
        align: Align{x: 0.0, y: 0.5}

        show_bg: true,
        draw_bg.color: (COLOR_PRIMARY)

        // make this a ScrollYView
        scroll_bars: mod.widgets.ScrollBars {
            show_scroll_x: false show_scroll_y: true
            scroll_bar_y.drag_scrolling: true
        }

        welcome_message := RoundedView {
            padding: 40.
            width: Fill, height: Fit
            flow: Down, spacing: 20

            draw_bg.color: (COLOR_PRIMARY)

            title := Label {
                text: "Welcome to Hepta Native!",
                draw_text +: {
                    color: (mod.widgets.WELCOME_TEXT_COLOR),
                    text_style: theme.font_bold {
                        font_size: 22.0
                    }
                }
            }

            // Using the HTML widget to taking advantage of embedding a link within text with proper vertical alignment
            MessageHtml {
                padding: Inset{top: 12, left: 0.}
                font_size: 14.
                font_color: (mod.widgets.WELCOME_TEXT_COLOR)
                text_style_normal: theme.font_regular { font_size: 14.0 }
                a: {
                    padding: Inset{left: 8., right: 8., top: 4., bottom: 5.},
                    // draw_text +: {
                    //     text_style: theme.font_bold {top_drop: 1.2, font_size: 11. },
                    //     color: #f,
                    //     color_pressed: #f00,
                    //     color_hover: #0f0,
                    // }
                }
                body:"<p>Our Matrix client is under active development, so you may need to use other clients to perform admin actions like creating rooms, kicking/banning users, and starting verification requests.</p>
                <p><br></p>
                <p>But don't worry, we're constantly expanding the featureset of Hepta Native!</p>
                <p><br></p>
                <p>Look for the latest announcements in our Matrix channel:</p>
                <p><b>Hepta Matrix-heart mode</b></p>
                "
            }

            runtime_cockpit := RoundedView {
                width: Fill, height: Fit
                flow: Down, spacing: 10
                padding: 18
                draw_bg.color: #xFFFFFF18

                cockpit_title := Label {
                    text: "Hepta runtime cockpit"
                    draw_text +: {
                        color: (mod.widgets.WELCOME_TEXT_COLOR)
                        text_style: theme.font_bold { font_size: 16.0 }
                    }
                }
                cockpit_hint := Label {
                    text: "Hepta cockpit fast path: runtime events are rendered as first-class collaboration cards."
                    draw_text +: {
                        color: (mod.widgets.WELCOME_TEXT_COLOR)
                        text_style: theme.font_regular { font_size: 12.0 }
                    }
                }
                desktop_dock_restore_evidence := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    text: "Desktop continuity: dock restore loads saved tabs locally, initializes only visible tabs, defers hidden tab content until tab press/drop/close, and sends no Matrix request."
                    draw_text +: {
                        color: (mod.widgets.WELCOME_TEXT_COLOR)
                        text_style: theme.font_regular { font_size: 12.0 }
                    }
                }
                event_runtime := Label {
                    text: "• m.hepta.runtime_event → runtime status / wake / gateway signals"
                    draw_text +: { color: (mod.widgets.WELCOME_TEXT_COLOR), text_style: theme.font_regular { font_size: 12.0 } }
                }
                event_tool := Label {
                    text: "• m.hepta.tool_call + m.hepta.tool_result → bounded tool evidence"
                    draw_text +: { color: (mod.widgets.WELCOME_TEXT_COLOR), text_style: theme.font_regular { font_size: 12.0 } }
                }
                event_approval := Label {
                    text: "• m.hepta.approval_request → native approval cards"
                    draw_text +: { color: (mod.widgets.WELCOME_TEXT_COLOR), text_style: theme.font_regular { font_size: 12.0 } }
                }
                event_task := Label {
                    text: "• m.hepta.task + m.hepta.agent_run → durable task and agent progress"
                    draw_text +: { color: (mod.widgets.WELCOME_TEXT_COLOR), text_style: theme.font_regular { font_size: 12.0 } }
                }
                event_memory := Label {
                    text: "• m.hepta.memory_citation → source-backed memory references"
                    draw_text +: { color: (mod.widgets.WELCOME_TEXT_COLOR), text_style: theme.font_regular { font_size: 12.0 } }
                }
            }

            hepta_fixture_cockpit := mod.widgets.HeptaFixtureCockpit {}

            hepta_mobile_detail := mod.widgets.HeptaMobileDetailPane {}
        }
    }
}

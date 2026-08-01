use makepad_widgets::*;

use crate::{
    hepta_action_queue::{
        inspect_action_outbox, sample_action_queue_items, selected_action_detail,
        summarize_action_queue, HeptaActionQueueStage,
    },
    hepta_event::{
        card_text_for_event, HeptaEventEnvelope, HeptaEventStatus, EVENT_AGENT_RUN,
        EVENT_MEMORY_CITATION, EVENT_RUNTIME_EVENT, EVENT_TOOL_RESULT,
    },
    hepta_fixture::{sample_matrix_timeline_events, HeptaFixtureMatrixEvent},
    shared::avatar::{AvatarWidgetExt, AvatarWidgetRefExt},
};

#[cfg(test)]
const HEPTA_FIXTURE_COCKPIT_VISIBLE_CARD_CAPACITY: usize = 9;

const HEPTA_TELEGRAM_AVATAR_BLUE: Vec4 = vec4(0.165, 0.671, 0.933, 1.0);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaFixtureMetricTile = RoundedView {
        width: 210,
        height: Fit,
        flow: Down,
        spacing: 3.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        value := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 17.0 }
            }
            text: "0"
        }

        caption := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "metric"
        }
    }

    mod.widgets.HeptaFixtureModeBadge = RoundedView {
        width: Fit,
        height: Fit,
        padding: Inset{top: 7.0, bottom: 7.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #x60E6B626
            border_color: #x60E6B680
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_CONTROL_RADIUS)
        }

        label := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_SUCCESS),
                text_style: theme.font_bold { font_size: 10.5 }
            }
            text: "Local preview"
        }
    }

    mod.widgets.HeptaFixtureWorkbenchLane = RoundedView {
        width: 280,
        height: Fit,
        flow: Down,
        spacing: 6.0,
        padding: Inset{top: 11.0, bottom: 11.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        label := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                text_style: theme.font_bold { font_size: 11.0 }
            }
            text: "Workbench"
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 13.0 }
            }
            text: "Operation lane"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 11.0 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaFixtureShellStateCard = RoundedView {
        width: 280,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        label := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                text_style: theme.font_bold { font_size: 10.0 }
            }
            text: "State"
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 12.5 }
            }
            text: "Shell state"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaFixtureActionDockItem = RoundedShadowView {
        width: 210,
        height: Fit,
        padding: Inset{top: 8.0, bottom: 8.0, left: 8.0, right: 8.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_CONTROL_RADIUS)
            shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
            shadow_radius: 6.0
            shadow_offset: vec2(0.0, 1.0)
        }

        label := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 10.5 }
            }
            text: "Action"
        }
    }

    mod.widgets.HeptaFixtureChromeCard = RoundedShadowView {
        width: 210,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
            shadow_radius: 8.0
            shadow_offset: vec2(0.0, 2.0)
        }

        label := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                text_style: theme.font_bold { font_size: 11.0 }
            }
            text: "Chrome"
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 13.0 }
            }
            text: "Route"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 11.0 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaFixtureEventGroupHeader = RoundedView {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 8.0,
        align: Align{y: 0.5},
        padding: Inset{top: 7.0, bottom: 7.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 11.5 }
            }
            text: "Event group"
        }

        count := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                text_style: theme.font_bold { font_size: 10.0 }
            }
            text: "0 cards"
        }
    }

    mod.widgets.HeptaFixtureMiniCard = RoundedView {
        width: 280,
        height: Fit,
        flow: Down,
        spacing: 6.0,
        padding: Inset{top: 11.0, bottom: 11.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
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
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Hepta event"
            }
            status := Label {
                width: Fit,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_SUCCESS),
                    text_style: theme.font_bold { font_size: 10.0 }
                }
                text: "running"
            }
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                text_style: theme.font_bold { font_size: 13.0 }
            }
            text: "Runtime event"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 11.5 }
            }
            text: ""
        }

        meta := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: ""
        }
    }

    mod.widgets.HeptaTelegramAvatar = Avatar {
        width: 38,
        height: 38,
    }

    mod.widgets.HeptaTelegramChatRow = RoundedView {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 9.0,
        align: Align{y: 0.5},
        padding: Inset{top: 8.0, bottom: 8.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_size: 0.0
            border_radius: (mod.widgets.HEPTA_GLASS_CONTROL_RADIUS)
        }

        avatar := mod.widgets.HeptaTelegramAvatar {}

        content := View {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0

            top := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8.0,
                align: Align{y: 0.5}

                title := Label {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    draw_text +: {
                        color: #x132332,
                        text_style: theme.font_bold { font_size: 12.0 }
                    }
                    text: "Hepta"
                }

                time := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: #x506575,
                        text_style: theme.font_regular { font_size: 9.5 }
                    }
                    text: "16:48"
                }
            }

            snippet := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: #x6F8190,
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: "Plan is ready. Review evidence before approving."
            }
        }
    }

    mod.widgets.HeptaTelegramBubble = RoundedView {
        width: 500,
        height: Fit,
        flow: Down,
        spacing: 4.0,
        padding: Inset{top: 7.0, bottom: 7.0, left: 11.0, right: 11.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        sender := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x0E87B2,
                text_style: theme.font_bold { font_size: 10.5 }
            }
            text: "Hepta"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x132332,
                text_style: theme.font_regular { font_size: 11.0 }
            }
            text: "Current task: rebuild Native as a Telegram chat surface. Evidence and screenshot gates stay attached."
        }

        meta := Label {
            width: Fill,
            height: Fit,
            draw_text +: {
                color: #x506575,
                text_style: theme.font_regular { font_size: 9.0 }
            }
            text: "16:30"
        }
    }

    mod.widgets.HeptaTelegramDatePill = RoundedView {
        width: Fit,
        height: Fit,
        padding: Inset{top: 4.0, bottom: 4.0, left: 9.0, right: 9.0},
        show_bg: true,
        draw_bg +: {
            color: #xD7F4FF90
            border_color: #x72CDE3A0
            border_size: 1.0
            border_radius: 12.0
        }

        label := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: #x506575,
                text_style: theme.font_bold { font_size: 9.5 }
            }
            text: "Today"
        }
    }

    mod.widgets.HeptaTelegramMobileBubble = RoundedView {
        width: 360,
        height: Fit,
        flow: Down,
        spacing: 4.0,
        padding: Inset{top: 8.0, bottom: 8.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        sender := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x0E87B2,
                text_style: theme.font_bold { font_size: 10.0 }
            }
            text: "Hepta"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x132332,
                text_style: theme.font_regular { font_size: 10.8 }
            }
            text: "Current task: rebuild Native as a Telegram chat surface."
        }
    }

    mod.widgets.HeptaTelegramPill = RoundedView {
        width: Fit,
        height: Fit,
        padding: Inset{top: 6.0, bottom: 6.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: #xA8E9F9C8
            border_color: #x52C5E2C8
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_CONTROL_RADIUS)
        }

        label := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: #x132332FF,
                text_style: theme.font_bold { font_size: 10.0 }
            }
            text: "Plan"
        }
    }

    mod.widgets.HeptaTelegramHeaderIconButton = RobrixNeutralIconButton {
        width: 40,
        height: 40,
        margin: 0,
        padding: Inset{top: 8.0, bottom: 8.0, left: 8.0, right: 8.0},
        spacing: 0.0,
        align: Align{x: 0.5, y: 0.5},

        draw_bg +: {
            color: #x00000000
            color_hover: #xDDF6FCE8
            color_down: #x9DE5F6C8
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: 17.0
        }
        draw_icon.color: (mod.widgets.COLOR_HEPTA_GLASS_DIM)
        draw_text +: {
            color: #x00000000
            color_hover: #x00000000
            color_down: #x00000000
            text_style: theme.font_bold { font_size: 0.1 }
        }
        icon_walk: Walk{width: 16.0, height: 16.0, margin: 0.0}
    }

    mod.widgets.HeptaTelegramComposer = RoundedView {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 9.0,
        align: Align{y: 0.5},
        padding: Inset{top: 8.0, bottom: 8.0, left: 12.0, right: 12.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: 0.0
        }

        attach := Label {
            width: Fit,
            height: Fit,
            draw_text +: {
                color: #x6F8190,
                text_style: theme.font_bold { font_size: 14.0 }
            }
            text: "+"
        }

        input := RoundedView {
            width: Fill,
            height: 44,
            padding: Inset{top: 12.0, bottom: 12.0, left: 12.0, right: 12.0},
            show_bg: true,
            draw_bg +: {
                color: #xE5F5F9F0
                border_color: #x78CADDA8
                border_size: 1.0
                border_radius: 19.0
            }

            placeholder := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: #x6F8190,
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: "Message Hepta"
            }
        }

        action := CircleView {
            width: 44,
            height: 44,
            align: Align{x: 0.5, y: 0.5},
            show_bg: true,
            draw_bg.color: #xA8E9F9D8

            label := Label {
                width: Fit,
                height: Fit,
                align: Align{x: 0.5, y: 0.5},
                draw_text +: {
                    color: #x132332FF,
                    text_style: theme.font_bold { font_size: 14.0 }
                }
                text: ">"
            }
        }
    }

    mod.widgets.HeptaTelegramInfoItem = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4.0,
        padding: Inset{top: 10.0, bottom: 10.0, left: 10.0, right: 10.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x132332,
                text_style: theme.font_bold { font_size: 11.0 }
            }
            text: "Evidence"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Right,
            draw_text +: {
                color: #x6F8190,
                text_style: theme.font_regular { font_size: 10.0 }
            }
            text: "Timeline and screenshot gates attached."
        }
    }

    mod.widgets.HeptaFixtureCockpit = set_type_default() do #(HeptaFixtureCockpit::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 0.0,
        padding: Inset{top: 0.0, bottom: 0.0, left: 0.0, right: 0.0},
        show_bg: true,
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_ENVIRONMENT)
            border_color: (mod.widgets.COLOR_HEPTA_GLASS_ENVIRONMENT)
            border_size: 0.0
            border_radius: 0.0
        }

        telegram_desktop_shell := View {
            width: Fill,
            height: Fill,
            flow: Right,
            spacing: 0.0,

            chat_list := RoundedView {
                width: 320,
                height: Fill,
                flow: Down,
                spacing: 4.0,
                padding: Inset{top: 12.0, bottom: 12.0, left: 10.0, right: 10.0},
                show_bg: true,
                draw_bg +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
                    border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                    border_size: 1.0
                    border_radius: 0.0
                }

                list_header := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10.0,
                    align: Align{y: 0.5}

                    menu := Label {
                        width: Fit,
                        height: Fit,
                        draw_text +: {
                            color: #x6F8190,
                            text_style: theme.font_bold { font_size: 15.0 }
                        }
                        text: "☰"
                    }

                    search := RoundedView {
                        width: 150,
                        height: Fit,
                        padding: Inset{top: 8.0, bottom: 8.0, left: 12.0, right: 12.0},
                        show_bg: true,
                        draw_bg +: {
                            color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                            border_size: 1.0
                            border_radius: 16.0
                        }

                        label := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: #x506575,
                                text_style: theme.font_regular { font_size: 10.5 }
                            }
                            text: "Search"
                        }
                    }

                    message_search_shortcut := RoundedView {
                        width: Fit,
                        height: Fit,
                        padding: Inset{top: 8.0, bottom: 8.0, left: 12.0, right: 12.0},
                        show_bg: true,
                        draw_bg +: {
                            color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                            border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                            border_size: 1.0
                            border_radius: 16.0
                        }

                        label := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: #x132332,
                                text_style: theme.font_bold { font_size: 10.5 }
                            }
                            text: "Messages"
                        }
                    }
                }

                chat_hepta := mod.widgets.HeptaTelegramChatRow {}
                chat_actions := mod.widgets.HeptaTelegramChatRow {}
                chat_approvals := mod.widgets.HeptaTelegramChatRow {}
                chat_evidence := mod.widgets.HeptaTelegramChatRow {}
            }

            chat_thread := RoundedView {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 0.0,
                show_bg: true,
                draw_bg +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_ENVIRONMENT)
                    border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                    border_size: 1.0
                    border_radius: 0.0
                }

                thread_header := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10.0,
                    align: Align{y: 0.5},
                    padding: Inset{top: 9.0, bottom: 9.0, left: 14.0, right: 14.0}

                    avatar := mod.widgets.HeptaTelegramAvatar {}

                    title_stack := View {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: 2.0

                        title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: #x132332,
                                text_style: theme.font_bold { font_size: 13.0 }
                            }
                            text: "Hepta"
                        }

                        status := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: #x506575,
                                text_style: theme.font_regular { font_size: 10.0 }
                            }
                            text: "ready to review"
                        }
                    }

                    search_action := mod.widgets.HeptaTelegramHeaderIconButton {
                        draw_icon.svg: (ICON_SEARCH)
                        text: ""
                    }

                    info_action := mod.widgets.HeptaTelegramHeaderIconButton {
                        draw_icon.svg: (ICON_INFO)
                        text: ""
                    }

                    mute_action := mod.widgets.HeptaTelegramHeaderIconButton {
                        draw_icon.svg: (ICON_FORBIDDEN)
                        text: ""
                    }

                    menu_action := mod.widgets.HeptaTelegramHeaderIconButton {
                        draw_icon.svg: (ICON_MENU)
                        text: ""
                    }
                }

                message_list := View {
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    spacing: 6.0,
                    align: Align{y: 1.0},
                    padding: Inset{top: 12.0, bottom: 12.0, left: 22.0, right: 22.0}

                    day_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        align: Align{x: 0.5}

                        day := mod.widgets.HeptaTelegramDatePill {}
                    }
                    message_intro := mod.widgets.HeptaTelegramBubble {}
                    message_user_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        align: Align{x: 1.0}

                        message_user := mod.widgets.HeptaTelegramBubble {}
                    }
                    message_plan := mod.widgets.HeptaTelegramBubble {}
                    inline_actions := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0

                        review := mod.widgets.HeptaTelegramPill {}
                        evidence := mod.widgets.HeptaTelegramPill {}
                        approve := mod.widgets.HeptaTelegramPill {}
                    }

                    matrix_link_preview := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0

                        matrix_link := mod.widgets.HeptaTelegramPill {}
                        alias := mod.widgets.HeptaTelegramPill {}
                        event := mod.widgets.HeptaTelegramPill {}

                        note := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: #x6F8190,
                                text_style: theme.font_regular { font_size: 10.0 }
                            }
                            text: "Room links open a compact preview before any action."
                        }
                    }
                }

                composer := mod.widgets.HeptaTelegramComposer {}
            }

            info_panel := RoundedView {
                width: 260,
                height: Fill,
                flow: Down,
                spacing: 8.0,
                padding: Inset{top: 12.0, bottom: 12.0, left: 10.0, right: 10.0},
                show_bg: true,
                draw_bg +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
                    border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                    border_size: 1.0
                    border_radius: 0.0
                }

                panel_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: #x132332,
                        text_style: theme.font_bold { font_size: 12.5 }
                    }
                    text: "Info"
                }

                safety := mod.widgets.HeptaTelegramInfoItem {}
                evidence := mod.widgets.HeptaTelegramInfoItem {}
                approvals := mod.widgets.HeptaTelegramInfoItem {}
            }
        }

        telegram_mobile_shell := RoundedView {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: 0.0,
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ENVIRONMENT)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: 0.0
            }

            mobile_chat_header := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 9.0,
                align: Align{y: 0.5},
                padding: Inset{top: 12.0, bottom: 12.0, left: 12.0, right: 12.0}

                back := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: #x0E87B2,
                        text_style: theme.font_bold { font_size: 14.0 }
                    }
                    text: "<"
                }

                avatar := mod.widgets.HeptaTelegramAvatar {}

                title_stack := View {
                    width: Fill,
                    height: Fit,
                    flow: Down,
                    spacing: 2.0

                    title := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: #x132332,
                            text_style: theme.font_bold { font_size: 12.5 }
                        }
                        text: "Hepta"
                    }

                    status := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: #x506575,
                            text_style: theme.font_regular { font_size: 9.8 }
                        }
                        text: "ready to review"
                    }
                }

                more := mod.widgets.HeptaTelegramHeaderIconButton {
                    draw_icon.svg: (ICON_MENU)
                    text: ""
                }

                search := mod.widgets.HeptaTelegramHeaderIconButton {
                    draw_icon.svg: (ICON_SEARCH)
                    text: ""
                }
            }

            mobile_message_list := View {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 8.0,
                align: Align{y: 1.0},
                padding: Inset{top: 14.0, bottom: 14.0, left: 10.0, right: 10.0}

                mobile_day_row := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    align: Align{x: 0.5}

                    day := mod.widgets.HeptaTelegramDatePill {}
                }
                mobile_intro := mod.widgets.HeptaTelegramMobileBubble {}
                mobile_user_row := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    align: Align{x: 1.0}

                    mobile_user := mod.widgets.HeptaTelegramMobileBubble {}
                }
                mobile_plan := mod.widgets.HeptaTelegramMobileBubble {}
                mobile_inline_actions := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 7.0

                    plan := mod.widgets.HeptaTelegramPill {}
                    evidence := mod.widgets.HeptaTelegramPill {}
                    approve := mod.widgets.HeptaTelegramPill {}
                }

                mobile_matrix_link_preview := View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 7.0

                    matrix_link := mod.widgets.HeptaTelegramPill {}
                    alias := mod.widgets.HeptaTelegramPill {}
                    event := mod.widgets.HeptaTelegramPill {}
                }
            }

            mobile_composer := mod.widgets.HeptaTelegramComposer {}
        }

        top_bar := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            align: Align{y: 0.5}

            title_stack := View {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 4.0

                cockpit_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: #x132332FF,
                        text_style: theme.font_bold { font_size: 17.0 }
                    }
                    text: "Hepta Native"
                }

                cockpit_subtitle := Label {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    draw_text +: {
                        color: #x26394A,
                        text_style: theme.font_regular { font_size: 11.0 }
                    }
                    text: "Ask, plan, evidence, and approval stay in one local flow."
                }
            }

            mode_badge := mod.widgets.HeptaFixtureModeBadge {}
        }

        mobile_top_app_bar := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            align: Align{y: 0.5},
            padding: Inset{top: 12.0, bottom: 12.0, left: 12.0, right: 12.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 8.0
                shadow_offset: vec2(0.0, 2.0)
            }

            mobile_title := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Hepta Native"
            }

            mobile_badge := Label {
                width: Fit,
                height: Fit,
                draw_text +: {
                    color: #x128A61,
                    text_style: theme.font_bold { font_size: 10.0 }
                }
                text: "Ready"
            }

            mobile_search := Label {
                width: Fit,
                height: Fit,
                draw_text +: {
                    color: #x0E87B2,
                    text_style: theme.font_bold { font_size: 10.0 }
                }
                text: "Ask"
            }
        }

        command_bar := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 4.0,
            padding: Inset{top: 12.0, bottom: 12.0, left: 14.0, right: 14.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_INPUT)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 10.0
                shadow_offset: vec2(0.0, 2.0)
            }

            command_title := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Ask Hepta"
            }

            command_body := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: ""
            }
        }

        command_palette_header := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 8.0
                shadow_offset: vec2(0.0, 2.0)
            }

            palette_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Ask / Plan / Evidence / Approve"
            }

            palette_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: "Start with one request, then move through plan, evidence, and approval."
            }
        }

        command_results := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            route_jump := mod.widgets.HeptaFixtureChromeCard {}
            approval_search := mod.widgets.HeptaFixtureChromeCard {}
            task_search := mod.widgets.HeptaFixtureChromeCard {}
            inspector_jump := mod.widgets.HeptaFixtureChromeCard {}
        }

        chrome_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            chrome_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Work modes"
            }

            chrome_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: "Ask, review, approve, and inspect are always one tap away."
            }
        }

        chrome_routes := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            route_home := mod.widgets.HeptaFixtureChromeCard {}
            route_actions := mod.widgets.HeptaFixtureChromeCard {}
            route_approvals := mod.widgets.HeptaFixtureChromeCard {}
            route_inspector := mod.widgets.HeptaFixtureChromeCard {}
        }

        route_shell_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            route_shell_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Ask / Plan / Evidence / Approve"
            }

            route_shell_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: "One flow keeps the request, plan, evidence, and approval in view."
            }
        }

        route_shell_pages := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            page_home := mod.widgets.HeptaFixtureChromeCard {}
            page_actions := mod.widgets.HeptaFixtureChromeCard {}
            page_approvals := mod.widgets.HeptaFixtureChromeCard {}
            page_inspector := mod.widgets.HeptaFixtureChromeCard {}
        }

        route_main_content := View {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 8.0

            route_main_header := RoundedShadowView {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 3.0,
                padding: Inset{top: 13.0, bottom: 13.0, left: 14.0, right: 14.0},
                show_bg: true,
                draw_bg +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
                    border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                    border_size: 1.0
                    border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
                    shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                    shadow_radius: 10.0
                    shadow_offset: vec2(0.0, 2.0)
                }

                route_main_title := Label {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    draw_text +: {
                        color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                        text_style: theme.font_bold { font_size: 12.0 }
                    }
                text: "Current work"
                }

                route_main_hint := Label {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    draw_text +: {
                        color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                        text_style: theme.font_regular { font_size: 10.0 }
                    }
                    text: "Review the current task, evidence, and approval state in one place."
                }
            }

            route_main_actions := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 7.0

                preview_action := mod.widgets.HeptaFixtureActionDockItem { width: Fill }
                inspect_action := mod.widgets.HeptaFixtureActionDockItem { width: Fill }
                copy_action := mod.widgets.HeptaFixtureActionDockItem { width: Fill }
                execute_action := mod.widgets.HeptaFixtureActionDockItem { width: Fill }
            }

            route_main_rows := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 7.0

                row_one := mod.widgets.HeptaFixtureChromeCard { width: Fill }
                row_two := mod.widgets.HeptaFixtureChromeCard { width: Fill }
                row_three := mod.widgets.HeptaFixtureChromeCard { width: Fill }
                row_four := mod.widgets.HeptaFixtureChromeCard { width: Fill }
            }
        }

        selected_row_detail := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 7.0,
            padding: Inset{top: 14.0, bottom: 14.0, left: 14.0, right: 14.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 10.0
                shadow_offset: vec2(0.0, 2.0)
            }

            selected_row_route := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Home · selected item"
            }

            selected_row_title := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Selected work detail"
            }

            selected_row_body := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: ""
            }

            selected_row_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: ""
            }

            selected_row_inspector := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_DIM),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: ""
            }
        }

        active_route_surface := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 6.0,
            padding: Inset{top: 11.0, bottom: 11.0, left: 12.0, right: 12.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_FLOATING_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 12.0
                shadow_offset: vec2(0.0, 3.0)
            }

            surface_route := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_FOCUS),
                    text_style: theme.font_bold { font_size: 10.0 }
                }
                text: "Home"
            }

            surface_title := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 13.0 }
                }
                text: "Active route surface"
            }

            surface_body := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: ""
            }

            surface_signal := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: ""
            }

            surface_control := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: ""
            }

            surface_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: ""
            }

            surface_empty_state := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: ""
            }
        }

        route_state_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            route_state_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Next step preview"
            }

            route_state_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Every flow shows what is selected and the next available step."
            }
        }

        route_state_cards := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            state_home := mod.widgets.HeptaFixtureChromeCard {}
            state_actions := mod.widgets.HeptaFixtureChromeCard {}
            state_approvals := mod.widgets.HeptaFixtureChromeCard {}
            state_inspector := mod.widgets.HeptaFixtureChromeCard {}
        }

        metrics := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            summary_total := mod.widgets.HeptaFixtureMetricTile {}
            summary_active := mod.widgets.HeptaFixtureMetricTile {}
            summary_waiting := mod.widgets.HeptaFixtureMetricTile {}
            bridge_status := mod.widgets.HeptaFixtureMetricTile {}
        }

        safety_bar := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 4.0,
            padding: Inset{top: 10.0, bottom: 10.0, left: 11.0, right: 11.0},
            show_bg: true,
            draw_bg +: {
                color: #xFFF7D8E8
                border_color: #xD4AE4366
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            safety_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: #x6F4F05,
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Review status"
            }

            safety_body := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: #x725111,
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: ""
            }
        }

        shell_state_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            shell_state_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Screen states"
            }

            shell_state_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Loading, empty, and error surfaces stay readable while the workspace is local."
            }
        }

        shell_states := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            loading_state := mod.widgets.HeptaFixtureShellStateCard {}
            empty_state := mod.widgets.HeptaFixtureShellStateCard {}
            error_state := mod.widgets.HeptaFixtureShellStateCard {}
        }

        workbench_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            workbench_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Review queue"
            }

            workbench_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Drafts, approval requests, and queued steps stay visible before anything can run."
            }
        }

        workbench := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            composer_lane := mod.widgets.HeptaFixtureWorkbenchLane {}
            approval_lane := mod.widgets.HeptaFixtureWorkbenchLane {}
            outbox_lane := mod.widgets.HeptaFixtureWorkbenchLane {}
        }

        event_stack_header := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            padding: Inset{top: 9.0, bottom: 9.0, left: 10.0, right: 10.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_PANEL_RADIUS)
            }

            event_stack_title := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Evidence timeline"
            }

            event_stack_hint := Label {
                width: Fill,
                height: Fit,
                flow: Right,
                draw_text +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Runtime, task, tool, approval, agent, memory, and bridge cards stay grouped for review."
            }
        }

        runtime_group := mod.widgets.HeptaFixtureEventGroupHeader {}
        runtime_cards := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            event0 := mod.widgets.HeptaFixtureMiniCard {}
            event8 := mod.widgets.HeptaFixtureMiniCard {}
        }

        action_group := mod.widgets.HeptaFixtureEventGroupHeader {}
        action_cards := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            event1 := mod.widgets.HeptaFixtureMiniCard {}
            event3 := mod.widgets.HeptaFixtureMiniCard {}
            event4 := mod.widgets.HeptaFixtureMiniCard {}
            event6 := mod.widgets.HeptaFixtureMiniCard {}
        }

        evidence_group := mod.widgets.HeptaFixtureEventGroupHeader {}
        evidence_cards := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0

            event2 := mod.widgets.HeptaFixtureMiniCard {}
            event5 := mod.widgets.HeptaFixtureMiniCard {}
            event7 := mod.widgets.HeptaFixtureMiniCard {}
        }

        mobile_action_dock := RoundedShadowView {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 7.0,
            padding: Inset{top: 8.0, bottom: 8.0, left: 8.0, right: 8.0},
            show_bg: true,
            draw_bg +: {
                color: (mod.widgets.COLOR_HEPTA_GLASS_PANEL)
                border_color: (mod.widgets.COLOR_HEPTA_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: (mod.widgets.HEPTA_GLASS_FLOATING_RADIUS)
                shadow_color: (mod.widgets.COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 12.0
                shadow_offset: vec2(0.0, 3.0)
            }

            dock_plan := mod.widgets.HeptaFixtureActionDockItem {}
            dock_approve := mod.widgets.HeptaFixtureActionDockItem {}
            dock_outbox := mod.widgets.HeptaFixtureActionDockItem {}
            dock_inspect := mod.widgets.HeptaFixtureActionDockItem {}
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HeptaFixtureCockpit {
    #[deref]
    view: View,
    #[rust(false)]
    selection_initialized: bool,
    #[rust(false)]
    desktop_product_layout_logged: bool,
    #[rust(false)]
    desktop_full_layout_logged: bool,
    #[rust(false)]
    mobile_task_first_layout_logged: bool,
    #[rust(false)]
    top_design_route_workspace_logged: bool,
    #[rust(false)]
    mobile_secondary_content_visible_logged: bool,
    #[rust(false)]
    mobile_route_content_visible_logged: bool,
    #[rust(HeptaFixtureRouteKey::Home)]
    selected_route: HeptaFixtureRouteKey,
    #[rust(HeptaFixtureSecondarySurfaceKey::None)]
    selected_secondary_surface: HeptaFixtureSecondarySurfaceKey,
    #[rust]
    selected_row_index: usize,
}

impl Widget for HeptaFixtureCockpit {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.handle_interactive_selection_event(cx, event) {
            return;
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let available_width = cx.turtle().width();
        let layout = selected_fixture_layout_for_width(available_width);
        self.log_layout_once(layout, available_width);
        self.populate(cx, layout);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HeptaFixtureCockpit {
    fn log_layout_once(&mut self, layout: HeptaFixtureLayout, available_width: f64) {
        let already_logged = match layout {
            HeptaFixtureLayout::DesktopProduct => &mut self.desktop_product_layout_logged,
            HeptaFixtureLayout::DesktopFull => &mut self.desktop_full_layout_logged,
            HeptaFixtureLayout::MobileTaskFirst => &mut self.mobile_task_first_layout_logged,
        };
        if *already_logged {
            return;
        }
        log!(
            "Hepta Native fixture cockpit layout selected: {} width={:.0}",
            layout.log_label(),
            available_width
        );
        *already_logged = true;
    }

    fn ensure_selection_initialized(&mut self) {
        if self.selection_initialized {
            return;
        }
        self.selected_route = selected_fixture_route();
        self.selected_secondary_surface = selected_fixture_secondary_surface();
        self.selected_row_index = selected_fixture_row_for(self.selected_route);
        log!(
            "Hepta Native fixture route selected: {} row={}",
            self.selected_route.label(),
            self.selected_row_index
        );
        if !self.selected_secondary_surface.is_none() {
            log!(
                "Hepta Native fixture secondary surface selected: {}",
                self.selected_secondary_surface.label()
            );
        }
        self.selection_initialized = true;
    }

    fn populate(&mut self, cx: &mut Cx, layout: HeptaFixtureLayout) {
        self.ensure_selection_initialized();
        self.apply_layout_visibility(cx, layout);
        let events = sample_matrix_timeline_events();
        let summary = summarize_fixture_events(&events);
        let selected_route = self.selected_route;
        let selected_row_index = sanitize_fixture_row_index(self.selected_row_index);
        self.populate_telegram_shell(cx);
        self.populate_summary(cx, &summary);
        self.populate_app_chrome(cx, &fixture_app_chrome_for(selected_route));
        if self.selected_secondary_surface.is_none() {
            let selected_surface = fixture_route_surface_for(selected_route);
            if selected_route == HeptaFixtureRouteKey::Home {
                self.populate_command_palette_header(
                    cx,
                    "Ask / Plan / Evidence / Approve",
                    "Start with one request, then move through plan, evidence, and approval.",
                );
                self.populate_route_shell_header(
                    cx,
                    "Ask / Plan / Evidence / Approve",
                    "One flow keeps the request, plan, evidence, and approval in view.",
                );
            } else {
                self.populate_command_palette_header(
                    cx,
                    selected_surface.title,
                    selected_surface.focus,
                );
                self.populate_route_shell_header(
                    cx,
                    selected_surface.title,
                    selected_surface.focus,
                );
            }
            self.populate_command_results(cx, &fixture_command_results());
            self.populate_route_shell(cx, &fixture_route_shell_pages());
        } else {
            let secondary_surface =
                fixture_secondary_route_surface_for(self.selected_secondary_surface);
            self.populate_command_palette_header(
                cx,
                secondary_surface.title,
                secondary_surface.focus,
            );
            let secondary_results =
                fixture_secondary_command_results_for(self.selected_secondary_surface);
            self.populate_command_results(cx, &secondary_results);
            self.populate_route_shell_header(
                cx,
                secondary_surface.route,
                secondary_surface.evidence_anchor,
            );
            let secondary_pages =
                fixture_secondary_route_shell_pages_for(self.selected_secondary_surface);
            self.populate_route_shell(cx, &secondary_pages);
        }
        self.populate_route_states(cx, &fixture_route_states_for(selected_route));
        let (active_surface, primary_panel) =
            fixture_active_surface_for(selected_route, self.selected_secondary_surface);
        self.populate_active_route_surface(cx, &active_surface, &primary_panel);
        let selected_row_detail =
            fixture_selected_row_detail_for_row(selected_route, selected_row_index);
        self.populate_route_main_content(
            cx,
            &fixture_route_main_content_for(selected_route),
            selected_row_index,
            &fixture_selected_row_action_strip_for(&selected_row_detail),
        );
        self.populate_selected_row_detail(cx, &selected_row_detail);
        self.populate_shell_states(cx, &fixture_shell_state_cards(), selected_route);
        self.populate_workbench(cx, &summarize_operation_workbench());
        self.populate_event_groups(cx, &summarize_event_groups(&events));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event0)), events.get(0));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event8)), events.get(8));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event1)), events.get(1));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event3)), events.get(3));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event4)), events.get(4));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event6)), events.get(6));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event2)), events.get(2));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event5)), events.get(5));
        populate_fixture_card(cx, self.view.widget(cx, ids!(event7)), events.get(7));
    }

    fn apply_layout_visibility(&mut self, cx: &mut Cx, layout: HeptaFixtureLayout) {
        let mobile_task_first = layout.is_mobile_task_first();
        let has_secondary_surface = !self.selected_secondary_surface.is_none();
        let selected_secondary_surface_visible =
            fixture_secondary_surface_visible_for_layout(layout, self.selected_secondary_surface);
        let selected_secondary_surface_visible =
            selected_secondary_surface_visible && has_secondary_surface;
        let mobile_secondary_surface_visible =
            mobile_task_first && selected_secondary_surface_visible;
        let mobile_route_workspace_visible = mobile_task_first
            && !has_secondary_surface
            && self.selected_route != HeptaFixtureRouteKey::Home;
        let desktop_home_workspace_visible = layout.is_secondary_surface_visible()
            && !has_secondary_surface
            && self.selected_route == HeptaFixtureRouteKey::Home;
        let desktop_route_workspace_visible = layout.is_secondary_surface_visible()
            && !has_secondary_surface
            && self.selected_route != HeptaFixtureRouteKey::Home;
        let route_workspace_visible = selected_secondary_surface_visible
            || desktop_home_workspace_visible
            || desktop_route_workspace_visible
            || mobile_route_workspace_visible;
        let generic_route_scaffold_visible =
            selected_secondary_surface_visible || desktop_home_workspace_visible;
        let telegram_product_visible = !route_workspace_visible;
        let telegram_desktop_visible = telegram_product_visible && !mobile_task_first;
        let telegram_mobile_visible = telegram_product_visible && mobile_task_first;
        let desktop_route_cards_visible = !mobile_task_first && generic_route_scaffold_visible;
        let route_shell_visible = route_workspace_visible;
        let detail_metric_stack_visible = generic_route_scaffold_visible;
        let route_detail_visible = route_workspace_visible;
        let horizontal_detail_rows_visible = route_workspace_visible && !mobile_task_first;

        if !self.top_design_route_workspace_logged {
            log!(
                "Hepta Native fixture top-design route workspace: route={} secondary_surface={} generic_scaffold_visible={} route_detail_visible={} desktop_route_workspace_visible={}",
                self.selected_route.label(),
                self.selected_secondary_surface.label(),
                generic_route_scaffold_visible,
                route_detail_visible,
                desktop_route_workspace_visible
            );
            self.top_design_route_workspace_logged = true;
        }
        if mobile_secondary_surface_visible && !self.mobile_secondary_content_visible_logged {
            log!(
                "Hepta Native fixture mobile secondary content visible: surface={} route_shell_visible={} route_detail_visible={} primary_panel_visible={} horizontal_detail_rows_visible={} desktop_card_row_hidden={} action_dock_hidden={}",
                self.selected_secondary_surface.label(),
                route_shell_visible,
                route_detail_visible,
                route_detail_visible,
                horizontal_detail_rows_visible,
                !desktop_route_cards_visible,
                !(mobile_task_first
                    && selected_secondary_surface_visible
                    && !mobile_secondary_surface_visible)
            );
            self.mobile_secondary_content_visible_logged = true;
        }
        if mobile_route_workspace_visible && !self.mobile_route_content_visible_logged {
            log!(
                "Hepta Native fixture mobile route content visible: route={} route_shell_visible={} route_detail_visible={} primary_panel_visible={} horizontal_detail_rows_visible={} desktop_card_row_hidden=true action_dock_hidden=true",
                self.selected_route.label(),
                route_shell_visible,
                route_detail_visible,
                route_detail_visible,
                horizontal_detail_rows_visible,
            );
            self.mobile_route_content_visible_logged = true;
        }

        for (widget, visible) in [
            (
                self.view.widget(cx, ids!(telegram_desktop_shell)),
                telegram_desktop_visible,
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.info_panel)),
                false,
            ),
            (
                self.view.widget(cx, ids!(telegram_mobile_shell)),
                telegram_mobile_visible,
            ),
            (
                self.view.widget(cx, ids!(top_bar)),
                route_workspace_visible && !mobile_task_first,
            ),
            (
                self.view.widget(cx, ids!(mobile_top_app_bar)),
                mobile_task_first
                    && (selected_secondary_surface_visible || mobile_route_workspace_visible),
            ),
            (
                self.view.widget(cx, ids!(command_bar)),
                generic_route_scaffold_visible && !mobile_task_first,
            ),
            (
                self.view.widget(cx, ids!(command_palette_header)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(command_results)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(chrome_header)),
                desktop_route_cards_visible,
            ),
            (
                self.view.widget(cx, ids!(chrome_routes)),
                desktop_route_cards_visible,
            ),
            (
                self.view.widget(cx, ids!(route_shell_header)),
                route_shell_visible,
            ),
            (
                self.view.widget(cx, ids!(route_shell_pages)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view
                    .widget(cx, ids!(route_main_content.route_main_actions)),
                horizontal_detail_rows_visible,
            ),
            (
                self.view
                    .widget(cx, ids!(route_main_content.route_main_rows)),
                horizontal_detail_rows_visible,
            ),
            (
                self.view.widget(cx, ids!(route_main_content)),
                route_detail_visible,
            ),
            (
                self.view.widget(cx, ids!(selected_row_detail)),
                route_detail_visible,
            ),
            (
                self.view.widget(cx, ids!(metrics)),
                horizontal_detail_rows_visible,
            ),
            (
                self.view.widget(cx, ids!(safety_bar)),
                detail_metric_stack_visible,
            ),
            (
                self.view.widget(cx, ids!(active_route_surface)),
                route_detail_visible,
            ),
            (
                self.view.widget(cx, ids!(route_state_header)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(route_state_cards)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(shell_state_header)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(shell_states)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(workbench_header)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(workbench)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(event_stack_header)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(runtime_group)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(runtime_cards)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(action_group)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(action_cards)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(evidence_group)),
                generic_route_scaffold_visible,
            ),
            (
                self.view.widget(cx, ids!(evidence_cards)),
                generic_route_scaffold_visible && !mobile_secondary_surface_visible,
            ),
            (
                self.view.widget(cx, ids!(mobile_action_dock)),
                mobile_task_first
                    && selected_secondary_surface_visible
                    && !mobile_secondary_surface_visible,
            ),
        ] {
            widget.set_visible(cx, visible);
        }
    }

    fn populate_telegram_shell(&mut self, cx: &mut Cx) {
        let mut selected_chat_row = self
            .view
            .view(cx, ids!(telegram_desktop_shell.chat_list.chat_hepta));
        script_apply_eval!(cx, selected_chat_row, {
            draw_bg +: {
                color: #xBDEFFF88,
                border_color: #xBDEFFF88,
                border_size: 0.0,
            }
        });

        let chat_rows = [
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.chat_list.chat_hepta)),
                "H",
                "Hepta",
                "16:48",
                "Plan is ready. Review evidence before approving.",
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.chat_list.chat_actions)),
                "A",
                "Actions",
                "16:31",
                "2 pending steps are ready to review.",
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.chat_list.chat_approvals)),
                "R",
                "Approvals",
                "16:09",
                "Changes stay paused until approved.",
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.chat_list.chat_evidence)),
                "E",
                "Evidence",
                "15:42",
                "Screenshots and local checks are attached.",
            ),
        ];
        for (widget, initials, title, time, snippet) in chat_rows {
            widget.avatar(cx, ids!(avatar)).show_text(
                cx,
                Some(HEPTA_TELEGRAM_AVATAR_BLUE),
                None,
                initials,
            );
            widget
                .label(cx, ids!(content.top.title))
                .set_text(cx, title);
            widget.label(cx, ids!(content.top.time)).set_text(cx, time);
            widget
                .label(cx, ids!(content.snippet))
                .set_text(cx, snippet);
        }

        self.view
            .label(
                cx,
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .thread_header
                        .title_stack
                        .title
                ),
            )
            .set_text(cx, "Hepta");
        self.view
            .label(
                cx,
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .thread_header
                        .title_stack
                        .status
                ),
            )
            .set_text(cx, "ready to review");
        self.view
            .avatar(
                cx,
                ids!(telegram_desktop_shell.chat_thread.thread_header.avatar),
            )
            .show_text(cx, Some(HEPTA_TELEGRAM_AVATAR_BLUE), None, "H");

        let desktop_messages = [
            (
                self.view.widget(
                    cx,
                    ids!(
                        telegram_desktop_shell
                            .chat_thread
                            .message_list
                            .message_intro
                    ),
                ),
                "Hepta",
                "Tell me the outcome you want. I will draft a plan, attach evidence, and wait for your approval before the next step.",
                "16:27",
            ),
            (
                self.view.widget(
                    cx,
                    ids!(
                        telegram_desktop_shell
                            .chat_thread
                            .message_list
                            .message_user_row
                            .message_user
                    ),
                ),
                "You",
                "Review the current UI task and show the next step.",
                "16:29 ✓✓",
            ),
            (
                self.view.widget(
                    cx,
                    ids!(telegram_desktop_shell.chat_thread.message_list.message_plan),
                ),
                "Hepta",
                "Current task: rebuild Native as a Telegram chat surface. Evidence and screenshot gates stay attached to this thread.",
                "16:48",
            ),
        ];
        for (widget, sender, body, meta) in desktop_messages {
            widget.label(cx, ids!(sender)).set_text(cx, sender);
            widget.label(cx, ids!(body)).set_text(cx, body);
            widget.label(cx, ids!(meta)).set_text(cx, meta);
        }
        let mut desktop_user_bubble = self.view.view(
            cx,
            ids!(
                telegram_desktop_shell
                    .chat_thread
                    .message_list
                    .message_user_row
                    .message_user
            ),
        );
        script_apply_eval!(cx, desktop_user_bubble, {
            draw_bg +: {
                color: #xBDEFFF88,
                border_color: #xBDEFFF88,
                border_size: 1.0,
            }
        });

        let desktop_actions = [
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .inline_actions
                        .review
                        .label
                ),
                "Review",
            ),
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .inline_actions
                        .evidence
                        .label
                ),
                "Evidence",
            ),
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .inline_actions
                        .approve
                        .label
                ),
                "Approve",
            ),
        ];
        for (path, label) in desktop_actions {
            self.view.label(cx, path).set_text(cx, label);
        }

        let desktop_matrix_link_actions = [
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .matrix_link_preview
                        .matrix_link
                        .label
                ),
                "Room link",
            ),
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .matrix_link_preview
                        .alias
                        .label
                ),
                "Alias",
            ),
            (
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .message_list
                        .matrix_link_preview
                        .event
                        .label
                ),
                "Event",
            ),
        ];
        for (path, label) in desktop_matrix_link_actions {
            self.view.label(cx, path).set_text(cx, label);
        }

        self.view
            .label(
                cx,
                ids!(
                    telegram_desktop_shell
                        .chat_thread
                        .composer
                        .input
                        .placeholder
                ),
            )
            .set_text(cx, "Message Hepta");
        self.view
            .label(
                cx,
                ids!(telegram_desktop_shell.chat_thread.composer.action.label),
            )
            .set_text(cx, ">");

        let info_items = [
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.info_panel.safety)),
                "Review",
                "Changes stay paused until approval.",
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.info_panel.evidence)),
                "Evidence",
                "Timeline and screenshot gates attached.",
            ),
            (
                self.view
                    .widget(cx, ids!(telegram_desktop_shell.info_panel.approvals)),
                "Approvals",
                "2 pending decisions.",
            ),
        ];
        for (widget, title, body) in info_items {
            widget.label(cx, ids!(title)).set_text(cx, title);
            widget.label(cx, ids!(body)).set_text(cx, body);
        }

        self.view
            .label(
                cx,
                ids!(telegram_mobile_shell.mobile_chat_header.title_stack.title),
            )
            .set_text(cx, "Hepta");
        self.view
            .label(
                cx,
                ids!(telegram_mobile_shell.mobile_chat_header.title_stack.status),
            )
            .set_text(cx, "ready to review");
        self.view
            .avatar(cx, ids!(telegram_mobile_shell.mobile_chat_header.avatar))
            .show_text(cx, Some(HEPTA_TELEGRAM_AVATAR_BLUE), None, "H");
        let mobile_messages = [
            (
                self.view.widget(
                    cx,
                    ids!(telegram_mobile_shell.mobile_message_list.mobile_intro),
                ),
                "Hepta",
                "Tell me the outcome. I will plan, attach evidence, and wait for approval.",
            ),
            (
                self.view.widget(
                    cx,
                    ids!(
                        telegram_mobile_shell
                            .mobile_message_list
                            .mobile_user_row
                            .mobile_user
                    ),
                ),
                "You",
                "Review the current UI task.",
            ),
            (
                self.view.widget(
                    cx,
                    ids!(telegram_mobile_shell.mobile_message_list.mobile_plan),
                ),
                "Hepta",
                "Current task: rebuild Native as a Telegram chat surface. Screenshot gates are attached.",
            ),
        ];
        for (widget, sender, body) in mobile_messages {
            widget.label(cx, ids!(sender)).set_text(cx, sender);
            widget.label(cx, ids!(body)).set_text(cx, body);
        }
        let mut mobile_user_bubble = self.view.view(
            cx,
            ids!(
                telegram_mobile_shell
                    .mobile_message_list
                    .mobile_user_row
                    .mobile_user
            ),
        );
        script_apply_eval!(cx, mobile_user_bubble, {
            draw_bg +: {
                color: #xBDEFFF88,
                border_color: #xBDEFFF88,
                border_size: 1.0,
            }
        });
        let mobile_actions = [
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_inline_actions
                        .plan
                        .label
                ),
                "Review",
            ),
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_inline_actions
                        .evidence
                        .label
                ),
                "Evidence",
            ),
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_inline_actions
                        .approve
                        .label
                ),
                "Approve",
            ),
        ];
        for (path, label) in mobile_actions {
            self.view.label(cx, path).set_text(cx, label);
        }
        let mobile_matrix_link_actions = [
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_matrix_link_preview
                        .matrix_link
                        .label
                ),
                "Room link",
            ),
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_matrix_link_preview
                        .alias
                        .label
                ),
                "Alias",
            ),
            (
                ids!(
                    telegram_mobile_shell
                        .mobile_message_list
                        .mobile_matrix_link_preview
                        .event
                        .label
                ),
                "Event",
            ),
        ];
        for (path, label) in mobile_matrix_link_actions {
            self.view.label(cx, path).set_text(cx, label);
        }
        self.view
            .label(
                cx,
                ids!(telegram_mobile_shell.mobile_composer.input.placeholder),
            )
            .set_text(cx, "Message Hepta");
        self.view
            .label(cx, ids!(telegram_mobile_shell.mobile_composer.action.label))
            .set_text(cx, ">");
    }

    fn handle_interactive_selection_event(&mut self, cx: &mut Cx, event: &Event) -> bool {
        self.ensure_selection_initialized();

        let route_targets = [
            (
                self.view.widget(cx, ids!(command_results.route_jump)),
                HeptaFixtureRouteKey::Home,
            ),
            (
                self.view.widget(cx, ids!(command_results.approval_search)),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                self.view.widget(cx, ids!(command_results.task_search)),
                HeptaFixtureRouteKey::Inspector,
            ),
            (
                self.view.widget(cx, ids!(command_results.inspector_jump)),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                self.view.widget(cx, ids!(chrome_routes.route_home)),
                HeptaFixtureRouteKey::Home,
            ),
            (
                self.view.widget(cx, ids!(chrome_routes.route_actions)),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                self.view.widget(cx, ids!(chrome_routes.route_approvals)),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                self.view.widget(cx, ids!(chrome_routes.route_inspector)),
                HeptaFixtureRouteKey::Inspector,
            ),
            (
                self.view.widget(cx, ids!(route_shell_pages.page_home)),
                HeptaFixtureRouteKey::Home,
            ),
            (
                self.view.widget(cx, ids!(route_shell_pages.page_actions)),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                self.view.widget(cx, ids!(route_shell_pages.page_approvals)),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                self.view.widget(cx, ids!(route_shell_pages.page_inspector)),
                HeptaFixtureRouteKey::Inspector,
            ),
            (
                self.view.widget(cx, ids!(route_state_cards.state_home)),
                HeptaFixtureRouteKey::Home,
            ),
            (
                self.view.widget(cx, ids!(route_state_cards.state_actions)),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                self.view
                    .widget(cx, ids!(route_state_cards.state_approvals)),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                self.view
                    .widget(cx, ids!(route_state_cards.state_inspector)),
                HeptaFixtureRouteKey::Inspector,
            ),
            (
                self.view.widget(cx, ids!(mobile_action_dock.dock_plan)),
                HeptaFixtureRouteKey::Home,
            ),
            (
                self.view.widget(cx, ids!(mobile_action_dock.dock_approve)),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                self.view.widget(cx, ids!(mobile_action_dock.dock_outbox)),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                self.view.widget(cx, ids!(mobile_action_dock.dock_inspect)),
                HeptaFixtureRouteKey::Inspector,
            ),
        ];
        for (widget, route) in route_targets {
            match fixture_selection_hit(cx, event, widget) {
                HeptaFixtureSelectionHit::Activated => {
                    self.select_fixture_route_interactively(cx, route);
                    return true;
                }
                HeptaFixtureSelectionHit::Pressed => return true,
                HeptaFixtureSelectionHit::None => {}
            }
        }

        let row_targets = [
            (
                self.view
                    .widget(cx, ids!(route_main_content.route_main_rows.row_one)),
                0,
            ),
            (
                self.view
                    .widget(cx, ids!(route_main_content.route_main_rows.row_two)),
                1,
            ),
            (
                self.view
                    .widget(cx, ids!(route_main_content.route_main_rows.row_three)),
                2,
            ),
        ];
        for (widget, row_index) in row_targets {
            match fixture_selection_hit(cx, event, widget) {
                HeptaFixtureSelectionHit::Activated => {
                    self.select_fixture_row_interactively(cx, row_index);
                    return true;
                }
                HeptaFixtureSelectionHit::Pressed => return true,
                HeptaFixtureSelectionHit::None => {}
            }
        }

        false
    }

    fn select_fixture_route_interactively(&mut self, cx: &mut Cx, route: HeptaFixtureRouteKey) {
        self.set_interactive_selection(cx, route, default_fixture_row_index_for(route));
    }

    fn select_fixture_row_interactively(&mut self, cx: &mut Cx, row_index: usize) {
        self.set_interactive_selection(cx, self.selected_route, row_index);
    }

    fn set_interactive_selection(
        &mut self,
        cx: &mut Cx,
        route: HeptaFixtureRouteKey,
        row_index: usize,
    ) {
        let row_index = sanitize_fixture_row_index(row_index);
        if self.selected_route == route && self.selected_row_index == row_index {
            return;
        }
        self.selected_route = route;
        self.selected_row_index = row_index;
        self.selection_initialized = true;
        self.redraw(cx);
        cx.widget_action(
            self.widget_uid(),
            HeptaFixtureCockpitAction::SelectionChanged {
                route: route.label(),
                selected_row: row_index,
                live_side_effects: false,
            },
        );
    }

    fn populate_summary(&mut self, cx: &mut Cx, summary: &HeptaFixtureCockpitSummary) {
        self.view
            .label(cx, ids!(metrics.summary_total.value))
            .set_text(cx, &summary.total_events.to_string());
        self.view
            .label(cx, ids!(metrics.summary_total.caption))
            .set_text(cx, "local Hepta events");
        self.view
            .label(cx, ids!(metrics.summary_active.value))
            .set_text(cx, &summary.active_events.to_string());
        self.view
            .label(cx, ids!(metrics.summary_active.caption))
            .set_text(cx, "active runtime lanes");
        self.view
            .label(cx, ids!(metrics.summary_waiting.value))
            .set_text(cx, &summary.waiting_events.to_string());
        self.view
            .label(cx, ids!(metrics.summary_waiting.caption))
            .set_text(cx, "items awaiting approval");
        self.view
            .label(cx, ids!(metrics.bridge_status.value))
            .set_text(cx, summary.bridge_badge());
        self.view
            .label(cx, ids!(metrics.bridge_status.caption))
            .set_text(cx, "local evidence context");
        self.view
            .label(cx, ids!(safety_bar.safety_body))
            .set_text(cx, &summary.safety_display_line());
    }

    fn populate_app_chrome(&mut self, cx: &mut Cx, chrome: &HeptaFixtureAppChrome) {
        self.view
            .label(cx, ids!(command_bar.command_body))
            .set_text(cx, &chrome.display_command_line());
        let route_widgets = [
            self.view.widget(cx, ids!(chrome_routes.route_home)),
            self.view.widget(cx, ids!(chrome_routes.route_actions)),
            self.view.widget(cx, ids!(chrome_routes.route_approvals)),
            self.view.widget(cx, ids!(chrome_routes.route_inspector)),
        ];
        for (widget, route) in route_widgets.into_iter().zip(chrome.desktop_routes.iter()) {
            self.set_chrome_route(cx, widget, route);
        }
    }

    fn populate_command_results(&mut self, cx: &mut Cx, results: &[HeptaFixtureCommandResult]) {
        let result_widgets = [
            self.view.widget(cx, ids!(command_results.route_jump)),
            self.view.widget(cx, ids!(command_results.approval_search)),
            self.view.widget(cx, ids!(command_results.task_search)),
            self.view.widget(cx, ids!(command_results.inspector_jump)),
        ];
        for (widget, result) in result_widgets.into_iter().zip(results.iter()) {
            self.set_command_result(cx, widget, result);
        }
    }

    fn populate_command_palette_header(&mut self, cx: &mut Cx, title: &str, hint: &str) {
        self.view
            .label(cx, ids!(command_palette_header.palette_title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(command_palette_header.palette_hint))
            .set_text(cx, hint);
    }

    fn populate_route_shell_header(&mut self, cx: &mut Cx, title: &str, hint: &str) {
        self.view
            .label(cx, ids!(route_shell_header.route_shell_title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(route_shell_header.route_shell_hint))
            .set_text(cx, hint);
    }

    fn populate_route_shell(&mut self, cx: &mut Cx, pages: &[HeptaFixtureRoutePage]) {
        let page_widgets = [
            self.view.widget(cx, ids!(route_shell_pages.page_home)),
            self.view.widget(cx, ids!(route_shell_pages.page_actions)),
            self.view.widget(cx, ids!(route_shell_pages.page_approvals)),
            self.view.widget(cx, ids!(route_shell_pages.page_inspector)),
        ];
        for (widget, page) in page_widgets.into_iter().zip(pages.iter()) {
            self.set_route_page(cx, widget, page);
        }
    }

    fn populate_route_states(&mut self, cx: &mut Cx, states: &[HeptaFixtureRouteState]) {
        let state_widgets = [
            self.view.widget(cx, ids!(route_state_cards.state_home)),
            self.view.widget(cx, ids!(route_state_cards.state_actions)),
            self.view
                .widget(cx, ids!(route_state_cards.state_approvals)),
            self.view
                .widget(cx, ids!(route_state_cards.state_inspector)),
        ];
        for (widget, state) in state_widgets.into_iter().zip(states.iter()) {
            self.set_route_state(cx, widget, state);
        }
    }

    fn set_route_state(&mut self, cx: &mut Cx, widget: WidgetRef, state: &HeptaFixtureRouteState) {
        widget.label(cx, ids!(label)).set_text(cx, state.route);
        widget
            .label(cx, ids!(title))
            .set_text(cx, state.content_anchor);
        widget
            .label(cx, ids!(body))
            .set_text(cx, &state.display_line());
    }

    fn populate_active_route_surface(
        &mut self,
        cx: &mut Cx,
        surface: &HeptaFixtureRouteSurface,
        primary_panel: &HeptaFixtureRoutePrimaryPanel,
    ) {
        self.view
            .label(cx, ids!(active_route_surface.surface_route))
            .set_text(cx, surface.route);
        self.view
            .label(cx, ids!(active_route_surface.surface_title))
            .set_text(cx, primary_panel.title);
        self.view
            .label(cx, ids!(active_route_surface.surface_body))
            .set_text(cx, surface.focus);
        self.view
            .label(cx, ids!(active_route_surface.surface_signal))
            .set_text(cx, primary_panel.signal);
        self.view
            .label(cx, ids!(active_route_surface.surface_control))
            .set_text(cx, primary_panel.operator_control);
        self.view
            .label(cx, ids!(active_route_surface.surface_evidence))
            .set_text(cx, primary_panel.evidence_stack);
        self.view
            .label(cx, ids!(active_route_surface.surface_empty_state))
            .set_text(cx, &primary_panel.display_empty_state_line());
    }

    fn populate_route_main_content(
        &mut self,
        cx: &mut Cx,
        content: &HeptaFixtureRouteMainContent,
        selected_row_index: usize,
        action_strip: &HeptaFixtureSelectedRowActionStrip,
    ) {
        self.view
            .label(
                cx,
                ids!(route_main_content.route_main_header.route_main_title),
            )
            .set_text(cx, content.title);
        self.view
            .label(
                cx,
                ids!(route_main_content.route_main_header.route_main_hint),
            )
            .set_text(
                cx,
                &content.display_hint_with_selected_row(selected_row_index),
            );
        let action_widgets = [
            self.view.widget(
                cx,
                ids!(route_main_content.route_main_actions.preview_action),
            ),
            self.view.widget(
                cx,
                ids!(route_main_content.route_main_actions.inspect_action),
            ),
            self.view
                .widget(cx, ids!(route_main_content.route_main_actions.copy_action)),
            self.view.widget(
                cx,
                ids!(route_main_content.route_main_actions.execute_action),
            ),
        ];
        let action_lines = action_strip.display_action_lines();
        for (widget, action) in action_widgets.into_iter().zip(action_lines.iter()) {
            widget.label(cx, ids!(label)).set_text(cx, action);
        }

        let row_widgets = [
            self.view
                .widget(cx, ids!(route_main_content.route_main_rows.row_one)),
            self.view
                .widget(cx, ids!(route_main_content.route_main_rows.row_two)),
            self.view
                .widget(cx, ids!(route_main_content.route_main_rows.row_three)),
            self.view
                .widget(cx, ids!(route_main_content.route_main_rows.row_four)),
        ];
        let boundary_row = HeptaFixtureRouteMainRow {
            route: content.route,
            label: "Boundary",
            title: "Approval boundary",
            detail: "review evidence before any confirmed handoff",
            live_side_effects: false,
        };
        for (index, (widget, row)) in row_widgets
            .into_iter()
            .zip(content.rows.iter().chain(std::iter::once(&boundary_row)))
            .enumerate()
        {
            self.set_route_main_row(cx, widget, row, index == selected_row_index);
        }
    }

    fn set_route_main_row(
        &mut self,
        cx: &mut Cx,
        widget: WidgetRef,
        row: &HeptaFixtureRouteMainRow,
        selected: bool,
    ) {
        widget
            .label(cx, ids!(label))
            .set_text(cx, &row.label_line(selected));
        widget.label(cx, ids!(title)).set_text(cx, row.title);
        widget
            .label(cx, ids!(body))
            .set_text(cx, row.display_line());
    }

    fn populate_selected_row_detail(
        &mut self,
        cx: &mut Cx,
        detail: &HeptaFixtureSelectedRowDetail,
    ) {
        self.view
            .label(cx, ids!(selected_row_detail.selected_row_route))
            .set_text(cx, &detail.display_route_line());
        self.view
            .label(cx, ids!(selected_row_detail.selected_row_title))
            .set_text(cx, detail.detail_title);
        self.view
            .label(cx, ids!(selected_row_detail.selected_row_body))
            .set_text(cx, detail.detail_body);
        self.view
            .label(cx, ids!(selected_row_detail.selected_row_evidence))
            .set_text(cx, &detail.display_evidence_line());
        self.view
            .label(cx, ids!(selected_row_detail.selected_row_inspector))
            .set_text(cx, &detail.display_inspector_line());
    }

    fn set_route_page(&mut self, cx: &mut Cx, widget: WidgetRef, page: &HeptaFixtureRoutePage) {
        widget.label(cx, ids!(label)).set_text(cx, page.route);
        widget.label(cx, ids!(title)).set_text(cx, page.title);
        widget
            .label(cx, ids!(body))
            .set_text(cx, &page.display_line());
    }

    fn set_command_result(
        &mut self,
        cx: &mut Cx,
        widget: WidgetRef,
        result: &HeptaFixtureCommandResult,
    ) {
        widget.label(cx, ids!(label)).set_text(cx, result.label);
        widget.label(cx, ids!(title)).set_text(cx, result.title);
        widget
            .label(cx, ids!(body))
            .set_text(cx, &result.display_line());
    }

    fn set_chrome_route(
        &mut self,
        cx: &mut Cx,
        widget: WidgetRef,
        route: &HeptaFixtureChromeRoute,
    ) {
        widget.label(cx, ids!(label)).set_text(cx, route.label);
        widget.label(cx, ids!(title)).set_text(cx, route.title);
        widget
            .label(cx, ids!(body))
            .set_text(cx, &route.display_line());
    }

    fn populate_shell_states(
        &mut self,
        cx: &mut Cx,
        states: &[HeptaFixtureShellState],
        selected_route: HeptaFixtureRouteKey,
    ) {
        let state_widgets = [
            self.view.widget(cx, ids!(shell_states.loading_state)),
            self.view.widget(cx, ids!(shell_states.empty_state)),
            self.view.widget(cx, ids!(shell_states.error_state)),
        ];
        for (widget, state) in state_widgets.into_iter().zip(states.iter()) {
            self.set_shell_state(cx, widget, state);
        }
        let dock_items = [
            (
                ids!(mobile_action_dock.dock_plan.label),
                HeptaFixtureRouteKey::Home,
            ),
            (
                ids!(mobile_action_dock.dock_approve.label),
                HeptaFixtureRouteKey::Actions,
            ),
            (
                ids!(mobile_action_dock.dock_outbox.label),
                HeptaFixtureRouteKey::Approvals,
            ),
            (
                ids!(mobile_action_dock.dock_inspect.label),
                HeptaFixtureRouteKey::Inspector,
            ),
        ];
        for (label_id, route) in dock_items {
            let label = if route == selected_route {
                format!("Selected · {}", route.label())
            } else {
                route.label().to_string()
            };
            self.view.label(cx, label_id).set_text(cx, &label);
        }
    }

    fn set_shell_state(&mut self, cx: &mut Cx, widget: WidgetRef, state: &HeptaFixtureShellState) {
        widget.label(cx, ids!(label)).set_text(cx, state.label);
        widget.label(cx, ids!(title)).set_text(cx, state.title);
        widget.label(cx, ids!(body)).set_text(cx, state.body);
    }

    fn populate_workbench(&mut self, cx: &mut Cx, summary: &HeptaFixtureOperationWorkbench) {
        self.set_lane(
            cx,
            self.view.widget(cx, ids!(workbench.composer_lane)),
            "Ask",
            &summary.composer_title,
            &summary.composer_display_body(),
        );
        self.set_lane(
            cx,
            self.view.widget(cx, ids!(workbench.approval_lane)),
            "Approve",
            &summary.approval_title,
            &summary.approval_display_body(),
        );
        self.set_lane(
            cx,
            self.view.widget(cx, ids!(workbench.outbox_lane)),
            "Review",
            &summary.outbox_title,
            &summary.outbox_display_body(),
        );
    }

    fn set_lane(&mut self, cx: &mut Cx, lane: WidgetRef, label: &str, title: &str, body: &str) {
        lane.label(cx, ids!(label)).set_text(cx, label);
        lane.label(cx, ids!(title)).set_text(cx, title);
        lane.label(cx, ids!(body)).set_text(cx, body);
    }

    fn populate_event_groups(&mut self, cx: &mut Cx, groups: &HeptaFixtureEventGroups) {
        self.set_event_group(
            cx,
            self.view.widget(cx, ids!(runtime_group)),
            "Runtime lane",
            groups.runtime,
        );
        self.set_event_group(
            cx,
            self.view.widget(cx, ids!(action_group)),
            "Action lane",
            groups.action,
        );
        self.set_event_group(
            cx,
            self.view.widget(cx, ids!(evidence_group)),
            "Evidence lane",
            groups.evidence,
        );
    }

    fn set_event_group(&mut self, cx: &mut Cx, group: WidgetRef, title: &str, count: usize) {
        group.label(cx, ids!(title)).set_text(cx, title);
        group
            .label(cx, ids!(count))
            .set_text(cx, &format!("{count} cards"));
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
enum HeptaFixtureCockpitAction {
    SelectionChanged {
        route: &'static str,
        selected_row: usize,
        live_side_effects: bool,
    },
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeptaFixtureSelectionHit {
    None,
    Pressed,
    Activated,
}

fn fixture_selection_hit(
    cx: &mut Cx,
    event: &Event,
    widget: WidgetRef,
) -> HeptaFixtureSelectionHit {
    let area = widget.area();
    match event.hits(cx, area) {
        Hit::FingerDown(..) => {
            cx.set_key_focus(area);
            HeptaFixtureSelectionHit::Pressed
        }
        Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
            HeptaFixtureSelectionHit::Activated
        }
        _ => HeptaFixtureSelectionHit::None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureCockpitSummary {
    total_events: usize,
    active_events: usize,
    waiting_events: usize,
    completed_events: usize,
    current_bridge_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureShellState {
    label: &'static str,
    title: &'static str,
    body: &'static str,
    live_side_effects: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureChromeRoute {
    label: &'static str,
    title: &'static str,
    detail: &'static str,
    count: usize,
    selected: bool,
}

impl HeptaFixtureChromeRoute {
    #[cfg(test)]
    fn detail_line(&self) -> String {
        let selected = if self.selected { "selected" } else { "ready" };
        format!("{} · {} · {} items", self.detail, selected, self.count)
    }

    fn display_line(&self) -> String {
        let state = if self.selected { "current" } else { "ready" };
        format!("{} · {}", self.detail, state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureAppChrome {
    workspace_title: &'static str,
    command_placeholder: &'static str,
    desktop_routes: [HeptaFixtureChromeRoute; 4],
    mobile_tabs: [&'static str; 4],
    live_side_effects: bool,
}

impl HeptaFixtureAppChrome {
    #[cfg(test)]
    fn command_line(&self) -> String {
        format!(
            "{} · workspace={} · live mutation=false",
            self.command_placeholder, self.workspace_title
        )
    }

    fn display_command_line(&self) -> String {
        format!("{} · local preview ready", self.command_placeholder)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureCommandResult {
    label: &'static str,
    title: &'static str,
    detail: &'static str,
    shortcut: &'static str,
    route: &'static str,
    requires_live_mutation: bool,
}

impl HeptaFixtureCommandResult {
    #[cfg(test)]
    fn preview_line(&self) -> String {
        format!(
            "{} · route={} · shortcut={} · mutation={}",
            self.detail, self.route, self.shortcut, self.requires_live_mutation
        )
    }

    fn display_line(&self) -> String {
        format!("{} · opens {}", self.detail, self.route)
    }

    #[cfg(test)]
    fn selection_route(&self) -> HeptaFixtureRouteKey {
        parse_fixture_route(self.route)
    }
}

fn fixture_command_results() -> [HeptaFixtureCommandResult; 4] {
    [
        HeptaFixtureCommandResult {
            label: "Ask",
            title: "Ask Hepta",
            detail: "request captured · local context ready",
            shortcut: "A",
            route: "Home",
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "Plan",
            title: "Review the plan",
            detail: "4 staged actions · approval first",
            shortcut: "P",
            route: "Actions",
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "Evidence",
            title: "Inspect evidence",
            detail: "local cards · review checks",
            shortcut: "E",
            route: "Inspector",
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "Approve",
            title: "Approve exact step",
            detail: "request preview · locked action",
            shortcut: "R",
            route: "Approvals",
            requires_live_mutation: false,
        },
    ]
}

fn fixture_secondary_command_results_for(
    surface: HeptaFixtureSecondarySurfaceKey,
) -> [HeptaFixtureCommandResult; 4] {
    let route_surface = fixture_secondary_route_surface_for(surface);
    let primary_panel = fixture_secondary_primary_panel_for(surface);
    [
        HeptaFixtureCommandResult {
            label: route_surface.route,
            title: route_surface.title,
            detail: route_surface.focus,
            shortcut: "1",
            route: route_surface.route,
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "Action",
            title: route_surface.primary_action,
            detail: primary_panel.operator_control,
            shortcut: "2",
            route: route_surface.route,
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "Evidence",
            title: route_surface.evidence_anchor,
            detail: primary_panel.evidence_stack,
            shortcut: "3",
            route: route_surface.route,
            requires_live_mutation: false,
        },
        HeptaFixtureCommandResult {
            label: "State",
            title: primary_panel.signal,
            detail: "local preview · no live mutation",
            shortcut: "4",
            route: route_surface.route,
            requires_live_mutation: false,
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRoutePage {
    route: &'static str,
    title: &'static str,
    primary_surface: &'static str,
    accepts_selection: bool,
    fixture_boundary: &'static str,
    live_side_effects: bool,
}

impl HeptaFixtureRoutePage {
    #[cfg(test)]
    fn contract_line(&self) -> String {
        let selection = if self.accepts_selection {
            "selection-aware"
        } else {
            "static shell"
        };
        format!(
            "{} · {} · {} · live mutation={}",
            self.primary_surface, selection, self.fixture_boundary, self.live_side_effects
        )
    }

    fn display_line(&self) -> String {
        let selection = if self.accepts_selection {
            "selection aware"
        } else {
            "read only"
        };
        format!("{} · {}", self.primary_surface, selection)
    }
}

fn fixture_route_shell_pages() -> [HeptaFixtureRoutePage; 4] {
    [
        HeptaFixtureRoutePage {
            route: "Home",
            title: "Ask Hepta",
            primary_surface: "request + current task",
            accepts_selection: true,
            fixture_boundary: "m.hepta.* timeline fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "Actions",
            title: "Review actions",
            primary_surface: "plan preview + pending steps",
            accepts_selection: true,
            fixture_boundary: "local action queue fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "Approvals",
            title: "Approve request",
            primary_surface: "request preview + approval status",
            accepts_selection: true,
            fixture_boundary: "approval fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "Inspector",
            title: "Inspect evidence",
            primary_surface: "evidence + action boundary",
            accepts_selection: true,
            fixture_boundary: "read-only inspector fixture",
            live_side_effects: false,
        },
    ]
}

fn fixture_secondary_route_shell_pages_for(
    surface: HeptaFixtureSecondarySurfaceKey,
) -> [HeptaFixtureRoutePage; 4] {
    let route_surface = fixture_secondary_route_surface_for(surface);
    let primary_panel = fixture_secondary_primary_panel_for(surface);
    [
        HeptaFixtureRoutePage {
            route: route_surface.route,
            title: route_surface.title,
            primary_surface: route_surface.focus,
            accepts_selection: true,
            fixture_boundary: "secondary surface fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "Action",
            title: route_surface.primary_action,
            primary_surface: primary_panel.operator_control,
            accepts_selection: true,
            fixture_boundary: "local action fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "Evidence",
            title: route_surface.evidence_anchor,
            primary_surface: primary_panel.evidence_stack,
            accepts_selection: true,
            fixture_boundary: "local evidence fixture",
            live_side_effects: false,
        },
        HeptaFixtureRoutePage {
            route: "State",
            title: primary_panel.signal,
            primary_surface: "visible first-viewport secondary surface proof",
            accepts_selection: true,
            fixture_boundary: "true-window fixture",
            live_side_effects: false,
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeptaFixtureRouteKey {
    Home,
    Actions,
    Approvals,
    Inspector,
}

impl Default for HeptaFixtureRouteKey {
    fn default() -> Self {
        Self::Home
    }
}

const HEPTA_FIXTURE_ROUTE_KEYS: [HeptaFixtureRouteKey; 4] = [
    HeptaFixtureRouteKey::Home,
    HeptaFixtureRouteKey::Actions,
    HeptaFixtureRouteKey::Approvals,
    HeptaFixtureRouteKey::Inspector,
];
#[cfg(test)]
const HEPTA_FIXTURE_SECONDARY_SURFACE_KEYS: [HeptaFixtureSecondarySurfaceKey; 5] = [
    HeptaFixtureSecondarySurfaceKey::Search,
    HeptaFixtureSecondarySurfaceKey::Settings,
    HeptaFixtureSecondarySurfaceKey::Attachment,
    HeptaFixtureSecondarySurfaceKey::Voice,
    HeptaFixtureSecondarySurfaceKey::Modal,
];
const HEPTA_FIXTURE_ROUTE_ROW_COUNT: usize = 3;
const HEPTA_FIXTURE_MOBILE_TASK_FIRST_MAX_WIDTH: f64 = 620.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeptaFixtureSecondarySurfaceKey {
    None,
    Search,
    Settings,
    Attachment,
    Voice,
    Modal,
}

impl Default for HeptaFixtureSecondarySurfaceKey {
    fn default() -> Self {
        Self::None
    }
}

impl HeptaFixtureSecondarySurfaceKey {
    fn is_none(self) -> bool {
        matches!(self, HeptaFixtureSecondarySurfaceKey::None)
    }

    fn label(self) -> &'static str {
        match self {
            HeptaFixtureSecondarySurfaceKey::None => "None",
            HeptaFixtureSecondarySurfaceKey::Search => "Search",
            HeptaFixtureSecondarySurfaceKey::Settings => "Settings",
            HeptaFixtureSecondarySurfaceKey::Attachment => "Attachment",
            HeptaFixtureSecondarySurfaceKey::Voice => "Voice",
            HeptaFixtureSecondarySurfaceKey::Modal => "Modal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeptaFixtureLayout {
    DesktopProduct,
    DesktopFull,
    MobileTaskFirst,
}

impl HeptaFixtureLayout {
    fn is_mobile_task_first(self) -> bool {
        matches!(self, HeptaFixtureLayout::MobileTaskFirst)
    }

    fn is_secondary_surface_visible(self) -> bool {
        matches!(self, HeptaFixtureLayout::DesktopFull)
    }

    fn log_label(self) -> &'static str {
        match self {
            HeptaFixtureLayout::DesktopProduct => "desktop-product",
            HeptaFixtureLayout::DesktopFull => "desktop-full",
            HeptaFixtureLayout::MobileTaskFirst => "mobile-task-first",
        }
    }
}

fn parse_fixture_layout(value: &str) -> Option<HeptaFixtureLayout> {
    match value.trim().to_ascii_lowercase().as_str() {
        "desktop" | "wide" | "product" | "desktop-product" | "desktop_product" => {
            Some(HeptaFixtureLayout::DesktopProduct)
        }
        "full" | "debug" | "desktop-full" | "desktop_full" => Some(HeptaFixtureLayout::DesktopFull),
        "mobile" | "phone" | "task-first" | "task_first" | "mobile-task-first"
        | "mobile_task_first" => Some(HeptaFixtureLayout::MobileTaskFirst),
        _ => None,
    }
}

fn fixture_layout_for_available_width(available_width: f64) -> HeptaFixtureLayout {
    if available_width.is_finite()
        && available_width > 0.0
        && available_width <= HEPTA_FIXTURE_MOBILE_TASK_FIRST_MAX_WIDTH
    {
        HeptaFixtureLayout::MobileTaskFirst
    } else {
        HeptaFixtureLayout::DesktopProduct
    }
}

fn selected_fixture_layout_for_width(available_width: f64) -> HeptaFixtureLayout {
    std::env::var("HEPTA_NATIVE_FIXTURE_LAYOUT")
        .ok()
        .as_deref()
        .and_then(parse_fixture_layout)
        .unwrap_or_else(|| fixture_layout_for_available_width(available_width))
}

fn fixture_secondary_surface_visible_for_layout(
    layout: HeptaFixtureLayout,
    secondary_surface: HeptaFixtureSecondarySurfaceKey,
) -> bool {
    layout.is_secondary_surface_visible()
        || (layout.is_mobile_task_first() && !secondary_surface.is_none())
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureLayoutContract {
    layout: HeptaFixtureLayout,
    visible_sections: Vec<&'static str>,
    collapsed_sections: Vec<&'static str>,
    live_side_effects: bool,
}

#[cfg(test)]
impl HeptaFixtureLayoutContract {
    fn for_layout(layout: HeptaFixtureLayout) -> Self {
        let (visible_sections, collapsed_sections) = match layout {
            HeptaFixtureLayout::MobileTaskFirst => (
                vec![
                    "telegram mobile chat",
                    "telegram chat header",
                    "telegram message thread",
                    "telegram composer",
                ],
                vec![
                    "desktop top bar",
                    "mobile top app bar",
                    "command bar",
                    "command palette",
                    "core flows",
                    "desktop route cards",
                    "current work",
                    "selected row detail",
                    "current work row cards",
                    "metrics",
                    "safety status",
                    "active route surface",
                    "route state cards",
                    "screen states",
                    "review queue",
                    "evidence timeline",
                ],
            ),
            HeptaFixtureLayout::DesktopProduct => (
                vec![
                    "telegram desktop shell",
                    "telegram chat list",
                    "telegram message thread",
                    "telegram composer",
                ],
                vec![
                    "desktop top bar",
                    "mobile top app bar",
                    "mobile dock",
                    "telegram info panel",
                    "command bar",
                    "command palette",
                    "desktop route cards",
                    "duplicate core flow cards",
                    "current work",
                    "selected row detail",
                    "current work row cards",
                    "metrics",
                    "safety status",
                    "active route surface",
                    "route state cards",
                    "screen states",
                    "review queue",
                    "evidence timeline",
                ],
            ),
            HeptaFixtureLayout::DesktopFull => (
                vec![
                    "desktop top bar",
                    "command bar",
                    "command palette",
                    "desktop route cards",
                    "core flows",
                    "current work",
                    "selected row detail",
                    "active route surface",
                    "route state cards",
                    "metrics",
                    "safety status",
                    "screen states",
                    "review queue",
                    "evidence timeline",
                ],
                vec![
                    "telegram desktop shell",
                    "telegram mobile chat",
                    "mobile top app bar",
                    "mobile dock",
                ],
            ),
        };

        Self {
            layout,
            visible_sections,
            collapsed_sections,
            live_side_effects: false,
        }
    }
}

impl HeptaFixtureRouteKey {
    fn label(self) -> &'static str {
        match self {
            HeptaFixtureRouteKey::Home => "Home",
            HeptaFixtureRouteKey::Actions => "Actions",
            HeptaFixtureRouteKey::Approvals => "Approvals",
            HeptaFixtureRouteKey::Inspector => "Inspector",
        }
    }

    fn content_anchor(self) -> &'static str {
        match self {
            HeptaFixtureRouteKey::Home => "metrics + event stack",
            HeptaFixtureRouteKey::Actions => "workbench + pending steps",
            HeptaFixtureRouteKey::Approvals => "approval request review",
            HeptaFixtureRouteKey::Inspector => "selected event details",
        }
    }

    fn selection_source(self) -> &'static str {
        match self {
            HeptaFixtureRouteKey::Home => "selected timeline event",
            HeptaFixtureRouteKey::Actions => "selected pending step",
            HeptaFixtureRouteKey::Approvals => "approval review item",
            HeptaFixtureRouteKey::Inspector => "selected evidence context",
        }
    }

    fn primary_action(self) -> &'static str {
        match self {
            HeptaFixtureRouteKey::Home => "draft next plan",
            HeptaFixtureRouteKey::Actions => "preview dry-run",
            HeptaFixtureRouteKey::Approvals => "inspect request detail",
            HeptaFixtureRouteKey::Inspector => "copy evidence summary",
        }
    }
}

fn parse_fixture_route(value: &str) -> HeptaFixtureRouteKey {
    match value.trim().to_ascii_lowercase().as_str() {
        "home" | "runtime" | "runtime cockpit" => HeptaFixtureRouteKey::Home,
        "action" | "actions" | "workbench" | "action workbench" => HeptaFixtureRouteKey::Actions,
        "approval" | "approvals" | "approval inbox" => HeptaFixtureRouteKey::Approvals,
        "inspect" | "inspector" | "runtime inspector" => HeptaFixtureRouteKey::Inspector,
        _ => HeptaFixtureRouteKey::Home,
    }
}

fn selected_fixture_route() -> HeptaFixtureRouteKey {
    std::env::var("HEPTA_NATIVE_FIXTURE_ROUTE")
        .ok()
        .as_deref()
        .map(parse_fixture_route)
        .unwrap_or(HeptaFixtureRouteKey::Home)
}

fn parse_fixture_secondary_surface(value: &str) -> HeptaFixtureSecondarySurfaceKey {
    match value.trim().to_ascii_lowercase().as_str() {
        "search" | "find" | "message-search" | "message_search" => {
            HeptaFixtureSecondarySurfaceKey::Search
        }
        "setting" | "settings" | "room-settings" | "room_settings" => {
            HeptaFixtureSecondarySurfaceKey::Settings
        }
        "attach" | "attachment" | "attachments" | "file" | "files" => {
            HeptaFixtureSecondarySurfaceKey::Attachment
        }
        "voice" | "voice-note" | "voice_note" | "audio" => HeptaFixtureSecondarySurfaceKey::Voice,
        "modal" | "confirm" | "confirmation" => HeptaFixtureSecondarySurfaceKey::Modal,
        _ => HeptaFixtureSecondarySurfaceKey::None,
    }
}

fn selected_fixture_secondary_surface() -> HeptaFixtureSecondarySurfaceKey {
    std::env::var("HEPTA_NATIVE_FIXTURE_SURFACE")
        .or_else(|_| std::env::var("HEPTA_NATIVE_FIXTURE_SECONDARY_SURFACE"))
        .ok()
        .as_deref()
        .map(parse_fixture_secondary_surface)
        .unwrap_or_default()
}

fn default_fixture_row_index_for(route: HeptaFixtureRouteKey) -> usize {
    match route {
        HeptaFixtureRouteKey::Home => 0,
        HeptaFixtureRouteKey::Actions
        | HeptaFixtureRouteKey::Approvals
        | HeptaFixtureRouteKey::Inspector => 1,
    }
}

fn sanitize_fixture_row_index(index: usize) -> usize {
    index.min(HEPTA_FIXTURE_ROUTE_ROW_COUNT - 1)
}

fn parse_fixture_row_index_for(route: HeptaFixtureRouteKey, value: &str) -> usize {
    let value = value.trim();
    if value.is_empty() {
        return default_fixture_row_index_for(route);
    }

    match value.to_ascii_lowercase().as_str() {
        "0" | "first" | "one" => return 0,
        "1" | "second" | "two" => return 1,
        "2" | "third" | "three" => return 2,
        _ => {}
    }

    let value = value.to_ascii_lowercase();
    let content = fixture_route_main_content_for(route);
    content
        .rows
        .iter()
        .position(|row| {
            row.label.to_ascii_lowercase() == value || row.title.to_ascii_lowercase() == value
        })
        .unwrap_or_else(|| default_fixture_row_index_for(route))
}

fn selected_fixture_row_for(route: HeptaFixtureRouteKey) -> usize {
    std::env::var("HEPTA_NATIVE_FIXTURE_ROW")
        .ok()
        .as_deref()
        .map(|value| parse_fixture_row_index_for(route, value))
        .unwrap_or_else(|| default_fixture_row_index_for(route))
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeptaFixtureInteractiveSelection {
    route: HeptaFixtureRouteKey,
    selected_row_index: usize,
    live_side_effects: bool,
}

#[cfg(test)]
impl HeptaFixtureInteractiveSelection {
    fn new(route: HeptaFixtureRouteKey, selected_row_index: usize) -> Self {
        Self {
            route,
            selected_row_index: sanitize_fixture_row_index(selected_row_index),
            live_side_effects: false,
        }
    }

    fn select_route(&mut self, route: HeptaFixtureRouteKey) {
        self.route = route;
        self.selected_row_index = default_fixture_row_index_for(route);
        self.live_side_effects = false;
    }

    fn select_row(&mut self, selected_row_index: usize) {
        self.selected_row_index = sanitize_fixture_row_index(selected_row_index);
        self.live_side_effects = false;
    }

    fn selected_detail(&self) -> HeptaFixtureSelectedRowDetail {
        fixture_selected_row_detail_for_row(self.route, self.selected_row_index)
    }

    fn selected_action_strip(&self) -> HeptaFixtureSelectedRowActionStrip {
        fixture_selected_row_action_strip_for(&self.selected_detail())
    }

    fn state_line(&self) -> String {
        format!(
            "{} · row={} · mutation={}",
            self.route.label(),
            self.selected_row_index,
            self.live_side_effects
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRouteState {
    route: &'static str,
    content_anchor: &'static str,
    selection_source: &'static str,
    primary_action: &'static str,
    active: bool,
    live_side_effects: bool,
}

impl HeptaFixtureRouteState {
    #[cfg(test)]
    fn state_line(&self) -> String {
        let active = if self.active { "active" } else { "standby" };
        format!(
            "{} · select={} · action={} · mutation={}",
            active, self.selection_source, self.primary_action, self.live_side_effects
        )
    }

    fn display_line(&self) -> String {
        let active = if self.active { "Current" } else { "Ready" };
        format!(
            "{} · {} · next: {}",
            active, self.selection_source, self.primary_action
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRouteSurface {
    route: &'static str,
    title: &'static str,
    focus: &'static str,
    primary_action: &'static str,
    evidence_anchor: &'static str,
    live_side_effects: bool,
}

impl HeptaFixtureRouteSurface {
    #[cfg(test)]
    fn surface_line(&self) -> String {
        format!(
            "{} · primary={} · evidence={} · mutation={}",
            self.focus, self.primary_action, self.evidence_anchor, self.live_side_effects
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRoutePrimaryPanel {
    route: &'static str,
    title: &'static str,
    signal: &'static str,
    operator_control: &'static str,
    evidence_stack: &'static str,
    empty_state: &'static str,
    live_side_effects: bool,
}

impl HeptaFixtureRoutePrimaryPanel {
    #[cfg(test)]
    fn empty_state_line(&self) -> String {
        format!(
            "empty={} · mutation={}",
            self.empty_state, self.live_side_effects
        )
    }

    fn display_empty_state_line(&self) -> String {
        format!("safe empty state: {}", self.empty_state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRouteMainRow {
    route: &'static str,
    label: &'static str,
    title: &'static str,
    detail: &'static str,
    live_side_effects: bool,
}

impl HeptaFixtureRouteMainRow {
    fn label_line(&self, selected: bool) -> String {
        if selected {
            format!("Selected · {}", self.label)
        } else {
            self.label.to_string()
        }
    }

    #[cfg(test)]
    fn preview_line(&self) -> String {
        format!(
            "{} · route={} · mutation={}",
            self.detail, self.route, self.live_side_effects
        )
    }

    fn display_line(&self) -> &'static str {
        self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureRouteMainContent {
    route: &'static str,
    title: &'static str,
    hint: &'static str,
    rows: [HeptaFixtureRouteMainRow; 3],
    live_side_effects: bool,
}

impl HeptaFixtureRouteMainContent {
    #[cfg(test)]
    fn hint_line(&self) -> String {
        format!(
            "{} · route={} · mutation={}",
            self.hint, self.route, self.live_side_effects
        )
    }

    #[cfg(test)]
    fn hint_line_with_selected_row(&self, selected_row_index: usize) -> String {
        let row = &self.rows[sanitize_fixture_row_index(selected_row_index)];
        format!(
            "{} · selected={} · route={} · mutation={}",
            self.hint, row.label, self.route, self.live_side_effects
        )
    }

    fn display_hint_with_selected_row(&self, selected_row_index: usize) -> String {
        let row = &self.rows[sanitize_fixture_row_index(selected_row_index)];
        format!("{} · selected: {}", self.hint, row.label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureSelectedRowDetail {
    route: &'static str,
    selected_row_index: usize,
    selected_row_label: &'static str,
    selected_row_title: &'static str,
    detail_title: &'static str,
    detail_body: &'static str,
    evidence: &'static str,
    inspector_title: &'static str,
    inspector_body: &'static str,
    live_side_effects: bool,
}

impl HeptaFixtureSelectedRowDetail {
    #[cfg(test)]
    fn route_line(&self) -> String {
        format!(
            "{} · selected row={} · row={}",
            self.route, self.selected_row_label, self.selected_row_index
        )
    }

    fn display_route_line(&self) -> String {
        format!("{} · {} selected", self.route, self.selected_row_label)
    }

    #[cfg(test)]
    fn evidence_line(&self) -> String {
        format!(
            "evidence={} · row={} · mutation={}",
            self.evidence, self.selected_row_label, self.live_side_effects
        )
    }

    fn display_evidence_line(&self) -> String {
        format!("Evidence: {} · review preview", self.evidence)
    }

    #[cfg(test)]
    fn inspector_line(&self) -> String {
        format!(
            "inspector={} · {}",
            self.inspector_title, self.inspector_body
        )
    }

    fn display_inspector_line(&self) -> String {
        format!("{} · {}", self.inspector_title, self.inspector_body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureSelectedRowActionStrip {
    route: &'static str,
    selected_row_label: &'static str,
    preview_action: &'static str,
    inspect_action: &'static str,
    copy_action: &'static str,
    execute_action: &'static str,
    execute_enabled: bool,
    live_side_effects: bool,
}

impl HeptaFixtureSelectedRowActionStrip {
    #[cfg(test)]
    fn action_lines(&self) -> [String; 4] {
        [
            format!(
                "Preview {} · {}",
                self.selected_row_label, self.preview_action
            ),
            format!(
                "Inspect {} · {}",
                self.selected_row_label, self.inspect_action
            ),
            format!("Copy {} · {}", self.selected_row_label, self.copy_action),
            format!(
                "{} · {} · enabled={} · mutation={}",
                self.execute_action,
                self.selected_row_label,
                self.execute_enabled,
                self.live_side_effects
            ),
        ]
    }

    fn display_action_lines(&self) -> [String; 4] {
        [
            format!("Preview · {}", self.preview_action),
            format!("Inspect · {}", self.inspect_action),
            format!("Copy · {}", self.copy_action),
            "Locked until approval".to_string(),
        ]
    }
}

#[cfg(test)]
fn fixture_route_states() -> [HeptaFixtureRouteState; 4] {
    fixture_route_states_for(HeptaFixtureRouteKey::Home)
}

fn fixture_route_states_for(selected_route: HeptaFixtureRouteKey) -> [HeptaFixtureRouteState; 4] {
    HEPTA_FIXTURE_ROUTE_KEYS.map(|route| HeptaFixtureRouteState {
        route: route.label(),
        content_anchor: route.content_anchor(),
        selection_source: route.selection_source(),
        primary_action: route.primary_action(),
        active: route == selected_route,
        live_side_effects: false,
    })
}

fn fixture_route_surface_for(route: HeptaFixtureRouteKey) -> HeptaFixtureRouteSurface {
    match route {
        HeptaFixtureRouteKey::Home => HeptaFixtureRouteSurface {
            route: route.label(),
            title: "Ask Hepta",
            focus: "chat, current task, review status, and latest evidence",
            primary_action: "draft next plan",
            evidence_anchor: "9 fixture events + 3 runtime lanes",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Actions => HeptaFixtureRouteSurface {
            route: route.label(),
            title: "Review actions",
            focus: "draft plan, pending steps, and staged confirmation queue",
            primary_action: "preview dry-run",
            evidence_anchor: "4 staged local steps",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Approvals => HeptaFixtureRouteSurface {
            route: route.label(),
            title: "Approval review surface",
            focus: "request preview, approval status, and decision",
            primary_action: "inspect request detail",
            evidence_anchor: "3 items waiting for approval",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Inspector => HeptaFixtureRouteSurface {
            route: route.label(),
            title: "Inspect evidence",
            focus: "selected evidence, app health, queue, and action boundary",
            primary_action: "copy evidence summary",
            evidence_anchor: "changes, AI calls, delivery, and execution paused",
            live_side_effects: false,
        },
    }
}

fn fixture_route_primary_panel_for(route: HeptaFixtureRouteKey) -> HeptaFixtureRoutePrimaryPanel {
    match route {
        HeptaFixtureRouteKey::Home => HeptaFixtureRoutePrimaryPanel {
            route: route.label(),
            title: "Ask Hepta",
            signal: "9 events / 3 active lanes / evidence ready",
            operator_control: "ask for a plan or inspect selected evidence",
            evidence_stack: "metrics + event groups + review status",
            empty_state: "empty workspace keeps composer disabled",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Actions => HeptaFixtureRoutePrimaryPanel {
            route: route.label(),
            title: "Review actions",
            signal: "4 staged steps / 2 confirmations / 1 approval block",
            operator_control: "preview dry-run before confirmation",
            evidence_stack: "plan preview + queue counts + request detail",
            empty_state: "empty queue stays read-only",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Approvals => HeptaFixtureRoutePrimaryPanel {
            route: route.label(),
            title: "Approval decision primary panel",
            signal: "3 approval items waiting",
            operator_control: "inspect request detail and approval reason",
            evidence_stack: "request preview + redaction + approval status",
            empty_state: "no approval selected shows a paused empty state",
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Inspector => HeptaFixtureRoutePrimaryPanel {
            route: route.label(),
            title: "Inspect evidence",
            signal: "evidence context selected",
            operator_control: "copy evidence summary",
            evidence_stack: "changes paused",
            empty_state: "no selection shows app health",
            live_side_effects: false,
        },
    }
}

fn fixture_active_surface_for(
    route: HeptaFixtureRouteKey,
    secondary: HeptaFixtureSecondarySurfaceKey,
) -> (HeptaFixtureRouteSurface, HeptaFixtureRoutePrimaryPanel) {
    if secondary.is_none() {
        return (
            fixture_route_surface_for(route),
            fixture_route_primary_panel_for(route),
        );
    }

    (
        fixture_secondary_route_surface_for(secondary),
        fixture_secondary_primary_panel_for(secondary),
    )
}

fn fixture_secondary_route_surface_for(
    surface: HeptaFixtureSecondarySurfaceKey,
) -> HeptaFixtureRouteSurface {
    match surface {
        HeptaFixtureSecondarySurfaceKey::Search => HeptaFixtureRouteSurface {
            route: surface.label(),
            title: "Find Messages",
            focus: "sender, date, and jump controls stay readable",
            primary_action: "jump to result",
            evidence_anchor: "3 grouped matches",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Settings => HeptaFixtureRouteSurface {
            route: surface.label(),
            title: "Room Settings",
            focus: "name, members, and notifications gather before changes",
            primary_action: "review setting",
            evidence_anchor: "name, members, notifications",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Attachment => HeptaFixtureRouteSurface {
            route: surface.label(),
            title: "Attachment Review",
            focus: "images, audio, and files stay staged before sending",
            primary_action: "choose source",
            evidence_anchor: "preview, caption, size",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Voice => HeptaFixtureRouteSurface {
            route: surface.label(),
            title: "Voice Note",
            focus: "audio preview, waveform, and retry controls stay visible",
            primary_action: "review audio",
            evidence_anchor: "timer, waveform, playback",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Modal => HeptaFixtureRouteSurface {
            route: surface.label(),
            title: "Confirm Change",
            focus: "target, result, and evidence stay attached before approval",
            primary_action: "keep reviewing",
            evidence_anchor: "selected chat + paused result",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::None => {
            fixture_route_surface_for(HeptaFixtureRouteKey::Home)
        }
    }
}

fn fixture_secondary_primary_panel_for(
    surface: HeptaFixtureSecondarySurfaceKey,
) -> HeptaFixtureRoutePrimaryPanel {
    match surface {
        HeptaFixtureSecondarySurfaceKey::Search => HeptaFixtureRoutePrimaryPanel {
            route: surface.label(),
            title: "Find Messages",
            signal: "3 matches / 2 filters / jump ready",
            operator_control: "filter results before opening",
            evidence_stack: "sender + file + people groups",
            empty_state: "no result keeps filters visible",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Settings => HeptaFixtureRoutePrimaryPanel {
            route: surface.label(),
            title: "Room Settings",
            signal: "name, members, notifications ready",
            operator_control: "review before applying",
            evidence_stack: "identity + permission summary",
            empty_state: "missing data keeps the form paused",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Attachment => HeptaFixtureRoutePrimaryPanel {
            route: surface.label(),
            title: "Attachment Review",
            signal: "image, audio, and file lanes ready",
            operator_control: "choose gallery, camera, files, or share",
            evidence_stack: "preview + caption + metadata",
            empty_state: "no file keeps send disabled",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Voice => HeptaFixtureRoutePrimaryPanel {
            route: surface.label(),
            title: "Voice Note",
            signal: "record, preview, drop, and send visible",
            operator_control: "review playback before sending",
            evidence_stack: "duration + levels + retry",
            empty_state: "no audio keeps controls paused",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::Modal => HeptaFixtureRoutePrimaryPanel {
            route: surface.label(),
            title: "Confirm Change",
            signal: "target selected / result paused",
            operator_control: "cancel, keep reviewing, or approve",
            evidence_stack: "details remain attached",
            empty_state: "no target keeps approval disabled",
            live_side_effects: false,
        },
        HeptaFixtureSecondarySurfaceKey::None => {
            fixture_route_primary_panel_for(HeptaFixtureRouteKey::Home)
        }
    }
}

fn fixture_route_main_content_for(route: HeptaFixtureRouteKey) -> HeptaFixtureRouteMainContent {
    match route {
        HeptaFixtureRouteKey::Home => HeptaFixtureRouteMainContent {
            route: route.label(),
            title: "Current work",
            hint: "Home keeps request, plan, evidence, and approval status together",
            rows: [
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Current task",
                    title: "9 events · 3 active lanes",
                    detail: "timeline summary, active lanes, and review badge",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Status",
                    title: "No live actions",
                    detail: "changes remain paused until approval",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Event stack",
                    title: "Runtime / action / evidence",
                    detail: "grouped cards keep selection context visible",
                    live_side_effects: false,
                },
            ],
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Actions => HeptaFixtureRouteMainContent {
            route: route.label(),
            title: "Action review",
            hint: "Actions keeps proposed steps reviewable before approval",
            rows: [
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Draft plan",
                    title: "Plan preview",
                    detail: "draft plan is staged locally before confirmation",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Pending steps",
                    title: "1 ready · 2 confirm · 1 blocked",
                    detail: "queued steps stay visible before dispatch",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Request detail",
                    title: "exact review required",
                    detail: "request evidence is visible before execution",
                    live_side_effects: false,
                },
            ],
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Approvals => HeptaFixtureRouteMainContent {
            route: route.label(),
            title: "Approval decision",
            hint: "Approvals keeps request, context, and decision together",
            rows: [
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Approval check",
                    title: "Local build approval",
                    detail: "blocked item waits for a clear approval",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Request detail",
                    title: "redacted request preview",
                    detail: "exact command evidence stays inspectable",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Decision",
                    title: "approval required",
                    detail: "approval state is visible before anything runs",
                    live_side_effects: false,
                },
            ],
            live_side_effects: false,
        },
        HeptaFixtureRouteKey::Inspector => HeptaFixtureRouteMainContent {
            route: route.label(),
            title: "Evidence review",
            hint: "Inspector explains selected evidence and action status",
            rows: [
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Evidence",
                    title: "selected evidence context",
                    detail: "selected event details stay attached to this task",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "Action boundary",
                    title: "Actions paused",
                    detail: "action status remains explicit",
                    live_side_effects: false,
                },
                HeptaFixtureRouteMainRow {
                    route: route.label(),
                    label: "App health",
                    title: "fallback states ready",
                    detail: "loading, empty, and error states remain readable",
                    live_side_effects: false,
                },
            ],
            live_side_effects: false,
        },
    }
}

#[cfg(test)]
fn fixture_selected_row_detail_for(route: HeptaFixtureRouteKey) -> HeptaFixtureSelectedRowDetail {
    fixture_selected_row_detail_for_row(route, default_fixture_row_index_for(route))
}

fn fixture_selected_row_detail_for_row(
    route: HeptaFixtureRouteKey,
    selected_row_index: usize,
) -> HeptaFixtureSelectedRowDetail {
    let selected_row_index = sanitize_fixture_row_index(selected_row_index);
    let content = fixture_route_main_content_for(route);
    let row = &content.rows[selected_row_index];
    let (detail_title, detail_body, evidence, inspector_title, inspector_body) = match (
        route,
        selected_row_index,
    ) {
        (HeptaFixtureRouteKey::Home, 0) => (
            "Current task summary",
            "The Home inspector keeps event count, active lane count, and review status in one read-only summary.",
            "timeline summary + review badge",
            "Home inspector",
            "selection remains read-only; no runtime connection opened",
        ),
        (HeptaFixtureRouteKey::Home, 1) => (
            "Review boundary",
            "The Home inspector keeps actions, delivery, and process state paused until approval.",
            "actions paused + delivery paused + execution paused",
            "Home review inspector",
            "actions stay paused",
        ),
        (HeptaFixtureRouteKey::Home, _) => (
            "Evidence stack",
            "The Home inspector keeps runtime, action, and evidence lanes grouped around the current selection.",
            "runtime lane + action lane + evidence lane",
            "Home event inspector",
            "evidence stays local",
        ),
        (HeptaFixtureRouteKey::Actions, 0) => (
            "Draft plan summary",
            "Actions keeps the draft plan, dry-run preview, and disabled submit affordance visible together.",
            "draft plan + dry-run preview",
            "Actions reviewer",
            "submit remains disabled",
        ),
        (HeptaFixtureRouteKey::Actions, 1) => (
            "Pending steps summary",
            "Actions keeps queued local steps, confirmations, and approval blocks inspectable before dispatch.",
            "queue counts + request preview",
            "Actions reviewer",
            "dispatch remains disabled",
        ),
        (HeptaFixtureRouteKey::Actions, _) => (
            "Request detail",
            "Actions keeps exact request evidence inspectable while execution stays unavailable.",
            "request preview + approval evidence",
            "Actions request reviewer",
            "execution remains disabled",
        ),
        (HeptaFixtureRouteKey::Approvals, 0) => (
            "Approval check detail",
            "Approvals keeps the blocked operation and approval reason attached to the pending decision.",
            "approval reason + blocked operation",
            "Approval reviewer",
            "approval remains required",
        ),
        (HeptaFixtureRouteKey::Approvals, 1) => (
            "Request detail",
            "Approvals keeps redaction, request evidence, and approval status together.",
            "request preview + approval reason",
            "Approvals reviewer",
            "approval still required",
        ),
        (HeptaFixtureRouteKey::Approvals, _) => (
            "Decision detail",
            "Approvals keeps the required confirmation visible without opening a mutation path.",
            "approval confirmation + disabled execute",
            "Approvals decision inspector",
            "live execution remains closed",
        ),
        (HeptaFixtureRouteKey::Inspector, 0) => (
            "Evidence detail",
            "Inspector keeps local evidence context attached to the detail pane.",
            "local evidence context + selected event",
            "Evidence reviewer",
            "local evidence only",
        ),
        (HeptaFixtureRouteKey::Inspector, 1) => (
            "Action boundary detail",
            "Inspector pins action, delivery, and process state together.",
            "actions paused + execution paused",
            "Review status",
            "app health stays read-only",
        ),
        (HeptaFixtureRouteKey::Inspector, _) => (
            "App health detail",
            "Inspector keeps loading, empty, and error surfaces ready without a runtime connection.",
            "loading + empty + error states",
            "App health reviewer",
            "runtime connection stays closed",
        ),
    };

    HeptaFixtureSelectedRowDetail {
        route: route.label(),
        selected_row_index,
        selected_row_label: row.label,
        selected_row_title: row.title,
        detail_title,
        detail_body,
        evidence,
        inspector_title,
        inspector_body,
        live_side_effects: false,
    }
}

fn fixture_selected_row_action_strip_for(
    detail: &HeptaFixtureSelectedRowDetail,
) -> HeptaFixtureSelectedRowActionStrip {
    let (preview_action, inspect_action, copy_action) =
        match (detail.route, detail.selected_row_label) {
            ("Home", "Current task") => {
                ("task summary", "timeline evidence", "copy evidence summary")
            }
            ("Home", "Status") => ("status preview", "action boundary", "copy status evidence"),
            ("Home", "Event stack") => (
                "event stack preview",
                "lane evidence",
                "copy stack evidence",
            ),
            ("Actions", "Draft plan") => {
                ("plan preview", "draft plan evidence", "copy plan evidence")
            }
            ("Actions", "Pending steps") => ("step summary", "queued steps", "copy step evidence"),
            ("Actions", "Request detail") => (
                "request preview",
                "approval evidence",
                "copy request evidence",
            ),
            ("Approvals", "Approval check") => (
                "approval preview",
                "approval reason",
                "copy approval evidence",
            ),
            ("Approvals", "Request detail") => {
                ("request preview", "exact request", "copy request evidence")
            }
            ("Approvals", "Decision") => (
                "decision preview",
                "approval confirmation",
                "copy decision evidence",
            ),
            ("Inspector", "Evidence") => ("evidence preview", "selected evidence", "copy evidence"),
            ("Inspector", "Action boundary") => (
                "boundary preview",
                "action boundary",
                "copy boundary evidence",
            ),
            ("Inspector", "App health") => {
                ("health preview", "fallback states", "copy health evidence")
            }
            _ => ("row preview", "row evidence", "copy evidence"),
        };

    HeptaFixtureSelectedRowActionStrip {
        route: detail.route,
        selected_row_label: detail.selected_row_label,
        preview_action,
        inspect_action,
        copy_action,
        execute_action: "Execute disabled",
        execute_enabled: false,
        live_side_effects: false,
    }
}

#[cfg(test)]
fn fixture_app_chrome() -> HeptaFixtureAppChrome {
    fixture_app_chrome_for(HeptaFixtureRouteKey::Home)
}

fn fixture_app_chrome_for(selected_route: HeptaFixtureRouteKey) -> HeptaFixtureAppChrome {
    HeptaFixtureAppChrome {
        workspace_title: "Hepta Runtime Cockpit",
        command_placeholder: "Ask Hepta for a focused plan",
        desktop_routes: [
            HeptaFixtureChromeRoute {
                label: "Home",
                title: "Ask",
                detail: "current task",
                count: 9,
                selected: selected_route == HeptaFixtureRouteKey::Home,
            },
            HeptaFixtureChromeRoute {
                label: "Actions",
                title: "Review",
                detail: "action queue",
                count: 4,
                selected: selected_route == HeptaFixtureRouteKey::Actions,
            },
            HeptaFixtureChromeRoute {
                label: "Approvals",
                title: "Approve",
                detail: "needs decision",
                count: 3,
                selected: selected_route == HeptaFixtureRouteKey::Approvals,
            },
            HeptaFixtureChromeRoute {
                label: "Inspector",
                title: "Inspect",
                detail: "evidence",
                count: 1,
                selected: selected_route == HeptaFixtureRouteKey::Inspector,
            },
        ],
        mobile_tabs: ["Home", "Actions", "Approvals", "Inspector"],
        live_side_effects: false,
    }
}

fn fixture_shell_state_cards() -> [HeptaFixtureShellState; 3] {
    [
        HeptaFixtureShellState {
            label: "Loading",
            title: "Sync preview",
            body: "Room timeline and action queue use local preview data while sync is not connected.",
            live_side_effects: false,
        },
        HeptaFixtureShellState {
            label: "Empty",
            title: "No active room",
            body: "Workspace placeholder keeps composer disabled until a room is selected.",
            live_side_effects: false,
        },
        HeptaFixtureShellState {
            label: "Error",
            title: "Permission blocked",
            body: "Actions, delivery, and process execution remain paused behind approval gates.",
            live_side_effects: false,
        },
    ]
}

impl HeptaFixtureCockpitSummary {
    fn bridge_badge(&self) -> &'static str {
        if self.current_bridge_visible {
            "ready"
        } else {
            "missing"
        }
    }

    #[cfg(test)]
    fn safety_line(&self) -> String {
        format!(
            "Fixture mode skips Matrix SDK loop and keeps Gateway, provider, channel delivery, process execution, and external mutation false. {} completed cards remain read-only evidence.",
            self.completed_events
        )
    }

    fn safety_display_line(&self) -> String {
        format!(
            "Preview mode. Actions, delivery, and automation stay paused; {} completed cards are available as evidence.",
            self.completed_events
        )
    }
}

fn summarize_fixture_events(events: &[HeptaFixtureMatrixEvent]) -> HeptaFixtureCockpitSummary {
    let mut summary = HeptaFixtureCockpitSummary {
        total_events: events.len(),
        active_events: 0,
        waiting_events: 0,
        completed_events: 0,
        current_bridge_visible: false,
    };

    for event in events {
        let Ok(envelope) = HeptaEventEnvelope::from_content_value(&event.content) else {
            continue;
        };
        match envelope.status {
            HeptaEventStatus::Started | HeptaEventStatus::Running => {
                summary.active_events += 1;
            }
            HeptaEventStatus::Waiting => {
                summary.waiting_events += 1;
            }
            HeptaEventStatus::Completed => {
                summary.completed_events += 1;
            }
            HeptaEventStatus::Failed | HeptaEventStatus::Blocked | HeptaEventStatus::Cancelled => {}
        }
        if envelope.id == "current-codex-runtime-bridge" {
            summary.current_bridge_visible = true;
        }
    }
    summary
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct HeptaFixtureEventGroups {
    runtime: usize,
    action: usize,
    evidence: usize,
}

fn summarize_event_groups(events: &[HeptaFixtureMatrixEvent]) -> HeptaFixtureEventGroups {
    let mut groups = HeptaFixtureEventGroups::default();
    for event in events {
        match classify_fixture_event(event) {
            HeptaFixtureEventGroup::Runtime => groups.runtime += 1,
            HeptaFixtureEventGroup::Action => groups.action += 1,
            HeptaFixtureEventGroup::Evidence => groups.evidence += 1,
        }
    }
    groups
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeptaFixtureEventGroup {
    Runtime,
    Action,
    Evidence,
}

fn classify_fixture_event(event: &HeptaFixtureMatrixEvent) -> HeptaFixtureEventGroup {
    let envelope = HeptaEventEnvelope::from_content_value(&event.content).ok();
    if event.event_type == EVENT_RUNTIME_EVENT
        || envelope
            .as_ref()
            .is_some_and(|envelope| envelope.id == "current-codex-runtime-bridge")
    {
        return HeptaFixtureEventGroup::Runtime;
    }
    if matches!(
        event.event_type,
        EVENT_TOOL_RESULT | EVENT_AGENT_RUN | EVENT_MEMORY_CITATION
    ) {
        return HeptaFixtureEventGroup::Evidence;
    }
    HeptaFixtureEventGroup::Action
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeptaFixtureOperationWorkbench {
    item_count: usize,
    local_preview_count: usize,
    awaiting_confirmation_count: usize,
    policy_blocked_count: usize,
    all_external_mutation_disabled: bool,
    composer_title: String,
    composer_body: String,
    approval_title: String,
    approval_body: String,
    outbox_title: String,
    outbox_body: String,
}

impl HeptaFixtureOperationWorkbench {
    fn composer_display_body(&self) -> String {
        self.composer_body
            .replace(" · mutation=false", " · review preview")
    }

    fn approval_display_body(&self) -> String {
        self.approval_body
            .replace(" · mutation=false", " · approval required")
            .replace("mutation=false", "approval required")
    }

    fn outbox_display_body(&self) -> String {
        format!(
            "{} local preview · {} need confirmation · {} blocked",
            self.local_preview_count, self.awaiting_confirmation_count, self.policy_blocked_count
        )
    }
}

fn summarize_operation_workbench() -> HeptaFixtureOperationWorkbench {
    let items = sample_action_queue_items();
    let action_summary = summarize_action_queue(&items);
    let selected = selected_action_detail(&items);
    let inspections = inspect_action_outbox(&items);
    let approval = items
        .iter()
        .find(|item| item.stage == HeptaActionQueueStage::PolicyBlocked);
    let approval_inspection = approval.and_then(|approval| {
        inspections
            .iter()
            .find(|inspection| inspection.item_id == approval.id)
    });
    let all_external_mutation_disabled = items.iter().all(|item| !item.external_mutation_enabled);

    let (composer_title, composer_body) = selected
        .map(|detail| {
            (
                format!(
                    "{} · {}",
                    readable_token(detail.stage),
                    readable_token(&detail.mutation_class)
                ),
                format!(
                    "{} · {} · mutation=false",
                    detail.title, detail.target_display
                ),
            )
        })
        .unwrap_or_else(|| {
            (
                "No selected dry-run".to_string(),
                "The local action queue has no composer preview item.".to_string(),
            )
        });

    let (approval_title, approval_body) = match (approval, approval_inspection) {
        (Some(approval), Some(inspection)) => (
            approval.title.clone(),
            format!(
                "{} · request={} · preview required · mutation={}",
                readable_token(inspection.policy_decision_label),
                short_hash(&inspection.exact_payload_hash),
                inspection.external_mutation_enabled,
            ),
        ),
        _ => (
            "No approval request".to_string(),
            "No approval review item is queued in the local fixture.".to_string(),
        ),
    };

    HeptaFixtureOperationWorkbench {
        item_count: items.len(),
        local_preview_count: action_summary.local_preview,
        awaiting_confirmation_count: action_summary.awaiting_confirmation,
        policy_blocked_count: action_summary.policy_blocked,
        all_external_mutation_disabled,
        composer_title,
        composer_body,
        approval_title,
        approval_body,
        outbox_title: format!("{} staged local actions", items.len()),
        outbox_body: format!(
            "local={} · confirm={} · blocked={} · mutation={}",
            action_summary.local_preview,
            action_summary.awaiting_confirmation,
            action_summary.policy_blocked,
            !all_external_mutation_disabled
        ),
    }
}

fn short_hash(hash: &str) -> String {
    hash.chars().take(8).collect()
}

fn readable_token(value: &str) -> String {
    value.replace('_', " ")
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

#[cfg(test)]
mod tests;

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.RobrixSplitter = Splitter {
        // size: theme.splitter_size
        // min_horizontal: theme.splitter_min_horizontal
        // max_horizontal: theme.splitter_max_horizontal
        // min_vertical: theme.splitter_min_vertical
        // max_vertical: theme.splitter_max_vertical

        draw_bg +: {
            color: COLOR_HEPTA_HAIRLINE
            color_hover: COLOR_HEPTA_FOCUS
            color_drag: COLOR_HEPTA_FOCUS

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                // A quiet hairline until the user reaches for the divider.
                let body_color = mix(
                    self.color
                    mix(self.color_hover, self.color_drag, self.drag)
                    self.hover
                )
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    1.5
                )
                sdf.fill(body_color)

                // Draw the grab bar shape
                if self.is_vertical > 0.5 {
                    sdf.box(
                        self.splitter_pad
                        self.rect_size.y * 0.5 - self.bar_size * 0.5
                        self.rect_size.x - 2.0 * self.splitter_pad
                        self.bar_size
                        self.border_radius
                    )
                }
                else {
                    sdf.box(
                        self.rect_size.x * 0.5 - self.bar_size * 0.5
                        self.splitter_pad
                        self.bar_size
                        self.rect_size.y - 2.0 * self.splitter_pad
                        self.border_radius
                    )
                }

                let grab_color = mix(self.color, self.color_hover, self.hover)
                return sdf.fill_keep(grab_color)
            }
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {drag: 0.0, hover: 0.0}
                    }
                }

                on: AnimatorState{
                    from: {
                        all: Forward {duration: 0.1}
                        drag: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {
                            drag: 0.0,
                            hover: snap(1.0)
                        }
                    }
                }

                drag: AnimatorState{
                    from: { all: Forward { duration: 0.1 }}
                    apply: {
                        draw_bg: {
                            drag: snap(1.0),
                            hover: 1.0
                        }
                    }
                }
            }
        }
    }

    mod.widgets.RobrixTabCloseButton = TabCloseButton {
        height: 10.0
        width: 10.0
        margin: Inset{ right: theme.space_2, left: -1 }
        draw_button +: {
            color: COLOR_HEPTA_MUTED
            color_hover: COLOR_HEPTA_DANGER
            color_active: COLOR_HEPTA_TEXT
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_button: {hover: 0.0}
                    }
                }

                on: AnimatorState{
                    cursor: MouseCursor.Hand
                    from: {all: Snap}
                    apply: {
                        draw_button: {hover: 1.0}
                    }
                }
            }
        }
    }

    mod.widgets.RobrixTab = Tab {
        width: Fit
        height: Fill

        align: Align{x: 0.0, y: 0.5}
        padding: Inset{left: 12, right: 12, top: 8, bottom: 8}
        margin: Inset{left: 2, right: 2, top: 3, bottom: 3}

        close_button: mod.widgets.RobrixTabCloseButton {}
        draw_text +: {
            text_style: theme.font_regular {}

            color: COLOR_HEPTA_MUTED
            color_hover: COLOR_HEPTA_TEXT
            color_active: COLOR_HEPTA_FOCUS
        }

        draw_bg +: {
            color: #xFFFFFF00
            color_2: #xFFFFFF00
            color_hover: COLOR_HEPTA_GLASS
            color_2_hover: COLOR_HEPTA_GLASS_STRONG
            color_active: COLOR_HEPTA_FOCUS_SURFACE
            color_2_active: COLOR_HEPTA_GLASS_STRONG
            border_size: 1.0
            border_color: COLOR_HEPTA_HAIRLINE
            border_radius: (HEPTA_RADIUS_CONTROL)
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {hover: 0.0}
                        draw_text: {hover: 0.0}
                    }
                }

                on: AnimatorState{
                    cursor: MouseCursor.Hand
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: snap(1.0)}
                        draw_text: {hover: snap(1.0)}
                    }
                }
            }

            active: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        close_button: {draw_button: {active: 0.0}}
                        draw_bg: {active: 0.0}
                        draw_text: {active: 0.0}
                    }
                }

                on: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        close_button: {draw_button: {active: 1.0}}
                        draw_bg: {active: 1.0}
                        draw_text: {active: 1.0}
                    }
                }
            }
        }
    }

    mod.widgets.RobrixTabBar = TabBar {
        CloseableTab := mod.widgets.RobrixTab {closeable: true}
        PermanentTab := mod.widgets.RobrixTab {closeable: false}

        draw_drag +: {
            draw_depth: 10
            color: #x0
        }
        draw_fill +: {
            color: COLOR_HEPTA_GLASS_STRONG
        }
        draw_bg +: {
            color: COLOR_HEPTA_GLASS_STRONG
        }

        width: Fill
        height: max(theme.tab_height, 25.)

        scroll_bars: ScrollBarsTabs {
            show_scroll_x: true
            show_scroll_y: false
            scroll_bar_x +: {
                bar_size: 4
                use_vertical_finger_scroll: true
            }
        }
    }

    mod.widgets.RobrixDock = Dock {
        flow: Down

        round_corner +: {
            color: COLOR_HEPTA_ENVIRONMENT
        }

        padding: Inset{left: theme.dock_border_size, top: 0, right: theme.dock_border_size, bottom: theme.dock_border_size}
        drag_target_preview +: {
            draw_depth: 10.0
            color: mix(COLOR_ACTIVE_PRIMARY, #FFFFFF00, 0.5)
        }
        tab_bar: mod.widgets.RobrixTabBar {}
        splitter: mod.widgets.RobrixSplitter {}
    }
}

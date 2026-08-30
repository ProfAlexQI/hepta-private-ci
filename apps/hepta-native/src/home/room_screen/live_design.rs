script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.COLOR_BG = #xfff8ee
    mod.widgets.COLOR_OVERLAY_BG = #x000000d8
    mod.widgets.COLOR_READ_MARKER = #xeb2733

    mod.widgets.REACTION_TEXT_COLOR = #4c00b0

    mod.widgets.COLOR_THREAD_SUMMARY_BG = #FFF4E5
    mod.widgets.COLOR_THREAD_SUMMARY_BG_HOVER = #FFEACC
    mod.widgets.COLOR_THREAD_SUMMARY_BORDER = #E8C99A
    mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT = #A35A00

    // An empty view that takes up no space in the portal list.
    mod.widgets.Empty = View { }

    // A download button or loading spinner shown beneath a message.
    mod.widgets.MessageDownloadSection = View {
        visible: false,
        width: Fit, height: Fit,
        flow: Right,
        margin: Inset{top: 8, bottom: 2}

        download_button := RobrixIconButton {
            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
            padding: Inset{left: 12, right: 12}
            margin: 0
            draw_icon.svg: (ICON_DOWNLOAD)
            icon_walk: Walk{width: 16, height: 16}
            text: "Download"
        }

        share_button := RobrixIconButton {
            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
            padding: Inset{left: 12, right: 12}
            margin: Inset{left: 8}
            draw_icon.svg: (ICON_SHARE)
            icon_walk: Walk{width: 16, height: 16}
            text: "Share"
        }

        downloading_view := View {
            visible: false,
            width: Fit, height: mod.widgets.SETTINGS_BUTTON_HEIGHT
            flow: Right,
            align: Align{y: 0.5}
            spacing: 8,
            padding: Inset{left: 12, right: 6}

            spinner := LoadingSpinner {
                width: 16, height: 16
                draw_bg.color: (COLOR_ACTIVE_PRIMARY)
            }
            status_label := Label {
                width: Fit, height: Fit,
                padding: 0
                margin: 0
                draw_text +: {
                    text_style: REGULAR_TEXT { font_size: 11 },
                    color: (COLOR_ACTIVE_PRIMARY)
                }
                text: "Downloading…"
            }
            cancel_button := RobrixNegativeIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{left: 12, right: 12}
                margin: 0
                draw_icon.svg: (ICON_CLOSE)
                icon_walk: Walk{width: 16, height: 16}
                text: "Cancel"
            }
        }

        success_button := RobrixPositiveIconButton {
            visible: false,
            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
            padding: Inset{left: 12, right: 12}
            margin: 0
            draw_icon.svg: (ICON_CHECKMARK)
            icon_walk: Walk{width: 16, height: 16}
            text: "Downloaded"
        }

        failure_button := RobrixNegativeIconButton {
            visible: false,
            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
            padding: Inset{left: 12, right: 12}
            margin: 0
            draw_icon.svg: (ICON_CLOSE)
            icon_walk: Walk{width: 16, height: 16}
            text: "Download Failed"
        }
    }

    // A summary at the bottom of a message that is the root of a thread.
    mod.widgets.ThreadRootSummary = RoundedView {
        visible: false
        width: Fill,
        height: Fit
        flow: Right,
        align: Align{x: 0.0, y: 0.5}
        spacing: 5.0
        margin: Inset{ top: 5.0 }
        padding: 12,
        cursor: MouseCursor.Hand

        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_THREAD_SUMMARY_BG)
            border_radius: (HEPTA_RADIUS_CONTROL)
            border_size: 1.5
            border_color: (mod.widgets.COLOR_THREAD_SUMMARY_BORDER)
        }

        thread_summary_count := Label {
            width: Fit,
            draw_text +: {
                text_style: USERNAME_TEXT_STYLE { font_size: 11 }
                color: (mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT)
            }
            text: ""
        }

        Icon {
            width: Fit, height: Fit,
            align: Align{x: 0.5, y: 0.5}
            draw_icon +: {
                svg: crate_resource("self://resources/icons/double_chat.svg")
                color: (mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT)
            }
            icon_walk: Walk{ width: 25, height: 25, margin: Inset{top: 3, right: 7} }
        }

        thread_summary_latest := MessageHtml {
            max_lines: 2
            text_overflow: Ellipsis
        }
    }

    // The view used for each text-based message event in a room's timeline.
    mod.widgets.Message = set_type_default() do #(Message::register_widget(vm)) {

        width: Fill,
        height: Fit,
        margin: 0.0
        flow: Down,
        cursor: MouseCursor.Default,
        padding: 0.0,
        spacing: 0.0

        show_bg: true
        draw_bg +: {
            highlight: instance(0.0)
            hover: instance(0.0)
            color: instance((COLOR_HEPTA_CONTENT))

            mentions_bar_color: instance((COLOR_HEPTA_CONTENT))
            mentions_bar_width: instance(4.0)

            pixel: fn() {
                let base_color = mix(
                    self.color,
                    COLOR_HEPTA_SURFACE,
                    self.hover
                );

                let with_highlight = mix(
                    base_color,
                    COLOR_HEPTA_FOCUS_SURFACE,
                    self.highlight
                );

                let sdf = Sdf2d.viewport(self.pos * self.rect_size);

                // draw bg
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(with_highlight);

                // draw the left vertical line
                sdf.rect(0., 0., self.mentions_bar_width, self.rect_size.y);
                sdf.fill(self.mentions_bar_color);

                return sdf.result;
            }
        }

        animator: Animator{
            highlight: {
                default: @off
                off: AnimatorState{
                    redraw: true,
                    from: { all: Forward {duration: 2.0} }
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { draw_bg: {highlight: 0.0} }
                }
                on: AnimatorState{
                    redraw: true,
                    from: { all: Forward {duration: 0.5} }
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { draw_bg: {highlight: 1.0} }
                }
            }
            hover: {
                default: @off
                off: AnimatorState{
                    redraw: true,
                    from: { all: Snap }
                    apply: { draw_bg: {hover: 0.0} }
                }
                on: AnimatorState{
                    redraw: true,
                    from: { all: Snap }
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }

        // A preview of the earlier message that this message was in reply to.
        replied_to_message := mod.widgets.RepliedToMessage {
            flow: Down
            margin: Inset{ bottom: 3, top: 10 }
            preview_content +: {
                margin +: { left: 29 }
                padding +: { bottom: 10 }
            }
        }

        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{top: 0, bottom: 10, left: 10, right: 10},

            profile := View {
                align: Align{x: 0.5, y: 0.0} // centered horizontally, top aligned
                width: 65.0,
                height: Fit,
                margin: Inset{top: 4.5, right: 10}
                flow: Down,
                avatar := Avatar {
                    width: 48,
                    height: 48,
                }
                timestamp := Timestamp {
                    margin: Inset{ top: 5.9 }
                }
                edited_indicator := EditedIndicator { }
                tsp_sign_indicator := TspSignIndicator { }
            }

            content := View {
                width: Fill,
                height: Fit
                flow: Down,
                padding: 0.0

                username_view := View {
                    flow: Right,
                    width: Fill,
                    height: Fit,
                    username := Label {
                        width: Fill,
                        flow: Right, // do not wrap
                        padding: 0,
                        margin: Inset{bottom: 9.0, top: 20.0, right: 10.0,}
                        max_lines: 1
                        text_overflow: Ellipsis
                        draw_text +: {
                            text_style: USERNAME_TEXT_STYLE {},
                            color: (USERNAME_TEXT_COLOR)
                        }
                        text: "<Username not available>"
                    }
                }

                message := HtmlOrPlaintext { }
                link_preview_view := mod.widgets.LinkPreview {}
                download_section := mod.widgets.MessageDownloadSection {}
                View {
                    width: Fill,
                    height: Fit
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }

    // The view used for a condensed message that came right after another message
    // from the same sender, and thus doesn't need to display the sender's profile again.
    mod.widgets.CondensedMessage = mod.widgets.Message {
        padding: Inset{ top: 2.0, bottom: 2.0 }
        replied_to_message +: {
            preview_content +: {
                margin: Inset{ left: 74, bottom: 5.0 }
            }
        }
        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{ top: 0, bottom: 2.5, left: 10.0, right: 10.0 },
            profile := View {
                align: Align{x: 0.5, y: 0.0} // centered horizontally, top aligned
                width: 65.0,
                height: Fit,
                flow: Down,
                timestamp := Timestamp {
                    margin: Inset{top: 2.5}
                }
                edited_indicator := EditedIndicator { }
                tsp_sign_indicator := TspSignIndicator { }
            }
            content := View {
                width: Fill,
                height: Fit,
                flow: Down,
                padding: Inset{ left: 10.0 }

                message := HtmlOrPlaintext { }
                link_preview_view := mod.widgets.LinkPreview {}
                download_section := mod.widgets.MessageDownloadSection {}
                View {
                    width: Fill,
                    height: Fit
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }

    // A single shared object on the script heap of type `Size::Fit{max: ...}`,
    // which is used for the max image thumbnail height for every `Image` widget
    // within a message widget.
    // Also see: `AppPreferences::on_thumbnail_max_height_changed`).
    mod.widgets.IMG_MSG_FIT = Fit{max: FitBound.Abs(300.0)}

    // The view used for each static image-based message event in a room's timeline.
    // This excludes stickers and other animated GIFs, video clips, audio clips, etc.
    mod.widgets.ImageMessage = mod.widgets.Message {
        body +: {
            content +: {
                message := View {
                    width: Fill, height: Fit,
                    flow: Down,
                    caption_view := View {
                        visible: false,
                        width: Fill, height: Fit,
                        margin: Inset{ bottom: 5.0 }
                        caption := HtmlOrPlaintext {}
                    }
                    image := TextOrImage {
                        image_view +: { image +: {
                            height: (mod.widgets.IMG_MSG_FIT)
                        } }
                    }
                }
                download_section := mod.widgets.MessageDownloadSection {}
                View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }

        }
    }

    // The view used for a condensed image message that came right after another message
    // from the same sender, and thus doesn't need to display the sender's profile again.
    // This excludes stickers and other animated GIFs, video clips, audio clips, etc.
    mod.widgets.CondensedImageMessage = mod.widgets.CondensedMessage {
        body +: {
            content +: {
                message := View {
                    width: Fill, height: Fit,
                    flow: Down,
                    caption_view := View {
                        visible: false,
                        width: Fill, height: Fit,
                        margin: Inset{ bottom: 5.0 }
                        caption := HtmlOrPlaintext {}
                    }
                    image := TextOrImage {
                        image_view +: { image +: {
                            height: (mod.widgets.IMG_MSG_FIT)
                        } }
                    }
                }
                download_section := mod.widgets.MessageDownloadSection {}
                View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }


    // The view used for each state event (non-messages) in a room's timeline.
    // The timestamp, profile picture, and text are all very small.
    mod.widgets.SmallStateEvent = View {
        width: Fill,
        height: Fit,
        flow: Right,
        margin: Inset{ top: 4.0, bottom: 4.0}
        padding: Inset{ top: 1.0, bottom: 1.0, right: 10.0 }
        spacing: 0.0
        cursor: MouseCursor.Default

        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{ left: 7.0, top: 2.0, bottom: 2.0 }
            spacing: 5.0

            left_container := View {
                align: Align{x: 0.5, y: 0}
                width: 70.0,
                height: Fit

                timestamp := Timestamp {
                    margin: Inset{top: 3}
                }
            }

            avatar := Avatar {
                width: 19.,
                height: 19.,
                margin: 0

                text_view +: {
                    text +: {
                        draw_text +: {
                            text_style: TITLE_TEXT { font_size: 7.0 }
                        }
                    }
                }
            }

            // Show an invite button only for a `Knocked` room membership change.
            // All other small state events will not show this button.
            invite_user_button := RobrixPositiveIconButton {
                visible: false
                margin: Inset{ top: -1.5, left: 2, right: 2}
                padding: Inset{top: 4, bottom: 4, left: 9, right: 9}
                draw_bg +: {
                    border_size: 0.75
                }
                draw_icon.svg: (ICON_ADD_USER)
                draw_text.text_style: SMALL_STATE_TEXT_STYLE {}
                icon_walk: Walk{width: 15, height: Fit, margin: Inset{right: -4}}
                text: "Invite to Room"
            }

            content := Label {
                width: Fill,
                height: Fit
                flow: Flow.Right{wrap: true},
                margin: Inset{top: 2.5}
                padding: Inset{ top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
                draw_text +: {
                    text_style: SMALL_STATE_TEXT_STYLE {},
                    color: (SMALL_STATE_TEXT_COLOR)
                }
                text: ""
            }

            avatar_row := mod.widgets.AvatarRow {}
        }
    }


    // The view used for each day divider in a room's timeline.
    // The date text is centered between two horizontal lines.
    mod.widgets.DateDivider = View {
        width: Fill,
        height: Fit,
        margin: Inset{top: 7.0, bottom: 7.0}
        flow: Right,
        padding: Inset{left: 7.0, right: 7.0},
        spacing: 0.0,
        align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

        left_line := LineH { }

        date := Label {
            padding: Inset{left: 7.0, right: 7.0}
            draw_text +: {
                text_style: TEXT_SUB {},
                color: (COLOR_DIVIDER_DARK)
            }
            text: "<date>"
        }

        right_line := LineH { }
    }

    // The view used for the divider indicating where the user's last-viewed message is.
    // This is implemented as a DateDivider with a different color and a fixed text label.
    mod.widgets.ReadMarker = mod.widgets.DateDivider {
        left_line := LineH {
            draw_bg.color: (mod.widgets.COLOR_READ_MARKER)
        }

        date := Label {
            draw_text.color: (mod.widgets.COLOR_READ_MARKER)
            text: "New Messages"
        }

        right_line := LineH {
            draw_bg.color: (mod.widgets.COLOR_READ_MARKER)
        }
    }


    // The top space is used to display a loading message while the room is being paginated.
    mod.widgets.TopSpace = SolidView {
        visible: false,
        width: Fill,
        height: Fit,
        align: Align{x: 0.5, y: 0}
        flow: Right,
        show_bg: true,
        draw_bg.color: #xDAF5E5F0, // mostly opaque light green

        label := Label {
            width: Fill,
            height: Fit,
            align: Align{x: 0.5, y: 0.5},
            flow: Right,
            padding: Inset{ top: 10.0, bottom: 7.0, left: 15.0, right: 15.0 }
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
                color: (TIMESTAMP_TEXT_COLOR)
            }
            text: "Loading earlier messages..."
        }
    }

    mod.widgets.Timeline = View {
        width: Fill,
        height: Fill,
        align: Align{x: 0.5, y: 0.0} // center horizontally, align to top vertically
        flow: Overlay,
        new_batch: true

        list := PortalList {
            height: Fill,
            width: Fill
            flow: Down

            auto_tail: true, // set to `true` to lock the view to the last item.
            // only bounce at the end, not the start because that triggers back pagination.
            bounce_at_start: false,
            bounce_at_end: true,
            // Read-receipt logic listens for scroll position changes.
            emit_scroll_actions: true,
            // Prefetch older history shortly before the user actually hits the top.
            reached_start_margin: 2,
            // TODO: enable `reuse_items: true` once Makepad's Html/TextFlow widget
            //   properly resets all internal state during `script_apply(Reload)`.
            //   Currently, stale TextFlow layout state (particularly related to
            //   list items) leaks through when a widget is recycled, causing
            //   excessive whitespace in HTML messages with `<ul>`/`<ol>` lists.

            // Below, we must place all of the possible templates (views) that can be used in the portal list.
            Message := mod.widgets.Message {}
            CondensedMessage := mod.widgets.CondensedMessage {}
            ImageMessage := mod.widgets.ImageMessage {}
            CondensedImageMessage := mod.widgets.CondensedImageMessage {}
            SmallStateEvent := mod.widgets.SmallStateEvent {}
            Empty := mod.widgets.Empty {}
            DateDivider := mod.widgets.DateDivider {}
            ReadMarker := mod.widgets.ReadMarker {}
        }

        // A jump to bottom button (with an unread message badge) that is shown
        // when the timeline is not at the bottom.
        jump_to_bottom_button := JumpToBottomButton { }
    }


    mod.widgets.RoomScreen = #(RoomScreen::register_widget(vm)) {
        width: Fill, height: Fill,
        cursor: MouseCursor.Default,
        flow: Down,
        spacing: 0.0

        room_screen_wrapper := SolidView {
            width: Fill, height: Fill,
            flow: Overlay,

            show_bg: true
            draw_bg.color: (COLOR_PRIMARY_DARKER)

            restore_status_view := RestoreStatusView {}

            // This used to be a KeyboardView wrapper, but now the on-screen keyboard shift
            // is handled by the top-level Window.
            timeline_and_input_bar := View {
                width: Fill, height: Fill,
                flow: Down,

                // First, display the timeline of all messages/events.
                timeline := mod.widgets.Timeline { }

                // Below that, display a typing notice when other users in the room are typing.
                typing_notice := TypingNotice { }

                room_input_bar := RoomInputBar { }
            }

            // Note: here, we're within a View that has an Overlay flow,
            // so the order that we define the below views determines which one is on top.

            // The top space should be displayed as an overlay at the top of the timeline.
            top_space := mod.widgets.TopSpace { }

            // The user profile sliding pane should be displayed on top of other "static" subviews
            // (on top of all other views that are always visible).
            user_profile_sliding_pane := mod.widgets.UserProfileSlidingPane { }

            // The loading pane appears while the user is waiting for something in the room screen
            // to finish loading, e.g., when loading an older replied-to message.
            loading_pane := LoadingPane { }

            // The popup menu for uploading/sending other content to this room,
            // which is controlled by actions from the RoomInputBar.
            room_input_popup_menu := RoomInputPopupMenu { }


            /*
             * TODO: add the action bar back in as a series of floating buttons.
             *
            message_action_bar_popup := PopupNotification {
                align: Align{x: 0.0, y: 0.0}
                content: {
                    height: Fit,
                    width: Fit,
                    show_bg: false,
                    align: Align{
                        x: 0.5,
                        y: 0.5
                    }

                    message_action_bar := MessageActionBar {}
                }
            }
            */
        }
    }
}

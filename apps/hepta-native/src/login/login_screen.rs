use std::{collections::BTreeSet, ops::Not};

use accesskit::{Action as AccessibilityAction, ActionData, ActionRequest, TreeId};
use makepad_widgets::*;
use url::Url;

use crate::sliding_sync::{submit_async_request, LoginByPassword, LoginRequest, MatrixRequest};

use super::login_status_modal::{LoginStatusModalAction, LoginStatusModalWidgetExt};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ICON_EYE_OPEN   = crate_resource("self://resources/icons/eye_open.svg")
    mod.widgets.ICON_EYE_CLOSED = crate_resource("self://resources/icons/eye_closed.svg")

    // Android's GLES path in our pinned Makepad revision cannot upload decoded
    // BGRA PNG textures on every device. Keep the login identity and provider
    // marks texture-free so a failed image upload can never turn them into
    // opaque black rectangles. Provider path geometry is from Simple Icons
    // 14.15.0 (CC0-1.0); the marks remain trademarks of their owners.
    mod.widgets.HeptaBrandMark = RoundedView {
        width: 48,
        height: 48,
        align: Align{x: 0.5, y: 0.5},
        show_bg: true,
        draw_bg +: {
            color: (COLOR_HEPTA_FOCUS)
            border_radius: 13.0
            border_size: 0.75
            border_color: (COLOR_HEPTA_HAIRLINE_STRONG)
        }

        Label {
            width: Fill,
            height: Fill,
            padding: 0,
            align: Align{x: 0.5, y: 0.5},
            draw_text +: {
                color: #FFFFFF
                text_style: theme.font_bold {font_size: 22.0}
            }
            text: "H"
        }
    }

    mod.widgets.SsoButton = RoundedView {
        width: (HEPTA_TOUCH_TARGET),
        height: (HEPTA_TOUCH_TARGET),
        cursor: MouseCursor.Hand,
        visible: true,
        padding: 0,
        margin: 0,
        align: Align{x: 0.5, y: 0.5},
        draw_bg +: {
            border_size: 0.75
            border_radius: (HEPTA_RADIUS_CONTROL)
            border_color: (COLOR_HEPTA_HAIRLINE)
            color: (COLOR_HEPTA_GLASS_STRONG)
        }
    }

    mod.widgets.LoginScreen = set_type_default() do #(LoginScreen::register_widget(vm)) {
        ..mod.widgets.SolidView

        width: Fill, height: Fill,
        align: Align{x: 0.5, y: 0.5}
        show_bg: true,
        draw_bg +: {
            color: COLOR_HEPTA_ENVIRONMENT
        }

        ScrollYView {
            width: Fill, height: Fill,
            flow: Down, // Required for vertical scrolling to work.
            // Top anchoring guarantees that an overflowing card has a stable
            // scroll origin in landscape and while the IME is visible.
            align: Align{x: 0.5, y: 0.0}
            show_bg: true,
            draw_bg.color: (COLOR_HEPTA_ENVIRONMENT)

            // allow the view to be scrollable but hide the actual scroll bar
            scroll_bars: {
                show_scroll_x: false, show_scroll_y: true,
                scroll_bar_y: {
                    bar_size: 0.0
                    min_handle_size: 0.0
                    drag_scrolling: true
                }
            }

            login_panel := RoundedShadowView {
                margin: Inset{
                    top: 12,
                    bottom: (24.0 + mod.widgets.SAFE_INSET_PAD_BOTTOM),
                    left: (8.0 + mod.widgets.SAFE_INSET_PAD_LEFT),
                    right: (8.0 + mod.widgets.SAFE_INSET_PAD_RIGHT)
                }
                padding: 12
                width: Fill{max: 636.}
                height: Fit
                align: Align{x: 0.5, y: 0.0}
                flow: Overlay,

                show_bg: true,
                draw_bg +: {
                    color: (COLOR_HEPTA_GLASS_STRONG)
                    border_radius: (HEPTA_RADIUS_FLOATING)
                    border_size: 0.75
                    border_color: (COLOR_HEPTA_HAIRLINE)
                    shadow_color: (COLOR_HEPTA_SHADOW)
                    shadow_radius: 8.0
                    shadow_offset: vec2(0.0, 2.0)
                }

                login_columns := View {
                    width: Fill
                    height: Fit
                    flow: Flow.Right{wrap: true}
                    align: Align{x: 0.5, y: 0.0}
                    spacing: 12.0

                    credentials_column := View {
                        width: 260
                        height: Fit
                        flow: Down
                        align: Align{x: 0.5, y: 0.0}
                        spacing: 7.0

                        mod.widgets.HeptaBrandMark {}

                        title := Label {
                            width: Fit, height: Fit
                            margin: Inset{ bottom: 2 }
                            padding: 0,
                            draw_text +: {
                                color: (COLOR_TEXT)
                                text_style: theme.font_bold {font_size: 18.0}
                            }
                            text: "Sign in to Hepta"
                        }

                        user_id_input := RobrixTextInput {
                            width: 260, height: Fit{min: FitBound.Abs(48)}
                            flow: Right, // do not wrap
                            padding: 10,
                            empty_text: "User ID"
                            autocapitalize: None,
                            autocorrect: Disabled,
                            content_type: Username,
                        }

                        View {
                            width: 260, height: Fit{min: FitBound.Abs(48)}
                            flow: Overlay
                            align: Align{x: 1.0, y: 0.5}

                            password_input := RobrixTextInput {
                                width: Fill, height: Fit{min: FitBound.Abs(48)}
                                flow: Right, // do not wrap
                                padding: Inset{top: 10, bottom: 10, left: 10, right: 38}
                                empty_text: "Password"
                                is_password: true,
                                autocapitalize: None,
                                autocorrect: Disabled,
                                content_type: Password,
                            }

                            View {
                                width: (HEPTA_TOUCH_TARGET), height: Fill
                                align: Align{x: 0.5, y: 0.5}

                                show_password_button := RobrixNeutralIconButton {
                                    width: (HEPTA_TOUCH_TARGET), height: (HEPTA_TOUCH_TARGET),
                                    align: Align{x: 0.5, y: 0.5}
                                    padding: 5
                                    spacing: 0
                                    margin: 0
                                    draw_bg +: {
                                        color: (COLOR_HEPTA_GLASS_STRONG)
                                    }
                                    draw_icon +: {
                                        svg: (mod.widgets.ICON_EYE_CLOSED),
                                        color: (COLOR_HEPTA_MUTED),
                                    }
                                    icon_walk: Walk{width: 18, height: 18, margin: 0}
                                    text: ""
                                }

                                hide_password_button := RobrixNeutralIconButton {
                                    visible: false,
                                    width: (HEPTA_TOUCH_TARGET), height: (HEPTA_TOUCH_TARGET),
                                    align: Align{x: 0.5, y: 0.5}
                                    padding: 5
                                    spacing: 0
                                    margin: 0
                                    draw_bg +: {
                                        color: (COLOR_HEPTA_GLASS_STRONG)
                                    }
                                    draw_icon +: {
                                        svg: (mod.widgets.ICON_EYE_OPEN),
                                        color: (COLOR_HEPTA_MUTED),
                                    }
                                    icon_walk: Walk{width: 18, height: 18, margin: 0}
                                    text: ""
                                }
                            }
                        }

                        View {
                            width: 260, height: Fit,
                            flow: Down,

                            homeserver_input := RobrixTextInput {
                                width: 260, height: Fit{min: FitBound.Abs(48)},
                                flow: Right, // do not wrap
                                padding: Inset{top: 5, bottom: 5, left: 10, right: 10}
                                empty_text: "matrix.org"
                                autocapitalize: None,
                                autocorrect: Disabled,
                                content_type: Url,
                                input_mode: Url,
                                draw_text +: {
                                    text_style: TITLE_TEXT {font_size: 14.0}
                                }
                            }

                            homeserver_helper := View {
                                width: 260,
                                height: Fit,
                                flow: Right,
                                padding: Inset{top: 3, left: 2, right: 2}
                                spacing: 0.0,
                                align: Align{x: 0.5, y: 0.5}

                                LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }

                                Label {
                                    width: Fit, height: Fit
                                    padding: 0
                                    draw_text +: {
                                        color: (COLOR_HEPTA_MUTED)
                                        text_style: REGULAR_TEXT {font_size: 12}
                                    }
                                    text: "Homeserver URL (optional)"
                                }

                                LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }
                            }
                        }

                        login_button := RobrixIconButton {
                            width: 260,
                            height: (HEPTA_TOUCH_TARGET)
                            padding: 10
                            margin: Inset{top: 2, bottom: 2}
                            align: Align{x: 0.5, y: 0.5}
                            text: "Login"
                        }
                    }

                    alternatives_column := View {
                        width: 275
                        height: Fit
                        flow: Down
                        align: Align{x: 0.5, y: 0.0}
                        spacing: 8.0

                        marketing_title := Label {
                            width: Fill, height: Fit
                            draw_text +: {
                                color: (COLOR_HEPTA_TEXT)
                                text_style: theme.font_bold {font_size: 18.0}
                            }
                            text: "Calm work, clearly reviewed"
                        }

                        marketing_copy := Label {
                            width: Fill, height: Fit
                            draw_text +: {
                                color: (COLOR_HEPTA_MUTED)
                                text_style: REGULAR_TEXT {font_size: 12.0}
                            }
                            text: "Rooms, evidence, and reviewed work in one calm surface."
                        }

                        privacy_panel := RoundedView {
                            width: 275
                            height: Fit
                            padding: 12
                            flow: Down
                            spacing: 5
                            show_bg: true
                            draw_bg +: {
                                color: (COLOR_HEPTA_CONTENT)
                                border_radius: (HEPTA_RADIUS_PANEL)
                                border_size: 0.75
                                border_color: (COLOR_HEPTA_HAIRLINE)
                            }

                            Label {
                                width: Fit, height: Fit
                                draw_text +: {
                                    color: (COLOR_HEPTA_TEXT)
                                    text_style: theme.font_bold {font_size: 13.0}
                                }
                                text: "Private by default"
                            }

                            Label {
                                width: Fill, height: Fit
                                draw_text +: {
                                    color: (COLOR_HEPTA_MUTED)
                                    text_style: REGULAR_TEXT {font_size: 12.0}
                                }
                                text: "Credentials stay on this device. Live access starts after server verification."
                            }
                        }

                        marketing_separator := LineH {
                            width: 275
                            draw_bg.color: (COLOR_HEPTA_HAIRLINE)
                        }

                        Label {
                            width: Fit, height: Fit
                            padding: 0,
                            draw_text +: {
                                color: (COLOR_TEXT)
                                text_style: TITLE_TEXT {font_size: 12.0}
                            }
                            text: "Continue with SSO:"
                        }

                        sso_view := View {
                            width: 275, height: (HEPTA_TOUCH_TARGET),
                            flow: Right,
                            spacing: 2.0,
                            align: Align{x: 0.5, y: 0.5}

                            apple_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "M12.152 6.896c-.948 0-2.415-1.078-3.96-1.04-2.04.027-3.91 1.183-4.961 3.014-2.117 3.675-.546 9.103 1.519 12.09 1.013 1.454 2.208 3.09 3.792 3.039 1.52-.065 2.09-.987 3.935-.987 1.831 0 2.35.987 3.96.948 1.637-.026 2.676-1.48 3.676-2.948 1.156-1.688 1.636-3.325 1.662-3.415-.039-.013-3.182-1.221-3.22-4.857-.026-3.04 2.48-4.494 2.597-4.559-1.429-2.09-3.623-2.324-4.39-2.376-2-.156-3.675 1.09-4.61 1.09zM15.53 3.83c.843-1.012 1.4-2.427 1.245-3.83-1.207.052-2.662.805-3.532 1.818-.78.896-1.454 2.338-1.273 3.714 1.338.104 2.715-.688 3.559-1.701"
                                    }
                                }
                            }
                            facebook_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "M9.101 23.691v-7.98H6.627v-3.667h2.474v-1.58c0-4.085 1.848-5.978 5.858-5.978.401 0 .955.042 1.468.103.395.047.776.112 1.141.195v3.325c-.219-.03-.437-.042-.653-.036-.25-.006-.494-.009-.733-.009-.707 0-1.259.096-1.675.309-.29.147-.516.354-.679.622-.258.42-.374.995-.374 1.752v1.297h3.919l-.386 2.103-.287 1.564h-3.246v8.245C19.396 23.238 24 18.179 24 12.044c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.628 3.874 10.35 9.101 11.647Z"
                                    }
                                }
                            }
                            github_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61-.546-1.385-1.335-1.755-1.335-1.755-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
                                    }
                                }
                            }
                            gitlab_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "m23.6004 9.5927-.0337-.0862L20.3.9814a.851.851 0 0 0-.3362-.405.8748.8748 0 0 0-.9997.0539.8748.8748 0 0 0-.29.4399l-2.2055 6.748H7.5375l-2.2057-6.748a.8573.8573 0 0 0-.29-.4412.8748.8748 0 0 0-.9997-.0537.8585.8585 0 0 0-.3362.4049L.4332 9.5015l-.0325.0862a6.0657 6.0657 0 0 0 2.0119 7.0105l.0113.0087.03.0213 4.976 3.7264 2.462 1.8633 1.4995 1.1321a1.0085 1.0085 0 0 0 1.2197 0l1.4995-1.1321 2.4619-1.8633 5.006-3.7489.0125-.01a6.0682 6.0682 0 0 0 2.0094-7.003z"
                                    }
                                }
                            }
                            google_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"
                                    }
                                }
                            }
                            twitter_button := mod.widgets.SsoButton {
                                mark := Vector {
                                    width: 22, height: 22
                                    viewbox: vec4(0, 0, 24, 24)
                                    Path {
                                        fill: (COLOR_HEPTA_TEXT)
                                        d: "M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z"
                                    }
                                }
                            }
                        }

                        View {
                            width: 275,
                            height: Fit,
                            flow: Right,
                            spacing: 0.0,
                            align: Align{x: 0.5, y: 0.5}

                            LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }

                            Label {
                                width: Fit, height: Fit
                                padding: Inset{left: 1, right: 1, top: 0, bottom: 0}
                                draw_text +: {
                                    color: (COLOR_HEPTA_MUTED)
                                    text_style: REGULAR_TEXT {}
                                }
                                text: "Don't have an account?"
                            }

                            LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }
                        }

                        signup_button := RobrixIconButton {
                            width: Fit, height: (HEPTA_TOUCH_TARGET)
                            padding: Inset{left: 15, right: 15, top: 10, bottom: 10}
                            margin: 0
                            align: Align{x: 0.5, y: 0.5}
                            text: "Sign up here"
                        }
                    }
                }

                // The modal that pops up to display login status messages,
                // such as when the user is logging in or when there is an error.
                login_status_modal := Modal {
                    can_dismiss: false,
                    content := mod.widgets.LoginStatusModal {}
                }
            }
        }
    }
}

static MATRIX_SIGN_UP_URL: &str =
    "https://matrix.org/docs/chat_basics/matrix-for-im/#creating-a-matrix-account";

#[derive(Script, ScriptHook, Widget)]
pub struct LoginScreen {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    /// Whether the password field is currently showing plaintext.
    #[rust]
    password_visible: bool,
    /// Boolean to indicate if the SSO login process is still in flight
    #[rust]
    sso_pending: bool,
    /// The URL to redirect to after logging in with SSO.
    #[rust]
    sso_redirect_url: Option<String>,
    /// Mirrors the modal button's product state for the semantic tree.
    #[rust]
    modal_button_enabled: bool,
    /// True while the platform software keyboard is occluding the login view.
    #[rust]
    ime_visible: bool,
    /// Compact mode keeps the credential path primary on short mobile windows.
    #[rust]
    compact_layout: bool,
    /// Last logical window size, used to retain compact landscape state after IME hide.
    #[rust]
    window_size: Vec2d,
}

impl LoginScreen {
    fn should_use_compact_layout(&self) -> bool {
        if !cfg!(any(target_os = "android", target_os = "ios")) {
            return false;
        }
        let short_window = self.window_size.y > 0.0 && self.window_size.y <= 520.0;
        let landscape = self.window_size.x > 0.0
            && self.window_size.y > 0.0
            && self.window_size.x > self.window_size.y;
        self.ime_visible || short_window || landscape
    }

    fn apply_responsive_layout(&mut self, cx: &mut Cx, force: bool) {
        let compact = self.should_use_compact_layout();
        if !force && compact == self.compact_layout {
            return;
        }
        self.compact_layout = compact;

        for path in [
            ids!(marketing_title),
            ids!(marketing_copy),
            ids!(privacy_panel),
            ids!(marketing_separator),
        ] {
            self.view.widget(cx, path).set_visible(cx, !compact);
        }
        self.view
            .widget(cx, ids!(homeserver_helper))
            .set_visible(cx, !compact);

        let insets = cx.display_context.safe_area_insets;
        let edge = if compact { 6.0 } else { 8.0 };
        let panel_margin = Inset {
            // Short landscape and IME layouts need the credential action to
            // clear the bottom edge without shrinking any 48pt control.
            top: if compact { 4.0 } else { 12.0 },
            bottom: (if compact { 28.0 } else { 24.0 }) + insets.bottom,
            left: edge + insets.left,
            right: edge + insets.right,
        };
        let pad_x = if compact { 10.0 } else { 12.0 };
        let pad_y = if compact { 6.0 } else { 12.0 };
        let panel_padding = Inset {
            top: pad_y,
            bottom: pad_y,
            left: pad_x,
            right: pad_x,
        };
        let column_spacing = if compact { 8.0 } else { 12.0 };
        let credentials_spacing = if compact { 3.0 } else { 7.0 };
        let alternatives_spacing = if compact { 6.0 } else { 8.0 };
        let mut panel = self.view.view(cx, ids!(login_panel));
        let mut columns = self.view.view(cx, ids!(login_columns));
        let mut credentials = self.view.view(cx, ids!(credentials_column));
        let mut alternatives = self.view.view(cx, ids!(alternatives_column));
        script_apply_eval!(cx, panel, {
            margin: #(panel_margin)
            padding: #(panel_padding)
        });
        script_apply_eval!(cx, columns, { spacing: #(column_spacing) });
        script_apply_eval!(cx, credentials, { spacing: #(credentials_spacing) });
        script_apply_eval!(cx, alternatives, { spacing: #(alternatives_spacing) });
        self.redraw(cx);
    }
}

impl Widget for LoginScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        match event {
            Event::VirtualKeyboard(VirtualKeyboardEvent::WillShow { height, .. })
            | Event::VirtualKeyboard(VirtualKeyboardEvent::DidShow { height, .. }) => {
                self.ime_visible = *height > 0.0;
                self.apply_responsive_layout(cx, false);
            }
            Event::VirtualKeyboard(VirtualKeyboardEvent::DidHide { .. }) => {
                self.ime_visible = false;
                self.apply_responsive_layout(cx, false);
            }
            Event::WindowGeomChange(change) => {
                self.window_size = change.new_geom.inner_size;
                self.apply_responsive_layout(cx, true);
            }
            _ => {}
        }
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let step = self.view.draw_walk(cx, scope, walk);
        if step.is_done() {
            crate::accessibility::publish_login_tree(
                cx,
                &self.view,
                self.password_visible,
                self.sso_pending,
                self.modal_button_enabled,
            );
        }
        step
    }
}

impl MatchEvent for LoginScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // The Window widget republishes resize/rotation as a WindowAction.
        // Mobile descendants do not consistently receive the raw platform
        // WindowGeomChange event, so consume the widget action as well.
        for action in actions {
            if let WindowAction::WindowGeomChange(change) = action.as_widget_action().cast() {
                self.window_size = change.new_geom.inner_size;
                self.apply_responsive_layout(cx, true);
                break;
            }
        }

        let login_button = self.view.button(cx, ids!(login_button));
        let signup_button = self.view.button(cx, ids!(signup_button));
        let user_id_input = self.view.text_input(cx, ids!(user_id_input));
        let password_input = self.view.text_input(cx, ids!(password_input));
        let homeserver_input = self.view.text_input(cx, ids!(homeserver_input));

        let login_status_modal = self.view.modal(cx, ids!(login_status_modal));
        let login_status_modal_content = self
            .view
            .login_status_modal(cx, ids!(login_status_modal.content));

        // Handle toggling password visibility
        let show_pw_button = self.view.button(cx, ids!(show_password_button));
        let hide_pw_button = self.view.button(cx, ids!(hide_password_button));
        let mut accessibility_clicks = BTreeSet::new();
        for request in actions
            .iter()
            .filter_map(|action| action.downcast_ref::<ActionRequest>())
            .filter(|request| request.target_tree == TreeId::ROOT)
        {
            let target = u64::from(request.target_node);
            match request.action {
                AccessibilityAction::Focus => match target {
                    3 => user_id_input.set_key_focus(cx),
                    4 => password_input.set_key_focus(cx),
                    5 => homeserver_input.set_key_focus(cx),
                    6 => {
                        if self.password_visible {
                            hide_pw_button.set_key_focus(cx);
                        } else {
                            show_pw_button.set_key_focus(cx);
                        }
                    }
                    7 => login_button.set_key_focus(cx),
                    8..=13 => {
                        let button_id = match target {
                            8 => ids!(apple_button),
                            9 => ids!(facebook_button),
                            10 => ids!(github_button),
                            11 => ids!(gitlab_button),
                            12 => ids!(google_button),
                            _ => ids!(twitter_button),
                        };
                        self.view.view(cx, button_id).set_key_focus(cx);
                    }
                    14 => signup_button.set_key_focus(cx),
                    103 if self.modal_button_enabled => {
                        login_status_modal_content.button_ref(cx).set_key_focus(cx)
                    }
                    _ => {}
                },
                AccessibilityAction::SetValue => {
                    let Some(ActionData::Value(value)) = request.data.as_ref() else {
                        continue;
                    };
                    match target {
                        3 => user_id_input.set_text(cx, value),
                        4 => password_input.set_text(cx, value),
                        5 => homeserver_input.set_text(cx, value),
                        _ => {}
                    }
                }
                AccessibilityAction::Click => {
                    if matches!(target, 6..=14) || (target == 103 && self.modal_button_enabled) {
                        accessibility_clicks.insert(target);
                    }
                }
                _ => {}
            }
        }

        if show_pw_button.clicked(actions)
            || hide_pw_button.clicked(actions)
            || accessibility_clicks.contains(&6)
        {
            self.password_visible = !self.password_visible;
            password_input.toggle_is_password(cx);
            show_pw_button.set_visible(cx, !self.password_visible);
            hide_pw_button.set_visible(cx, self.password_visible);
            password_input.set_key_focus(cx);
            self.redraw(cx);
        }

        if signup_button.clicked(actions) || accessibility_clicks.contains(&14) {
            log!("Opening URL \"{}\"", MATRIX_SIGN_UP_URL);
            let _ = robius_open::Uri::new(MATRIX_SIGN_UP_URL).open();
        }

        if login_button.clicked(actions)
            || accessibility_clicks.contains(&7)
            || user_id_input.returned(actions).is_some()
            || password_input.returned(actions).is_some()
            || homeserver_input.returned(actions).is_some()
        {
            let user_id = user_id_input.text();
            let password = password_input.text();
            let homeserver = homeserver_input.text();
            if user_id.is_empty() {
                login_status_modal_content.set_title(cx, "Missing User ID");
                login_status_modal_content.set_status(cx, "Please enter a valid User ID.");
                login_status_modal_content
                    .button_ref(cx)
                    .set_text(cx, "Okay");
            } else if password.is_empty() {
                login_status_modal_content.set_title(cx, "Missing Password");
                login_status_modal_content.set_status(cx, "Please enter a valid password.");
                login_status_modal_content
                    .button_ref(cx)
                    .set_text(cx, "Okay");
            } else {
                login_status_modal_content.set_title(cx, "Logging in...");
                login_status_modal_content.set_status(cx, "Waiting for a login response...");
                login_status_modal_content
                    .button_ref(cx)
                    .set_text(cx, "Cancel");
                submit_async_request(MatrixRequest::Login(LoginRequest::LoginByPassword(
                    LoginByPassword {
                        user_id,
                        password,
                        homeserver: homeserver.is_empty().not().then_some(homeserver),
                    },
                )));
            }
            self.modal_button_enabled = true;
            login_status_modal_content
                .button_ref(cx)
                .set_enabled(cx, true);
            login_status_modal.open(cx);
            self.redraw(cx);
        }

        let provider_brands = ["apple", "facebook", "github", "gitlab", "google", "twitter"];
        let button_set: &[&[LiveId]] = ids_array!(
            apple_button,
            facebook_button,
            github_button,
            gitlab_button,
            google_button,
            twitter_button
        );
        for action in actions {
            if let LoginStatusModalAction::Close = action.as_widget_action().cast() {
                login_status_modal.close(cx);
            }

            // Handle login-related actions received from background async tasks.
            match action.downcast_ref() {
                Some(LoginAction::CliAutoLogin {
                    user_id,
                    homeserver,
                }) => {
                    user_id_input.set_text(cx, user_id);
                    password_input.set_text(cx, "");
                    homeserver_input.set_text(cx, homeserver.as_deref().unwrap_or_default());
                    login_status_modal_content.set_title(cx, "Logging in via CLI...");
                    login_status_modal_content
                        .set_status(cx, &format!("Auto-logging in as user {user_id}..."));
                    let login_status_modal_button = login_status_modal_content.button_ref(cx);
                    login_status_modal_button.set_text(cx, "Cancel");
                    login_status_modal_button.set_enabled(cx, false); // Login cancel not yet supported
                    self.modal_button_enabled = false;
                    login_status_modal.open(cx);
                }
                Some(LoginAction::Status { title, status }) => {
                    login_status_modal_content.set_title(cx, title);
                    login_status_modal_content.set_status(cx, status);
                    let login_status_modal_button = login_status_modal_content.button_ref(cx);
                    login_status_modal_button.set_text(cx, "Cancel");
                    login_status_modal_button.set_enabled(cx, true);
                    self.modal_button_enabled = true;
                    login_status_modal.open(cx);
                    self.redraw(cx);
                }
                Some(LoginAction::LoginSuccess) => {
                    // The main `App` component handles showing the main screen
                    // and hiding the login screen & login status modal.
                    user_id_input.set_text(cx, "");
                    password_input.set_text(cx, "");
                    homeserver_input.set_text(cx, "");
                    login_status_modal.close(cx);
                    self.redraw(cx);
                }
                Some(LoginAction::LoginFailure(error)) => {
                    login_status_modal_content.set_title(cx, "Login Failed.");
                    login_status_modal_content.set_status(cx, error);
                    let login_status_modal_button = login_status_modal_content.button_ref(cx);
                    login_status_modal_button.set_text(cx, "Okay");
                    login_status_modal_button.set_enabled(cx, true);
                    self.modal_button_enabled = true;
                    login_status_modal.open(cx);
                    self.redraw(cx);
                }
                Some(LoginAction::SsoPending(pending)) => {
                    let cursor = if *pending {
                        MouseCursor::NotAllowed
                    } else {
                        MouseCursor::Hand
                    };
                    for view_ref in self.view_set(cx, button_set).iter() {
                        let Some(mut view_mut) = view_ref.borrow_mut() else {
                            continue;
                        };
                        view_mut.cursor = Some(cursor);
                    }
                    self.sso_pending = *pending;
                    self.redraw(cx);
                }
                Some(LoginAction::SsoSetRedirectUrl(url)) => {
                    self.sso_redirect_url = Some(url.to_string());
                }
                _ => {}
            }
        }

        if accessibility_clicks.contains(&103) {
            login_status_modal.close(cx);
        }

        // If the Login SSO screen's "cancel" button was clicked, send a http request to gracefully shutdown the SSO server
        if let Some(sso_redirect_url) = &self.sso_redirect_url {
            let login_status_modal_button = login_status_modal_content.button_ref(cx);
            if login_status_modal_button.clicked(actions) || accessibility_clicks.contains(&103) {
                let request_id = id!(SSO_CANCEL_BUTTON);
                let request = HttpRequest::new(
                    format!("{}/?login_token=", sso_redirect_url),
                    HttpMethod::GET,
                );
                cx.http_request(request_id, request);
                self.sso_redirect_url = None;
            }
        }

        // On iOS there's no redirect server, so the cancel button dismisses
        // the auth sheet instead. Its completion handler takes the normal
        // SSO failure path, which resets state for the next attempt.
        #[cfg(target_os = "ios")]
        if self.sso_pending {
            let login_status_modal_button = login_status_modal_content.button_ref(cx);
            if login_status_modal_button.clicked(actions) || accessibility_clicks.contains(&103) {
                crate::sliding_sync::cancel_active_sso_auth_session();
            }
        }

        // Handle any of the SSO login buttons being clicked
        for (index, (view_ref, brand)) in self
            .view_set(cx, button_set)
            .iter()
            .zip(&provider_brands)
            .enumerate()
        {
            if (view_ref.finger_up(actions).is_some()
                || accessibility_clicks.contains(&(index as u64 + 8)))
                && !self.sso_pending
            {
                submit_async_request(MatrixRequest::SpawnSSOServer {
                    identity_provider_id: format!("oidc-{}", brand),
                    brand: brand.to_string(),
                    homeserver_url: homeserver_input.text(),
                });
            }
        }
    }
}

/// Actions sent to or from the login screen.
#[derive(Clone, Default, Debug)]
pub enum LoginAction {
    /// A positive response from the backend Matrix task to the login screen.
    LoginSuccess,
    /// A negative response from the backend Matrix task to the login screen.
    LoginFailure(String),
    /// A login-related status message to display to the user.
    Status { title: String, status: String },
    /// The given login info was specified on the command line (CLI),
    /// and the login process is underway.
    CliAutoLogin {
        user_id: String,
        homeserver: Option<String>,
    },
    /// An acknowledgment that is sent from the backend Matrix task to the login screen
    /// informing it that the SSO login process is either still in flight (`true`) or has finished (`false`).
    ///
    /// Note that an inner value of `false` does *not* imply that the login request has
    /// successfully finished.
    /// The login screen can use this to prevent the user from submitting
    /// additional SSO login requests while a previous request is in flight.
    SsoPending(bool),
    /// Set the SSO redirect URL in the LoginScreen.
    ///
    /// When an SSO-based login is pendng, pressing the cancel button will send
    /// an HTTP request to this SSO server URL to gracefully shut it down.
    SsoSetRedirectUrl(Url),
    #[default]
    None,
}

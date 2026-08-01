use std::ops::Not;

use makepad_widgets::*;
use url::Url;

use crate::sliding_sync::{submit_async_request, LoginByPassword, LoginRequest, MatrixRequest};

use super::login_status_modal::{LoginStatusModalAction, LoginStatusModalWidgetExt};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.IMG_APP_LOGO = crate_resource("self://resources/icon_512.png")
    mod.widgets.ICON_EYE_OPEN   = crate_resource("self://resources/icons/eye_open.svg")
    mod.widgets.ICON_EYE_CLOSED = crate_resource("self://resources/icons/eye_closed.svg")

    mod.widgets.SsoButton = RoundedView {
        width: 44,
        height: 44,
        cursor: MouseCursor.Hand,
        visible: true,
        padding: 7,
        margin: Inset{ left: 20, right: 20, top: 5, bottom: 5}
        draw_bg +: {
            border_size: 1.0
            border_radius: (HEPTA_RADIUS_CONTROL)
            border_color: (COLOR_HEPTA_HAIRLINE)
            color: (COLOR_HEPTA_GLASS_STRONG)
        }
    }

    mod.widgets.SsoImage = Image {
        width: 30, height: 30,
        draw_bg +: {
            mask: instance(0.0)
            pixel: fn() {
                let color = mix(self.get_color(), #3, self.async_load)
                let gray = dot(color.rgb, vec3(0.299, 0.587, 0.114))
                let grayed = mix(color, vec4(gray, gray, gray, color.a), self.mask)
                return Pal.premul(vec4(grayed.xyz, grayed.w * self.opacity))
            }
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
            align: Align{x: 0.5, y: 0.5}
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

            RoundedShadowView {
                margin: Inset{top: 18, bottom: 18, left: 4, right: 4}
                padding: 16
                width: Fit
                height: Fit
                align: Align{x: 0.5, y: 0.5}
                flow: Overlay,

                show_bg: true,
                draw_bg +: {
                    color: (COLOR_HEPTA_GLASS_STRONG)
                    border_radius: (HEPTA_RADIUS_FLOATING)
                    border_size: 1.0
                    border_color: (COLOR_HEPTA_HAIRLINE)
                    shadow_color: (COLOR_HEPTA_SHADOW)
                    shadow_radius: 12.0
                    shadow_offset: vec2(0.0, 4.0)
                }

                View {
                    width: 275
                    height: Fit
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    spacing: 10.0

                    logo_image := Image {
                        fit: ImageFit.Smallest,
                        width: 56
                        src: (mod.widgets.IMG_APP_LOGO),
                    }

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
                        width: 275, height: Fit{min: FitBound.Abs(44)}
                        flow: Right, // do not wrap
                        padding: 10,
                        empty_text: "User ID"
                        autocapitalize: None,
                        autocorrect: Disabled,
                        content_type: Username,
                    }

                    View {
                        width: 275, height: Fit{min: FitBound.Abs(44)}
                        flow: Overlay
                        align: Align{x: 1.0, y: 0.5}

                        password_input := RobrixTextInput {
                            width: Fill, height: Fit{min: FitBound.Abs(44)}
                            flow: Right, // do not wrap
                            padding: Inset{top: 10, bottom: 10, left: 10, right: 38}
                            empty_text: "Password"
                            is_password: true,
                            autocapitalize: None,
                            autocorrect: Disabled,
                            content_type: Password,
                        }

                        View {
                            width: 44, height: Fill
                            align: Align{x: 0.5, y: 0.5}

                            show_password_button := RobrixNeutralIconButton {
                                width: 44, height: 44,
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
                                align: Align{x: 0.5, y: 0.5}
                                width: 44, height: 44,
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
                        width: 275, height: Fit,
                        flow: Down,

                        homeserver_input := RobrixTextInput {
                            width: 275, height: Fit{min: FitBound.Abs(44)},
                            flow: Right, // do not wrap
                            padding: Inset{top: 5, bottom: 5, left: 10, right: 10}
                            empty_text: "matrix.org"
                            autocapitalize: None,
                            autocorrect: Disabled,
                            content_type: Url,
                            input_mode: Url,
                            draw_text +: {
                                text_style: TITLE_TEXT {font_size: 10.0}
                            }
                        }

                        View {
                            width: 275,
                            height: Fit,
                            flow: Right,
                            padding: Inset{top: 3, left: 2, right: 2}
                            spacing: 0.0,
                            align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

                            LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }

                            Label {
                                width: Fit, height: Fit
                                padding: 0
                                draw_text +: {
                                    color: (COLOR_HEPTA_MUTED)
                                    text_style: REGULAR_TEXT {font_size: 10}
                                }
                                text: "Homeserver URL (optional)"
                            }

                            LineH { draw_bg.color: (COLOR_HEPTA_HAIRLINE) }
                        }
                    }
                    

                    login_button := RobrixIconButton {
                        width: 275,
                        height: 44
                        padding: 10
                        margin: Inset{top: 4, bottom: 6}
                        align: Align{x: 0.5, y: 0.5}
                        text: "Login"
                    }

                    LineH {
                        width: 275
                        margin: Inset{bottom: -5}
                        draw_bg.color: (COLOR_HEPTA_HAIRLINE)
                    }

                    Label {
                        width: Fit, height: Fit
                        padding: 0,
                        draw_text +: {
                            color: (COLOR_TEXT)
                            text_style: TITLE_TEXT {font_size: 11.0}
                        }
                        text: "Or, login with an SSO provider:"
                    }

                    sso_view := View {
                        width: 275, height: Fit,
                        margin: Inset{left: 10, right: 10}
                        flow: Flow.Right{wrap: true},
                        apple_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/apple.png")
                            }
                        }
                        facebook_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/facebook.png")
                            }
                        }
                        github_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/github.png")
                            }
                        }
                        gitlab_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/gitlab.png")
                            }
                        }
                        google_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/google.png")
                            }
                        }
                        twitter_button := mod.widgets.SsoButton {
                            image := mod.widgets.SsoImage {
                                src: crate_resource("self://resources/img/x.png")
                            }
                        }
                    }

                    View {
                        width: 275,
                        height: Fit,
                        flow: Right,
                        // padding: 3,
                        spacing: 0.0,
                        align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

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
                        width: Fit, height: Fit
                        padding: Inset{left: 15, right: 15, top: 10, bottom: 10}
                        margin: Inset{bottom: 5}
                        align: Align{x: 0.5, y: 0.5}
                        text: "Sign up here"
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

static MATRIX_SIGN_UP_URL: &str = "https://matrix.org/docs/chat_basics/matrix-for-im/#creating-a-matrix-account";

#[derive(Script, ScriptHook, Widget)]
pub struct LoginScreen {
    #[source] source: ScriptObjectRef,
    #[deref] view: View,
    /// Whether the password field is currently showing plaintext.
    #[rust] password_visible: bool,
    /// Boolean to indicate if the SSO login process is still in flight
    #[rust] sso_pending: bool,
    /// The URL to redirect to after logging in with SSO.
    #[rust] sso_redirect_url: Option<String>,
}


impl Widget for LoginScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for LoginScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let login_button = self.view.button(cx, ids!(login_button));
        let signup_button = self.view.button(cx, ids!(signup_button));
        let user_id_input = self.view.text_input(cx, ids!(user_id_input));
        let password_input = self.view.text_input(cx, ids!(password_input));
        let homeserver_input = self.view.text_input(cx, ids!(homeserver_input));

        let login_status_modal = self.view.modal(cx, ids!(login_status_modal));
        let login_status_modal_content = self.view.login_status_modal(cx, ids!(login_status_modal.content));

        // Handle toggling password visibility
        let show_pw_button = self.view.button(cx, ids!(show_password_button));
        let hide_pw_button = self.view.button(cx, ids!(hide_password_button));
        if show_pw_button.clicked(actions) || hide_pw_button.clicked(actions) {
            self.password_visible = !self.password_visible;
            password_input.toggle_is_password(cx);
            show_pw_button.set_visible(cx, !self.password_visible);
            hide_pw_button.set_visible(cx, self.password_visible);
            password_input.set_key_focus(cx);
            self.redraw(cx);
        }

        if signup_button.clicked(actions) {
            log!("Opening URL \"{}\"", MATRIX_SIGN_UP_URL);
            let _ = robius_open::Uri::new(MATRIX_SIGN_UP_URL).open();
        }

        if login_button.clicked(actions)
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
                login_status_modal_content.button_ref(cx).set_text(cx, "Okay");
            } else if password.is_empty() {
                login_status_modal_content.set_title(cx, "Missing Password");
                login_status_modal_content.set_status(cx, "Please enter a valid password.");
                login_status_modal_content.button_ref(cx).set_text(cx, "Okay");
            } else {
                login_status_modal_content.set_title(cx, "Logging in...");
                login_status_modal_content.set_status(cx, "Waiting for a login response...");
                login_status_modal_content.button_ref(cx).set_text(cx, "Cancel");
                submit_async_request(MatrixRequest::Login(LoginRequest::LoginByPassword(LoginByPassword {
                    user_id,
                    password,
                    homeserver: homeserver.is_empty().not().then_some(homeserver),
                })));
            }
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
                Some(LoginAction::CliAutoLogin { user_id, homeserver }) => {
                    user_id_input.set_text(cx, user_id);
                    password_input.set_text(cx, "");
                    homeserver_input.set_text(cx, homeserver.as_deref().unwrap_or_default());
                    login_status_modal_content.set_title(cx, "Logging in via CLI...");
                    login_status_modal_content.set_status(
                        cx,
                        &format!("Auto-logging in as user {user_id}...")
                    );
                    let login_status_modal_button = login_status_modal_content.button_ref(cx);
                    login_status_modal_button.set_text(cx, "Cancel");
                    login_status_modal_button.set_enabled(cx, false); // Login cancel not yet supported
                    login_status_modal.open(cx);
                }
                Some(LoginAction::Status { title, status }) => {
                    login_status_modal_content.set_title(cx, title);
                    login_status_modal_content.set_status(cx, status);
                    let login_status_modal_button = login_status_modal_content.button_ref(cx);
                    login_status_modal_button.set_text(cx, "Cancel");
                    login_status_modal_button.set_enabled(cx, true);
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
                    login_status_modal.open(cx);
                    self.redraw(cx);
                }
                Some(LoginAction::SsoPending(pending)) => {
                    let mask = if *pending { 1.0 } else { 0.0 };
                    let cursor = if *pending { MouseCursor::NotAllowed } else { MouseCursor::Hand };
                    for view_ref in self.view_set(cx, button_set).iter() {
                        let Some(mut view_mut) = view_ref.borrow_mut() else { continue };
                        let mut image = view_mut.image(cx, ids!(image));
                        script_apply_eval!(cx, image, {
                            draw_bg.mask: #(mask)
                        });
                        view_mut.cursor = Some(cursor);
                    }
                    self.sso_pending = *pending;
                    self.redraw(cx);
                }
                Some(LoginAction::SsoSetRedirectUrl(url)) => {
                    self.sso_redirect_url = Some(url.to_string());
                }
                _ => { }
            }
        }

        // If the Login SSO screen's "cancel" button was clicked, send a http request to gracefully shutdown the SSO server
        if let Some(sso_redirect_url) = &self.sso_redirect_url {
            let login_status_modal_button = login_status_modal_content.button_ref(cx);
            if login_status_modal_button.clicked(actions) {
                let request_id = id!(SSO_CANCEL_BUTTON);
                let request = HttpRequest::new(format!("{}/?login_token=",sso_redirect_url), HttpMethod::GET);
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
            if login_status_modal_button.clicked(actions) {
                crate::sliding_sync::cancel_active_sso_auth_session();
            }
        }

        // Handle any of the SSO login buttons being clicked
        for (view_ref, brand) in self.view_set(cx, button_set).iter().zip(&provider_brands) {
            if view_ref.finger_up(actions).is_some() && !self.sso_pending {
                submit_async_request(MatrixRequest::SpawnSSOServer{
                    identity_provider_id: format!("oidc-{}",brand),
                    brand: brand.to_string(),
                    homeserver_url: homeserver_input.text()
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
    Status {
        title: String,
        status: String,
    },
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

use makepad_widgets::*;

use crate::{
    app::{AppState, AppStateAction, SelectedRoom},
    home::{
        invite_screen::InviteScreenWidgetExt,
        navigation_tab_bar::{NavigationBarAction, SelectedTab},
        room_screen::RoomScreenWidgetExt,
        rooms_list::RoomsListAction,
        space_lobby::SpaceLobbyScreenWidgetExt,
    },
    settings::{
        app_preferences::{AppPreferencesAction, ViewModeOverride},
        settings_screen::SettingsScreenWidgetRefExt,
    },
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    // Defines the total height of the StackNavigationView's header.
    // This has to be set in multiple places because of how StackNavigation
    // uses an Overlay view internally.
    mod.widgets.STACK_VIEW_HEADER_HEIGHT = 45

    // A reusable base for StackNavigationView children in the mobile layout.
    // Each specific content view (room, invite, space lobby) extends this
    // and places its own screen widget inside the body.
    mod.widgets.RobrixContentView = StackNavigationView {
        width: Fill, height: Fill
        draw_bg.color: (COLOR_TELEGRAM_BG)
        header +: {
            height: (mod.widgets.STACK_VIEW_HEADER_HEIGHT),
            padding: 0
            align: Align{y: 0.5}

            // Below is a shader to draw a shadow under the bottom half of the header
            clip_x: false,
            clip_y: false,
            show_bg: true,
            draw_bg +: {
                color: instance((COLOR_TELEGRAM_PANEL))
                color_dither: uniform(1.0)
                gradient_border_horizontal: uniform(0.0)
                gradient_fill_horizontal: uniform(0.0)
                color_2: instance(vec4(-1))

                border_radius: uniform(4.0)
                border_size: uniform(0.0)
                border_color: instance(#0000)
                border_color_2: instance(vec4(-1))

                shadow_color: instance((COLOR_TELEGRAM_GLASS_SHADOW))
                shadow_radius: uniform(16.0)
                shadow_offset: uniform(vec2(0.0, 0.0))

                rect_size2: varying(vec2(0))
                rect_size3: varying(vec2(0))
                rect_pos2: varying(vec2(0))
                rect_shift: varying(vec2(0))
                sdf_rect_pos: varying(vec2(0))
                sdf_rect_size: varying(vec2(0))

                vertex: fn() {
                    let min_offset = min(self.shadow_offset vec2(0))
                    self.rect_size2 = self.rect_size + 2.0*vec2(self.shadow_radius)
                    self.rect_size3 = self.rect_size2 + abs(self.shadow_offset)
                    self.rect_pos2 = self.rect_pos - vec2(self.shadow_radius) + min_offset
                    self.sdf_rect_size = self.rect_size2 - vec2(self.shadow_radius * 2.0 + self.border_size * 2.0)
                    self.sdf_rect_pos = -min_offset + vec2(self.border_size + self.shadow_radius)
                    self.rect_shift = -min_offset

                    return self.clip_and_transform_vertex(self.rect_pos2 self.rect_size3)
                }

                pixel: fn() {
                    let sdf = Sdf2d.viewport(self.pos * self.rect_size3)

                    let mut fill_color = self.color
                    if self.color_2.x > -0.5 {
                        let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                        let dir = if self.gradient_fill_horizontal > 0.5 self.pos.x else self.pos.y
                        fill_color = mix(self.color self.color_2 dir + dither)
                    }

                    let mut stroke_color = self.border_color
                    if self.border_color_2.x > -0.5 {
                        let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                        let dir = if self.gradient_border_horizontal > 0.5 self.pos.x else self.pos.y
                        stroke_color = mix(self.border_color self.border_color_2 dir + dither)
                    }

                    sdf.box(
                        self.sdf_rect_pos.x
                        self.sdf_rect_pos.y
                        self.sdf_rect_size.x
                        self.sdf_rect_size.y
                        max(1.0 self.border_radius)
                    )
                    if sdf.shape > -1.0 {
                        let m = self.shadow_radius
                        let o = self.shadow_offset + self.rect_shift
                        let v = GaussShadow.rounded_box_shadow(vec2(m) + o self.rect_size2+o self.pos * (self.rect_size3+vec2(m)) self.shadow_radius*0.5 self.border_radius*2.0)
                        // Only draw shadow on the bottom half of the view
                        let pixel_y = self.pos.y * self.rect_size3.y
                        let mid_y = self.sdf_rect_pos.y + self.sdf_rect_size.y * 0.5
                        let bottom_mask = smoothstep(mid_y - m * 0.3 mid_y + m * 0.3 pixel_y)
                        sdf.clear(self.shadow_color * v * bottom_mask)
                    }

                    sdf.fill_keep(fill_color)

                    if self.border_size > 0.0 {
                        sdf.stroke(stroke_color self.border_size)
                    }
                    return sdf.result
                }
            }

            content +: {
                height: (mod.widgets.STACK_VIEW_HEADER_HEIGHT)
                align: Align{y: 0.5}
                padding: Inset{
                    left: (mod.widgets.SAFE_INSET_PAD_LEFT),
                    right: (mod.widgets.SAFE_INSET_PAD_RIGHT),
                }
                button_container +: {
                    padding: 0,
                    margin: 0
                    left_button +: {
                        width: Fit, height: Fit,
                        padding: Inset{left: 20, right: 23, top: 10, bottom: 10}
                        margin: Inset{left: 8, right: 0, top: 0, bottom: 0}
                        draw_icon +: { color: (COLOR_TELEGRAM_BLUE) }
                        icon_walk: Walk{width: 13, height: Fit}
                        spacing: 0
                        text: ""
                    }
                }
                title_container +: {
                    // padding: Inset{top: 8}
                    title +: {
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                        }
                    }
                }
            }
        }
        body +: {
            // The top margin leaves room for the stack nav header.
            // The other padding is for safe inset areas.
            flow: Down
            margin: Inset{top: (mod.widgets.STACK_VIEW_HEADER_HEIGHT)}
            padding: Inset{
                left: (mod.widgets.SAFE_INSET_PAD_LEFT),
                right: (mod.widgets.SAFE_INSET_PAD_RIGHT),
                bottom: (mod.widgets.SAFE_INSET_PAD_BOTTOM),
            }

            mobile_stack_navigation_evidence := Label {
                width: Fill,
                height: Fit,
                margin: Inset{top: 6.0, bottom: 4.0, left: 10.0, right: 10.0}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Mobile stack: room pushes/pops reuse 16 local room views, restore previous selected rooms from UI state, and send no Matrix search, message, room-state, or membership request."
            }
        }
    }

    // A wrapper view around the SpacesBar that lets us show/hide it via animation.
    mod.widgets.SpacesBarWrapper = set_type_default() do #(SpacesBarWrapper::register_widget(vm)) {
        ..mod.widgets.RoundedShadowView

        width: Fill,
        height: (NAVIGATION_TAB_BAR_SIZE)
        margin: Inset{
            left: (4.0 + mod.widgets.SAFE_INSET_PAD_LEFT),
            right: (4.0 + mod.widgets.SAFE_INSET_PAD_RIGHT),
        }
        show_bg: true
        draw_bg +: {
            color: (COLOR_TELEGRAM_PANEL)
            border_radius: 10.0
            border_size: 1.0
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
            shadow_color: (COLOR_TELEGRAM_GLASS_SHADOW)
            shadow_radius: 18.0
            shadow_offset: vec2(1.0, 0.0)
        }

        CachedWidget {
            root_spaces_bar := mod.widgets.SpacesBar {}
        }

        animator: Animator{
            spaces_bar_animator: {
                default: @hide
                show: AnimatorState{
                    redraw: true
                    from: { all: Forward { duration: (mod.widgets.SPACES_BAR_ANIMATION_DURATION_SECS) } }
                    apply: { height: (NAVIGATION_TAB_BAR_SIZE),  draw_bg: { shadow_color: #x00000055 } }
                }
                hide: AnimatorState{
                    redraw: true
                    from: { all: Forward { duration: (mod.widgets.SPACES_BAR_ANIMATION_DURATION_SECS) } }
                    apply: { height: 0,  draw_bg: { shadow_color: (COLOR_TRANSPARENT) } }
                }
            }
        }
    }

    // The home screen widget contains the main content:
    // rooms list, room screens, and the settings screen as an overlay.
    // It adapts to both desktop and mobile layouts.
    mod.widgets.HomeScreen = #(HomeScreen::register_widget(vm)) {
        main_adaptive_view := AdaptiveView {
            // NOTE: within each of these sub views, we used `CachedWidget` wrappers
            //       to ensure that there is only a single global instance of each
            //       of those widgets, which means they maintain their state
            //       across transitions between the Desktop and Mobile variant.
            Desktop := SolidView {
                width: Fill, height: Fill
                flow: Overlay
                align: Align{x: 0.0, y: 0.0}
                padding: 0,
                margin: 0,

                show_bg: true
                draw_bg +: {
                    color: (COLOR_TELEGRAM_BG)
                    color_2: #xEAF7FFFF
                    gradient_fill_horizontal: 0.0
                }

                tempered_glass_backdrop := Image {
                    width: Fill, height: Fill
                    fit: ImageFit.Stretch
                    src: (mod.widgets.IMG_TEMPERED_GLASS_BG)
                    draw_bg +: {
                        // Texture stays environmental; readable content surfaces carry
                        // the stronger glass material.
                        opacity: 0.24
                        pixel: fn() {
                            let color = self.get_color()
                            return Pal.premul(vec4(color.xyz, color.w * self.opacity))
                        }
                    }
                }

                standard_desktop_shell := SolidView {
                    width: Fill, height: Fill
                    flow: Right
                    show_bg: true
                    draw_bg.color: #x070B1200

                    // On the left, show the navigation tab bar vertically.
                    CachedWidget {
                        navigation_tab_bar := mod.widgets.NavigationTabBar {}
                    }

                    // To the right of that, we use the PageFlip widget to show either
                    // the main desktop UI or the settings screen.
                    home_screen_page_flip := PageFlip {
                        width: Fill, height: Fill
                        // We only need bottom and right-side padding,
                        // as the others are handled by the parent widget
                        // or by the navigation bar.
                        padding: Inset{
                            bottom: (mod.widgets.SAFE_INSET_PAD_BOTTOM),
                            right: (mod.widgets.SAFE_INSET_PAD_RIGHT),
                        }

                        lazy_init: true,
                        active_page: @home_page

                        home_page := View {
                            width: Fill, height: Fill
                            flow: Down

                            mod.widgets.MainDesktopUI {}
                        }

                        settings_page := RoundedView {
                            width: Fill, height: Fill
                            // This weird margin is just to make it line up with the home_page content.
                            margin: Inset{top: 3, left: 1, right: 0, bottom: 0}
                            show_bg: true,
                            draw_bg +: {
                                color: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                                border_size: 1.0
                                border_radius: 10.0
                            }

                            CachedWidget {
                                settings_screen := mod.widgets.SettingsScreen {}
                            }
                        }

                        add_room_page := RoundedView {
                            width: Fill, height: Fill
                            // This weird margin is just to make it line up with the home_page content.
                            margin: Inset{top: 3, left: 1, right: 0, bottom: 0}
                            show_bg: true,
                            draw_bg +: {
                                color: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                                border_size: 1.0
                                border_radius: 10.0
                            }

                            CachedWidget {
                                add_room_screen := mod.widgets.AddRoomScreen {}
                            }
                        }
                    }
                }

                fixture_desktop_product_shell := SolidView {
                    visible: false
                    width: Fill, height: Fill
                    flow: Down
                    padding: Inset{top: 14.0, bottom: 14.0, left: 14.0, right: 14.0}
                    show_bg: true
                    draw_bg.color: (COLOR_TELEGRAM_BG)

                    fixture_desktop_product_scroller := ScrollYView {
                        width: Fill,
                        height: Fill,
                        flow: Down,

                        fixture_desktop_cockpit := mod.widgets.HeptaFixtureCockpit {}
                    }
                }
            }

            Mobile := SolidView {
                width: Fill, height: Fill
                flow: Overlay

                show_bg: true
                draw_bg +: {
                    color: (COLOR_TELEGRAM_BG)
                    color_2: #xEAF7FFFF
                    gradient_fill_horizontal: 0.0
                }

                tempered_glass_backdrop := Image {
                    width: Fill, height: Fill
                    fit: ImageFit.Stretch
                    src: (mod.widgets.IMG_TEMPERED_GLASS_BG)
                    draw_bg +: {
                        opacity: 0.20
                        pixel: fn() {
                            let color = self.get_color()
                            return Pal.premul(vec4(color.xyz, color.w * self.opacity))
                        }
                    }
                }

                standard_mobile_shell := SolidView {
                    width: Fill, height: Fill
                    flow: Down
                    show_bg: true
                    draw_bg.color: #x070B1200

                    view_stack := StackNavigation {
                        root_view +: {
                            flow: Down
                            width: Fill, height: Fill

                            // At the top of the root view, we use the PageFlip widget to show either
                            // the main list of rooms or the settings screen.
                            home_screen_page_flip := PageFlip {
                                width: Fill, height: Fill
                                padding: Inset{
                                    left: (mod.widgets.SAFE_INSET_PAD_LEFT),
                                    right: (mod.widgets.SAFE_INSET_PAD_RIGHT),
                                }

                                lazy_init: true,
                                active_page: @home_page

                                home_page := View {
                                    width: Fill, height: Fill
                                    // Note: while the other page views have top padding, we do NOT add that here
                                    // because it is added in the `RoomsSideBar`'s `RoundedShadowView` itself.
                                    flow: Down

                                    mod.widgets.RoomsSideBar {}
                                }

                                settings_page := View {
                                    width: Fill, height: Fill

                                    CachedWidget {
                                        settings_screen := mod.widgets.SettingsScreen {}
                                    }
                                }

                                add_room_page := View {
                                    width: Fill, height: Fill

                                    CachedWidget {
                                        add_room_screen := mod.widgets.AddRoomScreen {}
                                    }
                                }
                            }

                            // Show the SpacesBar right above the navigation tab bar.
                            // We wrap it in the SpacesBarWrapper in order to animate it in or out,
                            // and wrap *that* in a CachedWidget in order to maintain its shown/hidden state
                            // across AdaptiveView transitions between Mobile view mode and Desktop view mode.
                            //
                            // ... Then we wrap *that* in a ... <https://www.youtube.com/watch?v=evUWersr7pc>
                            CachedWidget {
                                spaces_bar_wrapper := mod.widgets.SpacesBarWrapper {}
                            }

                            // At the bottom of the root view, show the navigation tab bar horizontally.
                            CachedWidget {
                                navigation_tab_bar := mod.widgets.NavigationTabBar {}
                            }
                        }

                        // Room views: multiple instances to support deep stacking
                        // (e.g., room -> thread -> room -> thread -> ...).
                        // Each stack depth gets its own dedicated view widget,
                        // avoiding complex state save/restore when views are reused.
                        room_view_0  := mod.widgets.RobrixContentView { body +: { room_screen_0  := mod.widgets.RoomScreen {} } }
                        room_view_1  := mod.widgets.RobrixContentView { body +: { room_screen_1  := mod.widgets.RoomScreen {} } }
                        room_view_2  := mod.widgets.RobrixContentView { body +: { room_screen_2  := mod.widgets.RoomScreen {} } }
                        room_view_3  := mod.widgets.RobrixContentView { body +: { room_screen_3  := mod.widgets.RoomScreen {} } }
                        room_view_4  := mod.widgets.RobrixContentView { body +: { room_screen_4  := mod.widgets.RoomScreen {} } }
                        room_view_5  := mod.widgets.RobrixContentView { body +: { room_screen_5  := mod.widgets.RoomScreen {} } }
                        room_view_6  := mod.widgets.RobrixContentView { body +: { room_screen_6  := mod.widgets.RoomScreen {} } }
                        room_view_7  := mod.widgets.RobrixContentView { body +: { room_screen_7  := mod.widgets.RoomScreen {} } }
                        room_view_8  := mod.widgets.RobrixContentView { body +: { room_screen_8  := mod.widgets.RoomScreen {} } }
                        room_view_9  := mod.widgets.RobrixContentView { body +: { room_screen_9  := mod.widgets.RoomScreen {} } }
                        room_view_10 := mod.widgets.RobrixContentView { body +: { room_screen_10 := mod.widgets.RoomScreen {} } }
                        room_view_11 := mod.widgets.RobrixContentView { body +: { room_screen_11 := mod.widgets.RoomScreen {} } }
                        room_view_12 := mod.widgets.RobrixContentView { body +: { room_screen_12 := mod.widgets.RoomScreen {} } }
                        room_view_13 := mod.widgets.RobrixContentView { body +: { room_screen_13 := mod.widgets.RoomScreen {} } }
                        room_view_14 := mod.widgets.RobrixContentView { body +: { room_screen_14 := mod.widgets.RoomScreen {} } }
                        room_view_15 := mod.widgets.RobrixContentView { body +: { room_screen_15 := mod.widgets.RoomScreen {} } }

                        invite_view := mod.widgets.RobrixContentView {
                            body +: {
                                invite_screen := mod.widgets.InviteScreen {}
                            }
                        }

                        space_lobby_view := mod.widgets.RobrixContentView {
                            body +: {
                                space_lobby_screen := mod.widgets.SpaceLobbyScreen {}
                            }
                        }
                    }
                }

                fixture_mobile_product_shell := SolidView {
                    visible: false
                    width: Fill, height: Fill
                    flow: Down
                    padding: Inset{
                        top: 10.0,
                        bottom: (10.0 + mod.widgets.SAFE_INSET_PAD_BOTTOM),
                        left: (10.0 + mod.widgets.SAFE_INSET_PAD_LEFT),
                        right: (10.0 + mod.widgets.SAFE_INSET_PAD_RIGHT),
                    }
                    show_bg: true
                    draw_bg.color: (COLOR_TELEGRAM_BG)

                    fixture_mobile_product_scroller := ScrollYView {
                        width: Fill,
                        height: Fill,
                        flow: Down,

                        fixture_mobile_cockpit := mod.widgets.HeptaFixtureCockpit {}
                    }
                }
            }
        }
    }
}

const HEPTA_FIXTURE_MOBILE_PRODUCT_SHELL_MAX_WIDTH: f64 = 620.0;

/// A simple wrapper around the SpacesBar that allows us to animate showing or hiding it.
#[derive(Script, Widget, Animator)]
pub struct SpacesBarWrapper {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[apply_default]
    animator: Animator,
}

impl ScriptHook for SpacesBarWrapper {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        scope: &mut Scope,
        _value: ScriptValue,
    ) {
        // When the widget tree is re-applied (e.g. after a preference change),
        // the deref `view` resets its height to the DSL default,
        // which clashes with whatever animator state we were in (shown, hidden).
        // Thus, we re-apply the current animator state to prevent a hidden SpacesBar
        // from briefly becoming shown before being hidden again.
        // Note that we can't just call `animator_cut` cuz that uses the script VM
        // which is unavailable from this `on_after_apply`
        if !apply.is_script_reapply() {
            return;
        }
        if let Some(state_apply) = self
            .animator
            .current_state_apply(live_id!(spaces_bar_animator))
        {
            self.script_apply(vm, &Apply::Animate, scope, state_apply.into());
        }
    }
}

impl Widget for SpacesBarWrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // TODO: i want to uncomment this, but adding it back in will break
        //       the animation of showing the SpacesBarWrapper.
        //       I'm not sure why the SpacesBar is getting redrawn constantly though.
        // if walk.height.to_fixed().is_some_and(|h| h < 0.01) {
        //     return DrawStep::done();
        // }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SpacesBarWrapperRef {
    /// Shows or hides the spaces bar by animating it in or out.
    fn show_or_hide(&self, cx: &mut Cx, show: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        if show {
            inner.animator_play(cx, ids!(spaces_bar_animator.show));
        } else {
            inner.animator_play(cx, ids!(spaces_bar_animator.hide));
        }
        inner.redraw(cx);
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,

    /// The previously-selected navigation tab, used to determine which tab
    /// and top-level view we return to after closing the settings screen.
    ///
    /// Note that the current selected tap is stored in `AppState` so that
    /// other widgets can easily access it.
    #[rust]
    previous_selection: SelectedTab,
    #[rust]
    is_spaces_bar_shown: bool,

    /// A stack of previously-selected rooms for mobile stack navigation.
    /// When a view is popped off the stack, the previous `selected_room` is restored.
    #[rust]
    mobile_room_nav_stack: Vec<SelectedRoom>,

    /// The most recently applied view-mode override, used to short-circuit
    /// redundant `AdaptiveView` selector reinstalls when an
    /// [`AppPreferencesAction::ViewModeChanged`] action repeats the current
    /// value (e.g., the unconditional broadcast on app-state restore).
    #[rust]
    applied_view_mode: ViewModeOverride,
    #[rust(false)]
    fixture_desktop_product_shell_logged: bool,
    #[rust(false)]
    fixture_mobile_product_shell_logged: bool,
}

impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::Actions(actions) = event {
            let app_state = scope.data.get_mut::<AppState>().unwrap();
            self.push_fixture_initial_mobile_room_view(cx, app_state);
            for action in actions {
                match action.downcast_ref() {
                    Some(NavigationBarAction::GoToHome) => {
                        if !matches!(app_state.selected_tab, SelectedTab::Home) {
                            // Top-level tab selection evidence: GoToHome only
                            // records previous_selection, selects the local
                            // Home tab, emits TabSelected, and updates the
                            // local PageFlip. It sends no Matrix search,
                            // room-list pagination, message, room-state,
                            // membership, or live mutation request.
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::Home;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::GoToAddRoom) => {
                        if !matches!(app_state.selected_tab, SelectedTab::AddRoom) {
                            // Top-level tab selection evidence: GoToAddRoom
                            // only records previous_selection, selects the
                            // local AddRoom page, emits TabSelected, and
                            // updates the local PageFlip. AddRoom join/knock
                            // requests remain behind AddRoomScreen guards; the
                            // tab selection itself sends no Matrix search,
                            // room-list pagination, message, room-state,
                            // membership, or live mutation request.
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::AddRoom;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::GoToSpace { space_name_id }) => {
                        // SpacesBar entry selection evidence: GoToSpace only
                        // updates the selected top-level tab and broadcasts
                        // TabSelected. Space child rows are resolved later
                        // from cached/read SpaceService state; this branch
                        // sends no Matrix mutation or live request itself.
                        let new_space_selection = SelectedTab::Space {
                            space_name_id: space_name_id.clone(),
                        };
                        if app_state.selected_tab != new_space_selection {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = new_space_selection;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    // Only open the settings screen if it is not currently open.
                    Some(NavigationBarAction::OpenSettings) => {
                        if !matches!(app_state.selected_tab, SelectedTab::Settings) {
                            // ProfileIcon settings evidence: opening settings
                            // only stores the previous local tab, selects the
                            // local Settings page, and populates it from the
                            // current AppState/cache. It does not send account,
                            // SettingsScreen from the current AppState/cache
                            // is a local UI populate step here.
                            // profile, message, room-state, membership, or live
                            // mutation requests by itself.
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::Settings;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            if let Some(settings_page) =
                                self.update_active_page_from_selection(cx, app_state)
                            {
                                settings_page
                                    .settings_screen(cx, ids!(settings_screen))
                                    .populate(cx, None, app_state);
                                self.view.redraw(cx);
                            } else {
                                error!("BUG: failed to set active page to show settings screen.");
                            }
                        }
                    }
                    Some(NavigationBarAction::CloseSettings) => {
                        if matches!(app_state.selected_tab, SelectedTab::Settings) {
                            // Settings close evidence: CloseSettings restores
                            // previous_selection in local UI state and
                            // broadcasts the restored tab. It does not send
                            // logout, account/profile mutation, message,
                            // room-state, membership, or live mutation
                            // requests.
                            app_state.selected_tab = self.previous_selection.clone();
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::ToggleSpacesBar) => {
                        self.is_spaces_bar_shown = !self.is_spaces_bar_shown;
                        self.view
                            .spaces_bar_wrapper(cx, ids!(spaces_bar_wrapper))
                            .show_or_hide(cx, self.is_spaces_bar_shown);
                    }
                    // We're the ones who emitted this action, so we don't need to handle it again.
                    Some(NavigationBarAction::TabSelected(_)) | None => {}
                }

                // React to App Settings changes that affect the HomeScreen layout.
                if let Some(AppPreferencesAction::ViewModeChanged(new_mode)) = action.downcast_ref()
                {
                    if *new_mode != self.applied_view_mode {
                        self.apply_view_mode(cx, *new_mode);
                        self.view.redraw(cx);
                    }
                }

                // Handle mobile stack navigation actions (push/pop room views).
                // In Desktop mode, MainDesktopUI also handles RoomsListAction::Selected
                // to manage dock tabs; the mobile push is harmless there (views aren't drawn).
                match action.as_widget_action().cast() {
                    RoomsListAction::Selected(selected_room) => {
                        self.push_selected_room_view(cx, app_state, selected_room, true);
                    }
                    RoomsListAction::InviteAccepted { room_name_id } => {
                        cx.action(AppStateAction::UpgradedInviteToJoinedRoom(
                            room_name_id.room_id().clone(),
                        ));
                    }
                    _ => {}
                }

                // Mobile stack navigation evidence: pressing back pops the local
                // selected-room stack to stay in sync with StackNavigation and
                // sends no Matrix search, message, room-state, or membership request.
                if let StackNavigationAction::Pop = action.as_widget_action().cast() {
                    if app_state.selected_room.is_some() {
                        app_state.selected_room = self.mobile_room_nav_stack.pop();
                    }
                }
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let available_width = cx.turtle().width();
        let app_state = scope.data.get_mut::<AppState>().unwrap();
        self.push_fixture_initial_mobile_room_view(cx, app_state);
        // Note: We need to update the active page before drawing,
        // because if we switched between Desktop and Mobile views,
        // the PageFlip widget will have been reset to its default,
        // so we must re-set it to the correct page based on `app_state.selected_tab`.
        self.update_active_page_from_selection(cx, app_state);
        self.sync_fixture_product_shell(cx, app_state, available_width);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl HomeScreen {
    /// Installs a variant selector on the main `AdaptiveView` that honors the
    /// current [`ViewModeOverride`] preference. `Automatic` falls back to the
    /// default width-based selector.
    fn apply_view_mode(&mut self, cx: &mut Cx, mode: ViewModeOverride) {
        self.view
            .adaptive_view(cx, ids!(main_adaptive_view))
            .set_variant_selector(mode.variant_selector());
        self.applied_view_mode = mode;
    }

    fn update_active_page_from_selection(
        &mut self,
        cx: &mut Cx,
        app_state: &mut AppState,
    ) -> Option<WidgetRef> {
        self.view
            .page_flip(cx, ids!(home_screen_page_flip))
            .set_active_page(
                cx,
                match app_state.selected_tab {
                    SelectedTab::Space { .. } | SelectedTab::Home => id!(home_page),
                    SelectedTab::Settings => id!(settings_page),
                    SelectedTab::AddRoom => id!(add_room_page),
                },
            )
    }

    /// Room StackNavigationView instances, one per stack depth.
    /// Each depth gets its own dedicated view widget to avoid
    /// complex state save/restore when views would otherwise be reused.
    const ROOM_VIEW_IDS: [LiveId; 16] = [
        live_id!(room_view_0),
        live_id!(room_view_1),
        live_id!(room_view_2),
        live_id!(room_view_3),
        live_id!(room_view_4),
        live_id!(room_view_5),
        live_id!(room_view_6),
        live_id!(room_view_7),
        live_id!(room_view_8),
        live_id!(room_view_9),
        live_id!(room_view_10),
        live_id!(room_view_11),
        live_id!(room_view_12),
        live_id!(room_view_13),
        live_id!(room_view_14),
        live_id!(room_view_15),
    ];

    /// The RoomScreen widget IDs inside each room view,
    /// corresponding 1:1 with [`Self::ROOM_VIEW_IDS`].
    const ROOM_SCREEN_IDS: [LiveId; 16] = [
        live_id!(room_screen_0),
        live_id!(room_screen_1),
        live_id!(room_screen_2),
        live_id!(room_screen_3),
        live_id!(room_screen_4),
        live_id!(room_screen_5),
        live_id!(room_screen_6),
        live_id!(room_screen_7),
        live_id!(room_screen_8),
        live_id!(room_screen_9),
        live_id!(room_screen_10),
        live_id!(room_screen_11),
        live_id!(room_screen_12),
        live_id!(room_screen_13),
        live_id!(room_screen_14),
        live_id!(room_screen_15),
    ];

    /// Returns the room view and room screen LiveIds for the given stack depth.
    /// Clamps to the last available view if depth exceeds the pool size.
    fn room_ids_for_depth(depth: usize) -> (LiveId, LiveId) {
        let index = depth.min(Self::ROOM_VIEW_IDS.len() - 1);
        (Self::ROOM_VIEW_IDS[index], Self::ROOM_SCREEN_IDS[index])
    }

    /// Pushes the appropriate StackNavigationView for the given `SelectedRoom`,
    /// configuring the view's content widget and header title.
    ///
    /// Each stack depth gets its own dedicated room view widget,
    /// supporting deep navigation (room → thread → room → thread → ...).
    fn push_selected_room_view(
        &mut self,
        cx: &mut Cx,
        app_state: &mut AppState,
        selected_room: SelectedRoom,
        preserve_previous_room: bool,
    ) {
        let new_depth = self.view.stack_navigation(cx, ids!(view_stack)).depth();

        let view_id = match &selected_room {
            SelectedRoom::JoinedRoom { room_name_id }
            | SelectedRoom::Thread { room_name_id, .. } => {
                let (view_id, room_screen_id) = Self::room_ids_for_depth(new_depth);
                let thread_root = if let SelectedRoom::Thread {
                    thread_root_event_id,
                    ..
                } = &selected_room
                {
                    Some(thread_root_event_id.clone())
                } else {
                    None
                };
                self.view
                    .room_screen(cx, &[room_screen_id])
                    .set_displayed_room(cx, room_name_id, thread_root);
                view_id
            }
            SelectedRoom::InvitedRoom { room_name_id } => {
                self.view
                    .invite_screen(cx, ids!(invite_screen))
                    .set_displayed_invite(cx, room_name_id);
                id!(invite_view)
            }
            SelectedRoom::Space { space_name_id } => {
                self.view
                    .space_lobby_screen(cx, ids!(space_lobby_screen))
                    .set_displayed_space(cx, space_name_id);
                id!(space_lobby_view)
            }
        };

        let stack_navigation = self.view.stack_navigation(cx, ids!(view_stack));
        stack_navigation.set_title(cx, view_id, &selected_room.display_name());

        // Save the current selected_room onto the navigation stack before replacing it.
        if preserve_previous_room && let Some(prev) = app_state.selected_room.take() {
            self.mobile_room_nav_stack.push(prev);
        }
        app_state.selected_room = Some(selected_room);

        // Push the view onto the mobile navigation stack. This is local UI state
        // replay over already selected room data; request paths stay inside the
        // room widgets and no Matrix search, message, room-state, or membership
        // request is emitted by the stack push itself.
        stack_navigation.push(cx, view_id);
        self.view.redraw(cx);
    }

    fn push_fixture_initial_mobile_room_view(&mut self, cx: &mut Cx, app_state: &mut AppState) {
        if !crate::hepta_fixture::is_fixture_mode_enabled() {
            return;
        }
        if self.view.stack_navigation(cx, ids!(view_stack)).depth() != 0 {
            return;
        }
        let Some(selected_room) = app_state.selected_room.clone() else {
            return;
        };
        self.push_selected_room_view(cx, app_state, selected_room, false);
    }

    fn sync_fixture_product_shell(
        &mut self,
        cx: &mut Cx,
        app_state: &mut AppState,
        available_width: f64,
    ) {
        let fixture_mode = crate::hepta_fixture::is_fixture_mode_enabled();
        let mobile_product_shell = fixture_mode
            && available_width.is_finite()
            && available_width > 0.0
            && available_width <= HEPTA_FIXTURE_MOBILE_PRODUCT_SHELL_MAX_WIDTH;
        let desktop_product_shell = fixture_mode && !mobile_product_shell;

        self.view
            .view(cx, ids!(standard_desktop_shell))
            .set_visible(cx, !desktop_product_shell);
        self.view
            .view(cx, ids!(fixture_desktop_product_shell))
            .set_visible(cx, desktop_product_shell);
        self.view
            .view(cx, ids!(standard_mobile_shell))
            .set_visible(cx, !mobile_product_shell);
        self.view
            .view(cx, ids!(fixture_mobile_product_shell))
            .set_visible(cx, mobile_product_shell);

        if !fixture_mode {
            return;
        }

        let Some(selected_room) = app_state.selected_room.clone() else {
            return;
        };
        let room_name_id = match selected_room {
            SelectedRoom::JoinedRoom { room_name_id } => room_name_id,
            SelectedRoom::Thread { room_name_id, .. } => room_name_id,
            SelectedRoom::InvitedRoom { .. } | SelectedRoom::Space { .. } => return,
        };
        if !crate::hepta_fixture::is_fixture_room_id(room_name_id.room_id().as_str()) {
            return;
        }

        if desktop_product_shell {
            if !self.fixture_desktop_product_shell_logged {
                log!(
                    "Hepta Native fixture product shell selected for desktop: {}",
                    room_name_id
                );
                log!(
                    "Hepta Native fixture cockpit selected for desktop product shell: {}",
                    room_name_id
                );
                log!(
                    "Hepta Native fixture Matrix composer hidden for desktop product shell: {}",
                    room_name_id
                );
                self.fixture_desktop_product_shell_logged = true;
            }
        }
        if mobile_product_shell {
            if !self.fixture_mobile_product_shell_logged {
                log!(
                    "Hepta Native fixture product shell selected for mobile: {}",
                    room_name_id
                );
                log!(
                    "Hepta Native fixture cockpit selected for mobile product shell: {}",
                    room_name_id
                );
                log!(
                    "Hepta Native fixture Matrix composer hidden for mobile product shell: {}",
                    room_name_id
                );
                self.fixture_mobile_product_shell_logged = true;
            }
        }
    }
}

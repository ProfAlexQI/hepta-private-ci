use super::*;

const HEPTA_FIXTURE_FORBIDDEN_PRIMARY_COPY: [&str; 17] = [
    "Fixture mode",
    "mutation=false",
    "payload hash",
    "renderer contract",
    "old JS",
    "blank module fallback",
    "NO_REPLY",
    "row detail",
    "metrics preview",
    "copy metrics evidence",
    "cargo-makepad",
    "Outbox",
    "outbox preview",
    "copy outbox evidence",
    "side-effect",
    "Side-effect",
    "Shell health",
];

fn assert_product_copy(value: &str) {
    for forbidden in HEPTA_FIXTURE_FORBIDDEN_PRIMARY_COPY {
        assert!(
            !value.contains(forbidden),
            "primary fixture copy leaked engineering term {forbidden:?}: {value}"
        );
    }
}

#[test]
fn hepta_fixture_cockpit_has_a_card_for_each_sample_event() {
    assert!(sample_matrix_timeline_events().len() <= HEPTA_FIXTURE_COCKPIT_VISIBLE_CARD_CAPACITY);
}

#[test]
fn hepta_fixture_cockpit_summary_tracks_product_state() {
    let summary = summarize_fixture_events(&sample_matrix_timeline_events());

    assert_eq!(summary.total_events, 9);
    assert_eq!(summary.active_events, 3);
    assert_eq!(summary.waiting_events, 3);
    assert_eq!(summary.completed_events, 3);
    assert!(summary.current_bridge_visible);
    assert_eq!(summary.bridge_badge(), "ready");
    assert!(summary.safety_line().contains("external mutation false"));
}

#[test]
fn hepta_fixture_operation_workbench_surfaces_safe_action_lanes() {
    let workbench = summarize_operation_workbench();

    assert_eq!(workbench.item_count, 4);
    assert_eq!(workbench.local_preview_count, 1);
    assert_eq!(workbench.awaiting_confirmation_count, 2);
    assert_eq!(workbench.policy_blocked_count, 1);
    assert!(workbench.all_external_mutation_disabled);
    assert!(workbench.composer_title.contains("draft task plan"));
    assert!(workbench.composer_body.contains("mutation=false"));
    assert!(
        workbench
            .approval_title
            .contains("approval-install-cargo-makepad")
    );
    assert!(workbench.approval_body.contains("preview required"));
    assert!(workbench.outbox_body.contains("blocked=1"));
    assert!(workbench.outbox_body.contains("mutation=false"));
}

#[test]
fn hepta_fixture_app_chrome_tracks_desktop_and_mobile_routes_without_live_mutation() {
    let chrome = fixture_app_chrome();

    assert_eq!(chrome.workspace_title, "Hepta Runtime Cockpit");
    assert_eq!(chrome.desktop_routes.len(), 4);
    assert_eq!(
        chrome.mobile_tabs,
        ["Home", "Actions", "Approvals", "Inspector"]
    );
    assert_eq!(
        chrome
            .desktop_routes
            .iter()
            .filter(|route| route.selected)
            .count(),
        1
    );
    assert!(chrome.command_line().contains("Ask Hepta"));
    assert!(!chrome.live_side_effects);
}

#[test]
fn hepta_fixture_layout_policy_collapses_mobile_to_task_first_without_live_mutation() {
    assert_eq!(
        fixture_layout_for_available_width(1280.0),
        HeptaFixtureLayout::DesktopProduct
    );
    assert_eq!(
        fixture_layout_for_available_width(500.0),
        HeptaFixtureLayout::MobileTaskFirst
    );
    assert_eq!(
        fixture_layout_for_available_width(f64::INFINITY),
        HeptaFixtureLayout::DesktopProduct
    );
    assert_eq!(
        parse_fixture_layout("task-first"),
        Some(HeptaFixtureLayout::MobileTaskFirst)
    );
    assert_eq!(
        parse_fixture_layout("desktop"),
        Some(HeptaFixtureLayout::DesktopProduct)
    );
    assert_eq!(
        parse_fixture_layout("desktop-full"),
        Some(HeptaFixtureLayout::DesktopFull)
    );

    let mobile = HeptaFixtureLayoutContract::for_layout(HeptaFixtureLayout::MobileTaskFirst);
    assert!(!mobile.live_side_effects);
    assert!(mobile.visible_sections.contains(&"telegram mobile chat"));
    assert!(mobile.visible_sections.contains(&"telegram chat header"));
    assert!(mobile.visible_sections.contains(&"telegram message thread"));
    assert!(mobile.visible_sections.contains(&"telegram composer"));
    assert!(mobile.collapsed_sections.contains(&"mobile top app bar"));
    assert!(mobile.collapsed_sections.contains(&"core flows"));
    assert!(mobile.collapsed_sections.contains(&"current work"));
    assert!(mobile.collapsed_sections.contains(&"selected row detail"));
    assert!(
        mobile
            .collapsed_sections
            .contains(&"current work row cards")
    );
    assert!(mobile.collapsed_sections.contains(&"metrics"));
    assert!(mobile.collapsed_sections.contains(&"safety status"));
    assert!(mobile.collapsed_sections.contains(&"command palette"));
    assert!(mobile.collapsed_sections.contains(&"active route surface"));
    assert!(mobile.collapsed_sections.contains(&"route state cards"));
    assert!(mobile.collapsed_sections.contains(&"review queue"));
    assert!(mobile.collapsed_sections.contains(&"evidence timeline"));

    let desktop = HeptaFixtureLayoutContract::for_layout(HeptaFixtureLayout::DesktopProduct);
    assert!(!desktop.live_side_effects);
    assert!(desktop.visible_sections.contains(&"telegram desktop shell"));
    assert!(desktop.visible_sections.contains(&"telegram chat list"));
    assert!(
        desktop
            .visible_sections
            .contains(&"telegram message thread")
    );
    assert!(desktop.visible_sections.contains(&"telegram composer"));
    assert!(desktop.collapsed_sections.contains(&"telegram info panel"));
    assert!(desktop.collapsed_sections.contains(&"command palette"));
    assert!(desktop.collapsed_sections.contains(&"current work"));
    assert!(desktop.collapsed_sections.contains(&"selected row detail"));
    assert!(!desktop.visible_sections.contains(&"desktop route cards"));
    assert!(!desktop.visible_sections.contains(&"core flows"));
    assert!(
        desktop
            .collapsed_sections
            .contains(&"current work row cards")
    );
    assert!(desktop.collapsed_sections.contains(&"metrics"));
    assert!(desktop.collapsed_sections.contains(&"safety status"));
    assert!(desktop.collapsed_sections.contains(&"desktop route cards"));
    assert!(
        desktop
            .collapsed_sections
            .contains(&"duplicate core flow cards")
    );
    assert!(desktop.collapsed_sections.contains(&"active route surface"));
    assert!(desktop.collapsed_sections.contains(&"route state cards"));
    assert!(desktop.collapsed_sections.contains(&"review queue"));
    assert!(desktop.collapsed_sections.contains(&"mobile dock"));

    let full = HeptaFixtureLayoutContract::for_layout(HeptaFixtureLayout::DesktopFull);
    assert!(full.visible_sections.contains(&"active route surface"));
    assert!(full.visible_sections.contains(&"route state cards"));
    assert!(full.visible_sections.contains(&"evidence timeline"));
    assert!(full.collapsed_sections.contains(&"telegram desktop shell"));
    assert!(full.collapsed_sections.contains(&"telegram mobile chat"));
    assert!(full.collapsed_sections.contains(&"mobile dock"));
}

#[test]
fn hepta_fixture_layout_markers_cover_window_smoke_modes_without_live_mutation() {
    assert_eq!(
        HeptaFixtureLayout::DesktopProduct.log_label(),
        "desktop-product"
    );
    assert_eq!(HeptaFixtureLayout::DesktopFull.log_label(), "desktop-full");
    assert_eq!(
        HeptaFixtureLayout::MobileTaskFirst.log_label(),
        "mobile-task-first"
    );
    assert!(
        !HeptaFixtureLayoutContract::for_layout(HeptaFixtureLayout::DesktopProduct)
            .live_side_effects
    );
    assert!(
        !HeptaFixtureLayoutContract::for_layout(HeptaFixtureLayout::MobileTaskFirst)
            .live_side_effects
    );
}

#[test]
fn hepta_fixture_mobile_secondary_surface_visibility_is_selection_bound() {
    assert!(!fixture_secondary_surface_visible_for_layout(
        HeptaFixtureLayout::MobileTaskFirst,
        HeptaFixtureSecondarySurfaceKey::None
    ));
    assert!(fixture_secondary_surface_visible_for_layout(
        HeptaFixtureLayout::MobileTaskFirst,
        HeptaFixtureSecondarySurfaceKey::Search
    ));
    assert!(fixture_secondary_surface_visible_for_layout(
        HeptaFixtureLayout::DesktopFull,
        HeptaFixtureSecondarySurfaceKey::None
    ));
    assert!(!fixture_secondary_surface_visible_for_layout(
        HeptaFixtureLayout::DesktopProduct,
        HeptaFixtureSecondarySurfaceKey::Search
    ));
}

#[test]
fn hepta_fixture_product_display_copy_hides_engineering_terms() {
    let summary = summarize_fixture_events(&sample_matrix_timeline_events());
    assert_product_copy(&summary.safety_display_line());

    let chrome = fixture_app_chrome();
    assert_product_copy(&chrome.display_command_line());
    for route in chrome.desktop_routes {
        assert_product_copy(&route.display_line());
    }

    let workbench = summarize_operation_workbench();
    assert_product_copy(&workbench.composer_display_body());
    assert_product_copy(&workbench.approval_display_body());
    assert_product_copy(&workbench.outbox_display_body());

    for result in fixture_command_results() {
        assert_product_copy(&result.display_line());
    }
    for page in fixture_route_shell_pages() {
        assert_product_copy(&page.display_line());
    }
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        for state in fixture_route_states_for(route) {
            assert_product_copy(&state.display_line());
        }

        let panel = fixture_route_primary_panel_for(route);
        assert_product_copy(&panel.display_empty_state_line());

        let content = fixture_route_main_content_for(route);
        assert_product_copy(&content.display_hint_with_selected_row(0));
        for row in content.rows {
            assert_product_copy(row.display_line());
        }

        for selected_row_index in 0..HEPTA_FIXTURE_ROUTE_ROW_COUNT {
            let detail = fixture_selected_row_detail_for_row(route, selected_row_index);
            assert_product_copy(detail.detail_title);
            assert_product_copy(detail.detail_body);
            assert_product_copy(&detail.display_route_line());
            assert_product_copy(&detail.display_evidence_line());
            assert_product_copy(&detail.display_inspector_line());

            let action_strip = fixture_selected_row_action_strip_for(&detail);
            for action in action_strip.display_action_lines() {
                assert_product_copy(&action);
            }
        }
    }

    for surface in HEPTA_FIXTURE_SECONDARY_SURFACE_KEYS {
        let route_surface = fixture_secondary_route_surface_for(surface);
        let primary_panel = fixture_secondary_primary_panel_for(surface);
        assert_product_copy(route_surface.title);
        assert_product_copy(route_surface.focus);
        assert_product_copy(route_surface.primary_action);
        assert_product_copy(route_surface.evidence_anchor);
        assert_product_copy(primary_panel.title);
        assert_product_copy(primary_panel.signal);
        assert_product_copy(primary_panel.operator_control);
        assert_product_copy(primary_panel.evidence_stack);
        assert_product_copy(&primary_panel.display_empty_state_line());

        for result in fixture_secondary_command_results_for(surface) {
            assert_product_copy(result.label);
            assert_product_copy(result.title);
            assert_product_copy(result.detail);
            assert_product_copy(&result.display_line());
        }

        for page in fixture_secondary_route_shell_pages_for(surface) {
            assert_product_copy(page.route);
            assert_product_copy(page.title);
            assert_product_copy(page.primary_surface);
            assert_product_copy(&page.display_line());
        }
    }
}

#[test]
fn hepta_fixture_default_product_copy_is_current_work_first() {
    let content = fixture_route_main_content_for(HeptaFixtureRouteKey::Home);
    assert_eq!(content.title, "Current work");
    assert_eq!(content.rows[0].label, "Current task");

    let detail = fixture_selected_row_detail_for_row(HeptaFixtureRouteKey::Home, 0);
    assert_eq!(detail.detail_title, "Current task summary");
    assert!(!detail.detail_body.contains("row"));

    let action_strip = fixture_selected_row_action_strip_for(&detail);
    assert_eq!(action_strip.preview_action, "task summary");
    assert_eq!(action_strip.copy_action, "copy evidence summary");
}

#[test]
fn hepta_fixture_deep_route_copy_uses_product_language() {
    let actions = fixture_route_main_content_for(HeptaFixtureRouteKey::Actions);
    assert_eq!(actions.title, "Action review");
    assert_eq!(actions.rows[1].label, "Pending steps");

    let approvals = fixture_route_main_content_for(HeptaFixtureRouteKey::Approvals);
    assert_eq!(approvals.title, "Approval decision");
    assert_eq!(approvals.rows[0].title, "Local build approval");
    assert_eq!(approvals.rows[1].label, "Request detail");

    let inspector = fixture_route_main_content_for(HeptaFixtureRouteKey::Inspector);
    assert_eq!(inspector.title, "Evidence review");
    assert_eq!(inspector.rows[1].label, "Action boundary");
    assert_eq!(inspector.rows[2].label, "App health");

    for route in [
        HeptaFixtureRouteKey::Actions,
        HeptaFixtureRouteKey::Approvals,
        HeptaFixtureRouteKey::Inspector,
    ] {
        let content = fixture_route_main_content_for(route);
        assert_product_copy(content.title);
        assert_product_copy(content.hint);
        for row in content.rows {
            assert_product_copy(row.label);
            assert_product_copy(row.title);
            assert_product_copy(row.detail);
        }

        for selected_row_index in 0..HEPTA_FIXTURE_ROUTE_ROW_COUNT {
            let detail = fixture_selected_row_detail_for_row(route, selected_row_index);
            assert_product_copy(detail.detail_title);
            assert_product_copy(detail.detail_body);
            assert_product_copy(detail.evidence);
            assert_product_copy(detail.inspector_title);
            assert_product_copy(detail.inspector_body);

            for action in fixture_selected_row_action_strip_for(&detail).display_action_lines() {
                assert_product_copy(&action);
            }
        }
    }
}

#[test]
fn hepta_fixture_app_chrome_tracks_selected_route_without_live_mutation() {
    for selected_route in HEPTA_FIXTURE_ROUTE_KEYS {
        let chrome = fixture_app_chrome_for(selected_route);
        let selected = chrome
            .desktop_routes
            .iter()
            .filter(|route| route.selected)
            .collect::<Vec<_>>();

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].label, selected_route.label());
        assert!(selected[0].detail_line().contains("selected"));
        assert!(!chrome.live_side_effects);
    }
}

#[test]
fn hepta_fixture_command_palette_surfaces_safe_route_and_search_results() {
    let chrome = fixture_app_chrome();
    let results = fixture_command_results();

    assert_eq!(results.len(), 4);
    assert!(
        results
            .iter()
            .any(|result| result.selection_route() == HeptaFixtureRouteKey::Home)
    );
    assert!(
        results
            .iter()
            .any(|result| result.selection_route() == HeptaFixtureRouteKey::Actions)
    );
    assert!(
        results
            .iter()
            .any(|result| result.selection_route() == HeptaFixtureRouteKey::Approvals)
    );
    assert!(
        results
            .iter()
            .any(|result| result.selection_route() == HeptaFixtureRouteKey::Inspector)
    );
    assert!(results.iter().any(|result| result.label == "Ask"
        && result.title == "Ask Hepta"
        && result.route == "Home"));
    assert!(results.iter().any(|result| result.label == "Plan"
        && result.title == "Review the plan"
        && result.route == "Actions"));
    assert!(
        results
            .iter()
            .any(|result| result.label == "Evidence" && result.route == "Inspector")
    );
    assert!(
        results
            .iter()
            .any(|result| result.label == "Approve" && result.detail.contains("locked action"))
    );
    assert!(results.iter().all(|result| !result.requires_live_mutation));
    assert!(results.iter().all(|result| !result.shortcut.is_empty()));
    assert!(
        results
            .iter()
            .all(|result| chrome.mobile_tabs.contains(&result.route))
    );
    assert!(
        results
            .iter()
            .any(|result| result.preview_line().contains("mutation=false"))
    );
}

#[test]
fn hepta_fixture_route_shell_pages_match_chrome_routes_without_live_mutation() {
    let chrome = fixture_app_chrome();
    let pages = fixture_route_shell_pages();

    assert_eq!(pages.len(), 4);
    for page in &pages {
        assert!(
            chrome
                .desktop_routes
                .iter()
                .any(|route| route.label == page.route)
        );
        assert!(page.accepts_selection);
        assert!(!page.live_side_effects);
        assert!(page.contract_line().contains("live mutation=false"));
    }
    assert!(
        pages
            .iter()
            .any(|page| page.route == "Actions" && page.primary_surface.contains("pending steps"))
    );
    assert!(
        pages
            .iter()
            .any(|page| page.route == "Approvals"
                && page.primary_surface.contains("request preview"))
    );
}

#[test]
fn hepta_fixture_route_states_drive_first_screen_content_without_live_mutation() {
    let chrome = fixture_app_chrome();
    let states = fixture_route_states();

    assert_eq!(states.len(), 4);
    assert_eq!(states.iter().filter(|state| state.active).count(), 1);
    assert_eq!(
        states.iter().find(|state| state.active).unwrap().route,
        "Home"
    );
    for state in &states {
        assert!(chrome.mobile_tabs.contains(&state.route));
        assert!(!state.content_anchor.is_empty());
        assert!(!state.selection_source.is_empty());
        assert!(!state.primary_action.is_empty());
        assert!(!state.live_side_effects);
        assert!(state.state_line().contains("mutation=false"));
    }
    assert!(states.iter().any(
        |state| state.route == "Actions" && state.content_anchor.contains("pending steps")
    ));
    assert!(
        states.iter().any(|state| state.route == "Approvals"
            && state.selection_source.contains("approval review"))
    );
}

#[test]
fn hepta_fixture_route_states_can_select_each_route_without_live_mutation() {
    for selected in HEPTA_FIXTURE_ROUTE_KEYS {
        let states = fixture_route_states_for(selected);

        assert_eq!(states.len(), 4);
        assert_eq!(states.iter().filter(|state| state.active).count(), 1);
        let active = states.iter().find(|state| state.active).unwrap();
        assert_eq!(active.route, selected.label());
        assert_eq!(active.content_anchor, selected.content_anchor());
        assert!(states.iter().all(|state| !state.live_side_effects));
        assert!(
            states
                .iter()
                .all(|state| state.state_line().contains("mutation=false"))
        );
    }
}

#[test]
fn hepta_fixture_route_parser_accepts_fixture_aliases_and_falls_back_home() {
    assert_eq!(
        parse_fixture_route("actions"),
        HeptaFixtureRouteKey::Actions
    );
    assert_eq!(
        parse_fixture_route("Approval inbox"),
        HeptaFixtureRouteKey::Approvals
    );
    assert_eq!(
        parse_fixture_route("runtime inspector"),
        HeptaFixtureRouteKey::Inspector
    );
    assert_eq!(
        parse_fixture_route("unknown route"),
        HeptaFixtureRouteKey::Home
    );
}

#[test]
fn hepta_fixture_secondary_surface_parser_and_copy_are_local_only() {
    assert_eq!(
        parse_fixture_secondary_surface("message-search"),
        HeptaFixtureSecondarySurfaceKey::Search
    );
    assert_eq!(
        parse_fixture_secondary_surface("room_settings"),
        HeptaFixtureSecondarySurfaceKey::Settings
    );
    assert_eq!(
        parse_fixture_secondary_surface("files"),
        HeptaFixtureSecondarySurfaceKey::Attachment
    );
    assert_eq!(
        parse_fixture_secondary_surface("audio"),
        HeptaFixtureSecondarySurfaceKey::Voice
    );
    assert_eq!(
        parse_fixture_secondary_surface("confirmation"),
        HeptaFixtureSecondarySurfaceKey::Modal
    );
    assert_eq!(
        parse_fixture_secondary_surface("unknown"),
        HeptaFixtureSecondarySurfaceKey::None
    );

    for surface in HEPTA_FIXTURE_SECONDARY_SURFACE_KEYS {
        let route_surface = fixture_secondary_route_surface_for(surface);
        let primary_panel = fixture_secondary_primary_panel_for(surface);
        assert_eq!(route_surface.route, surface.label());
        assert!(!route_surface.live_side_effects);
        assert!(!primary_panel.live_side_effects);
        assert_product_copy(route_surface.title);
        assert_product_copy(route_surface.focus);
        assert_product_copy(primary_panel.signal);
        assert_product_copy(primary_panel.operator_control);
        assert_product_copy(primary_panel.evidence_stack);

        let (active_surface, active_panel) =
            fixture_active_surface_for(HeptaFixtureRouteKey::Home, surface);
        assert_eq!(active_surface.route, surface.label());
        assert_eq!(active_panel.route, surface.label());
        assert!(!active_surface.live_side_effects);
        assert!(!active_panel.live_side_effects);

        for result in fixture_secondary_command_results_for(surface) {
            assert_eq!(result.route, surface.label());
            assert!(!result.requires_live_mutation);
        }

        let pages = fixture_secondary_route_shell_pages_for(surface);
        assert_eq!(pages[0].route, surface.label());
        assert!(pages.iter().all(|page| page.accepts_selection));
        assert!(pages.iter().all(|page| !page.live_side_effects));
    }

    let (default_surface, default_panel) = fixture_active_surface_for(
        HeptaFixtureRouteKey::Actions,
        HeptaFixtureSecondarySurfaceKey::None,
    );
    assert_eq!(default_surface.route, "Actions");
    assert_eq!(default_panel.route, "Actions");
}

#[test]
fn hepta_fixture_selected_route_surface_tracks_each_route_without_live_mutation() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        let surface = fixture_route_surface_for(route);

        assert_eq!(surface.route, route.label());
        assert!(!surface.title.is_empty());
        assert!(!surface.focus.is_empty());
        assert!(!surface.primary_action.is_empty());
        assert!(!surface.evidence_anchor.is_empty());
        assert!(!surface.live_side_effects);
        assert!(surface.surface_line().contains("mutation=false"));
    }

    assert!(
        fixture_route_surface_for(HeptaFixtureRouteKey::Actions)
            .focus
            .contains("pending steps")
    );
    assert!(
        fixture_route_surface_for(HeptaFixtureRouteKey::Approvals)
            .focus
            .contains("request preview")
    );
    assert!(
        fixture_route_surface_for(HeptaFixtureRouteKey::Inspector)
            .evidence_anchor
            .contains("AI calls")
    );
    assert!(
        fixture_route_surface_for(HeptaFixtureRouteKey::Inspector)
            .evidence_anchor
            .contains("execution paused")
    );
}

#[test]
fn hepta_fixture_route_primary_panels_cover_each_route_without_live_mutation() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        let panel = fixture_route_primary_panel_for(route);

        assert_eq!(panel.route, route.label());
        assert!(!panel.title.is_empty());
        assert!(!panel.signal.is_empty());
        assert!(!panel.operator_control.is_empty());
        assert!(!panel.evidence_stack.is_empty());
        assert!(!panel.empty_state.is_empty());
        assert!(!panel.live_side_effects);
        assert!(panel.empty_state_line().contains("mutation=false"));
    }

    assert!(
        fixture_route_primary_panel_for(HeptaFixtureRouteKey::Actions)
            .operator_control
            .contains("dry-run")
    );
    assert!(
        fixture_route_primary_panel_for(HeptaFixtureRouteKey::Approvals)
            .evidence_stack
            .contains("approval status")
    );
    assert!(
        fixture_route_primary_panel_for(HeptaFixtureRouteKey::Inspector)
            .evidence_stack
            .contains("changes paused")
    );
}

#[test]
fn hepta_fixture_route_main_content_rows_cover_each_route_without_live_mutation() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        let content = fixture_route_main_content_for(route);

        assert_eq!(content.route, route.label());
        assert!(!content.title.is_empty());
        assert!(!content.hint.is_empty());
        assert!(!content.live_side_effects);
        assert!(content.hint_line().contains("mutation=false"));
        assert_eq!(content.rows.len(), 3);
        for row in content.rows {
            assert_eq!(row.route, route.label());
            assert!(!row.label.is_empty());
            assert!(!row.title.is_empty());
            assert!(!row.detail.is_empty());
            assert!(!row.live_side_effects);
            assert!(row.preview_line().contains("mutation=false"));
        }
    }

    let actions = fixture_route_main_content_for(HeptaFixtureRouteKey::Actions);
    assert!(actions.rows.iter().any(|row| row.label == "Pending steps"));
    assert!(
        actions
            .rows
            .iter()
            .any(|row| row.detail.contains("before dispatch"))
    );

    let approvals = fixture_route_main_content_for(HeptaFixtureRouteKey::Approvals);
    assert!(
        approvals
            .rows
            .iter()
            .any(|row| row.label == "Request detail" && row.detail.contains("command evidence"))
    );
    assert!(
        approvals
            .rows
            .iter()
            .any(|row| row.label == "Approval check")
    );

    let inspector = fixture_route_main_content_for(HeptaFixtureRouteKey::Inspector);
    assert!(
        inspector
            .rows
            .iter()
            .any(|row| row.label == "Action boundary" && row.title.contains("paused"))
    );
}

#[test]
fn hepta_fixture_selected_row_detail_links_rows_to_inspector_without_live_mutation() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        let content = fixture_route_main_content_for(route);
        let detail = fixture_selected_row_detail_for(route);

        assert_eq!(detail.route, route.label());
        assert_eq!(
            detail.selected_row_index,
            default_fixture_row_index_for(route)
        );
        assert!(
            content
                .rows
                .iter()
                .any(|row| row.label == detail.selected_row_label
                    && row.title == detail.selected_row_title)
        );
        assert!(!detail.detail_title.is_empty());
        assert!(!detail.detail_body.is_empty());
        assert!(!detail.evidence.is_empty());
        assert!(!detail.inspector_title.is_empty());
        assert!(!detail.inspector_body.is_empty());
        assert!(!detail.live_side_effects);
        assert!(detail.evidence_line().contains("mutation=false"));
        assert!(detail.inspector_line().contains("inspector="));
    }

    assert_eq!(
        fixture_selected_row_detail_for(HeptaFixtureRouteKey::Actions).selected_row_label,
        "Pending steps"
    );
    assert_eq!(
        fixture_selected_row_detail_for(HeptaFixtureRouteKey::Approvals).selected_row_label,
        "Request detail"
    );
    assert!(
        fixture_selected_row_detail_for(HeptaFixtureRouteKey::Inspector)
            .evidence
            .contains("actions paused")
    );
}

#[test]
fn hepta_fixture_selected_row_variants_cover_each_route_row_without_live_mutation() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        let content = fixture_route_main_content_for(route);

        for selected_row_index in 0..HEPTA_FIXTURE_ROUTE_ROW_COUNT {
            let detail = fixture_selected_row_detail_for_row(route, selected_row_index);
            let row = &content.rows[selected_row_index];

            assert_eq!(detail.route, route.label());
            assert_eq!(detail.selected_row_index, selected_row_index);
            assert_eq!(detail.selected_row_label, row.label);
            assert_eq!(detail.selected_row_title, row.title);
            assert!(!detail.detail_title.is_empty());
            assert!(!detail.detail_body.is_empty());
            assert!(!detail.evidence.is_empty());
            assert!(!detail.inspector_title.is_empty());
            assert!(!detail.inspector_body.is_empty());
            assert!(!detail.live_side_effects);
            assert!(
                detail
                    .route_line()
                    .contains(&format!("row={selected_row_index}"))
            );
            assert!(detail.evidence_line().contains("mutation=false"));
            assert!(
                content
                    .hint_line_with_selected_row(selected_row_index)
                    .contains(row.label)
            );
            assert!(row.label_line(true).contains("Selected"));
        }
    }

    assert_eq!(
        fixture_selected_row_detail_for_row(HeptaFixtureRouteKey::Actions, 0).selected_row_label,
        "Draft plan"
    );
    assert_eq!(
        fixture_selected_row_detail_for_row(HeptaFixtureRouteKey::Approvals, 2).selected_row_label,
        "Decision"
    );
    assert!(
        fixture_selected_row_detail_for_row(HeptaFixtureRouteKey::Inspector, 2)
            .evidence
            .contains("loading")
    );
}

#[test]
fn hepta_fixture_row_parser_accepts_indices_and_route_labels() {
    assert_eq!(default_fixture_row_index_for(HeptaFixtureRouteKey::Home), 0);
    assert_eq!(
        default_fixture_row_index_for(HeptaFixtureRouteKey::Actions),
        1
    );
    assert_eq!(
        parse_fixture_row_index_for(HeptaFixtureRouteKey::Actions, "0"),
        0
    );
    assert_eq!(
        parse_fixture_row_index_for(HeptaFixtureRouteKey::Actions, "Request detail"),
        2
    );
    assert_eq!(
        parse_fixture_row_index_for(HeptaFixtureRouteKey::Approvals, "approval check"),
        0
    );
    assert_eq!(
        parse_fixture_row_index_for(HeptaFixtureRouteKey::Inspector, "action boundary"),
        1
    );
    assert_eq!(
        parse_fixture_row_index_for(HeptaFixtureRouteKey::Inspector, "unknown"),
        default_fixture_row_index_for(HeptaFixtureRouteKey::Inspector)
    );
}

#[test]
fn hepta_fixture_selected_row_action_strip_stays_fixture_only() {
    for route in HEPTA_FIXTURE_ROUTE_KEYS {
        for selected_row_index in 0..HEPTA_FIXTURE_ROUTE_ROW_COUNT {
            let detail = fixture_selected_row_detail_for_row(route, selected_row_index);
            let action_strip = fixture_selected_row_action_strip_for(&detail);
            let action_lines = action_strip.action_lines();

            assert_eq!(action_strip.route, route.label());
            assert_eq!(action_strip.selected_row_label, detail.selected_row_label);
            assert!(!action_strip.preview_action.is_empty());
            assert!(!action_strip.inspect_action.is_empty());
            assert!(!action_strip.copy_action.is_empty());
            assert_eq!(action_strip.execute_action, "Execute disabled");
            assert!(!action_strip.execute_enabled);
            assert!(!action_strip.live_side_effects);
            assert_eq!(action_lines.len(), 4);
            assert!(
                action_lines
                    .iter()
                    .all(|line| line.contains(detail.selected_row_label))
            );
            assert!(action_lines.iter().any(|line| line.contains("Preview")));
            assert!(action_lines.iter().any(|line| line.contains("Inspect")));
            assert!(action_lines.iter().any(|line| line.contains("Copy")));
            assert!(
                action_lines
                    .iter()
                    .any(|line| line.contains("enabled=false") && line.contains("mutation=false"))
            );
        }
    }

    let approvals = fixture_selected_row_action_strip_for(&fixture_selected_row_detail_for_row(
        HeptaFixtureRouteKey::Approvals,
        1,
    ));
    assert!(approvals.inspect_action.contains("exact request"));
    let inspector = fixture_selected_row_action_strip_for(&fixture_selected_row_detail_for_row(
        HeptaFixtureRouteKey::Inspector,
        1,
    ));
    assert!(inspector.inspect_action.contains("action boundary"));
}

#[test]
fn hepta_fixture_interactive_selection_updates_local_state_without_live_mutation() {
    let mut selection = HeptaFixtureInteractiveSelection::new(HeptaFixtureRouteKey::Actions, 0);

    assert_eq!(selection.route, HeptaFixtureRouteKey::Actions);
    assert_eq!(selection.selected_row_index, 0);
    assert!(!selection.live_side_effects);
    assert!(selection.state_line().contains("mutation=false"));
    assert_eq!(selection.selected_detail().selected_row_label, "Draft plan");

    selection.select_row(2);
    let payload_detail = selection.selected_detail();
    let payload_actions = selection.selected_action_strip();
    assert_eq!(payload_detail.selected_row_label, "Request detail");
    assert!(payload_detail.evidence.contains("approval evidence"));
    assert_eq!(payload_actions.selected_row_label, "Request detail");
    assert!(!payload_actions.execute_enabled);
    assert!(!payload_actions.live_side_effects);

    selection.select_route(HeptaFixtureRouteKey::Approvals);
    assert_eq!(selection.route, HeptaFixtureRouteKey::Approvals);
    assert_eq!(
        selection.selected_row_index,
        default_fixture_row_index_for(HeptaFixtureRouteKey::Approvals)
    );
    assert_eq!(
        selection.selected_detail().selected_row_label,
        "Request detail"
    );

    selection.select_row(99);
    let clamped_detail = selection.selected_detail();
    assert_eq!(
        selection.selected_row_index,
        HEPTA_FIXTURE_ROUTE_ROW_COUNT - 1
    );
    assert_eq!(clamped_detail.selected_row_label, "Decision");
    assert!(!selection.live_side_effects);
    assert!(selection.state_line().contains("mutation=false"));

    for result in fixture_command_results() {
        selection.select_route(result.selection_route());
        assert_eq!(selection.route, result.selection_route());
        assert_eq!(
            selection.selected_row_index,
            default_fixture_row_index_for(result.selection_route())
        );
        assert!(!result.requires_live_mutation);
        assert!(!selection.live_side_effects);
    }
}

#[test]
fn hepta_fixture_shell_states_cover_loading_empty_and_error_without_side_effects() {
    let states = fixture_shell_state_cards();

    assert_eq!(states.len(), 3);
    assert_eq!(states[0].label, "Loading");
    assert_eq!(states[1].label, "Empty");
    assert_eq!(states[2].label, "Error");
    assert!(states.iter().all(|state| !state.live_side_effects));
    assert!(
        states
            .iter()
            .any(|state| state.body.contains("sync is not connected"))
    );
}

#[test]
fn hepta_fixture_event_stack_is_grouped_by_runtime_action_and_evidence() {
    let events = sample_matrix_timeline_events();
    let groups = summarize_event_groups(&events);

    assert_eq!(groups.runtime, 2);
    assert_eq!(groups.action, 4);
    assert_eq!(groups.evidence, 3);
    assert_eq!(
        groups.runtime + groups.action + groups.evidence,
        events.len()
    );
    assert_eq!(
        classify_fixture_event(&events[8]),
        HeptaFixtureEventGroup::Runtime
    );
}

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Mutex, OnceLock},
};

use accesskit::{Action, Node, NodeId, Rect, Role, Tree, TreeId, TreeUpdate};
use makepad_widgets::*;

const ROOT_ID: NodeId = NodeId(1);

pub(crate) const HOME_ALL_ROOMS_ID: u64 = 201;
pub(crate) const HOME_ADD_ROOM_ID: u64 = 202;
pub(crate) const HOME_SETTINGS_ID: u64 = 203;
pub(crate) const HOME_TOGGLE_SPACES_ID: u64 = 204;
const HOME_MAIN_ID: u64 = 210;
const HOME_CONTEXT_ID: u64 = 211;

#[derive(Clone, Copy)]
struct HomeControl {
    id: u64,
    bounds: Rect,
    label: &'static str,
}

fn set_safe_value(node: &mut Node, role: Role, value: Option<String>) {
    if role != Role::PasswordInput {
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            node.set_value(value);
        }
    }
}

fn last_update() -> &'static Mutex<Option<TreeUpdate>> {
    static LAST_UPDATE: OnceLock<Mutex<Option<TreeUpdate>>> = OnceLock::new();
    LAST_UPDATE.get_or_init(|| Mutex::new(None))
}

fn area_bounds(cx: &Cx, area: Area) -> Option<Rect> {
    let rect = area.clipped_rect(cx);
    let values = [rect.pos.x, rect.pos.y, rect.size.x, rect.size.y];
    if !values.iter().all(|value| value.is_finite()) || rect.size.x <= 0.0 || rect.size.y <= 0.0 {
        return None;
    }
    Some(Rect {
        x0: rect.pos.x,
        y0: rect.pos.y,
        x1: rect.pos.x + rect.size.x,
        y1: rect.pos.y + rect.size.y,
    })
}

#[allow(clippy::too_many_arguments)]
fn push_bounded_node(
    nodes: &mut Vec<(NodeId, Node)>,
    children: &mut Vec<NodeId>,
    focus: &mut NodeId,
    bounds: Rect,
    id: u64,
    role: Role,
    label: impl Into<String>,
    description: &str,
    value: Option<String>,
    enabled: bool,
    actions: &[Action],
    focused: bool,
) {
    let id = NodeId(id);
    let mut node = Node::new(role);
    let label = label.into();
    if !label.is_empty() {
        node.set_label(label);
    }
    if !description.is_empty() {
        node.set_description(description);
    }
    // AccessKit values are useful for ordinary inputs, but a password value
    // must never enter the semantic tree even if a caller passes one.
    set_safe_value(&mut node, role, value);
    node.set_bounds(bounds);
    if !enabled {
        node.set_disabled();
    }
    if enabled {
        for action in actions {
            node.add_action(*action);
        }
    }
    if focused {
        *focus = id;
    }
    children.push(id);
    nodes.push((id, node));
}

#[allow(clippy::too_many_arguments)]
fn push_node(
    nodes: &mut Vec<(NodeId, Node)>,
    children: &mut Vec<NodeId>,
    focus: &mut NodeId,
    cx: &Cx,
    area: Area,
    id: u64,
    role: Role,
    label: impl Into<String>,
    description: &str,
    value: Option<String>,
    enabled: bool,
    actions: &[Action],
) {
    let Some(bounds) = area_bounds(cx, area) else {
        return;
    };
    push_bounded_node(
        nodes,
        children,
        focus,
        bounds,
        id,
        role,
        label,
        description,
        value,
        enabled,
        actions,
        cx.has_key_focus(area),
    );
}

fn finish_tree(
    mut root: Node,
    children: Vec<NodeId>,
    mut nodes: Vec<(NodeId, Node)>,
    focus: NodeId,
) -> TreeUpdate {
    root.set_children(children);
    nodes.insert(0, (ROOT_ID, root));
    TreeUpdate {
        nodes,
        tree: Some(Tree {
            root: ROOT_ID,
            toolkit_name: Some("Makepad".into()),
            toolkit_version: None,
        }),
        tree_id: TreeId::ROOT,
        focus,
    }
}

fn valid(update: &TreeUpdate) -> bool {
    let Some(tree) = update.tree.as_ref() else {
        return false;
    };
    let ids = update
        .nodes
        .iter()
        .map(|(id, _)| *id)
        .collect::<BTreeSet<_>>();
    if ids.len() != update.nodes.len() || !ids.contains(&update.focus) || !ids.contains(&tree.root)
    {
        return false;
    }

    let nodes = update
        .nodes
        .iter()
        .map(|(id, node)| (*id, node))
        .collect::<BTreeMap<_, _>>();
    let mut parent_counts = ids
        .iter()
        .copied()
        .map(|id| (id, 0_usize))
        .collect::<BTreeMap<_, _>>();
    for (id, node) in &update.nodes {
        for child in node.children() {
            if *child == *id || !ids.contains(child) {
                return false;
            }
            *parent_counts.get_mut(child).expect("child was checked") += 1;
        }
    }
    if parent_counts.get(&tree.root) != Some(&0)
        || parent_counts
            .iter()
            .any(|(id, count)| *id != tree.root && *count != 1)
    {
        return false;
    }

    let mut reachable = BTreeSet::new();
    let mut stack = vec![tree.root];
    while let Some(id) = stack.pop() {
        if !reachable.insert(id) {
            return false;
        }
        stack.extend(
            nodes
                .get(&id)
                .expect("reachable node was checked")
                .children()
                .iter()
                .copied(),
        );
    }
    if reachable != ids || !reachable.contains(&update.focus) {
        return false;
    }

    update.nodes.iter().all(|(_, node)| {
        let bounds_ready = node.bounds().is_some_and(|bounds| {
            [bounds.x0, bounds.y0, bounds.x1, bounds.y1]
                .iter()
                .all(|value| value.is_finite())
                && bounds.x1 > bounds.x0
                && bounds.y1 > bounds.y0
        });
        let children_ready = node.children().iter().all(|child| ids.contains(child));
        let actionable = [Action::Click, Action::Focus, Action::SetValue]
            .iter()
            .any(|action| node.supports_action(*action));
        bounds_ready
            && children_ready
            && (!actionable || node.label().is_some_and(|v| !v.is_empty()))
    })
}

pub(crate) fn reset_cache() {
    *last_update()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

fn publish(cx: &mut Cx, update: TreeUpdate) {
    if !valid(&update) {
        return;
    }
    let mut last = last_update()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if last.as_ref() == Some(&update) {
        return;
    }
    *last = Some(update.clone());
    drop(last);
    cx.update_accessibility_tree(Box::new(update));
}

pub(crate) fn publish_login_tree(
    cx: &mut Cx,
    view: &View,
    password_visible: bool,
    sso_pending: bool,
    modal_button_enabled: bool,
) {
    let modal = view.modal(cx, ids!(login_status_modal));
    if modal.is_open() {
        let Some(bounds) = area_bounds(cx, modal.area()) else {
            return;
        };
        let mut root = Node::new(Role::Dialog);
        root.set_label("Login status");
        root.set_bounds(bounds);
        let mut children = Vec::new();
        let mut nodes = Vec::new();
        let mut focus = ROOT_ID;
        let title = view.label(cx, ids!(login_status_modal.content.title));
        let body = view.label(cx, ids!(login_status_modal.content.body));
        let button = view.button(cx, ids!(login_status_modal.content.button));
        push_node(
            &mut nodes,
            &mut children,
            &mut focus,
            cx,
            title.area(),
            101,
            Role::Heading,
            title.text(),
            "",
            None,
            true,
            &[],
        );
        push_node(
            &mut nodes,
            &mut children,
            &mut focus,
            cx,
            body.area(),
            102,
            Role::Label,
            body.text(),
            "",
            None,
            true,
            &[],
        );
        push_node(
            &mut nodes,
            &mut children,
            &mut focus,
            cx,
            button.area(),
            103,
            Role::Button,
            "Dismiss login status",
            "",
            None,
            modal_button_enabled,
            &[Action::Focus, Action::Click],
        );
        publish(cx, finish_tree(root, children, nodes, focus));
        return;
    }

    let Some(bounds) = area_bounds(cx, view.area()) else {
        return;
    };
    let mut root = Node::new(Role::Window);
    root.set_label("Hepta sign in");
    root.set_bounds(bounds);
    let mut children = Vec::new();
    let mut nodes = Vec::new();
    let mut focus = ROOT_ID;
    let title = view.label(cx, ids!(title));
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        title.area(),
        2,
        Role::Heading,
        "Sign in to Hepta",
        "",
        None,
        true,
        &[],
    );
    let user = view.text_input(cx, ids!(user_id_input));
    let password = view.text_input(cx, ids!(password_input));
    let homeserver = view.text_input(cx, ids!(homeserver_input));
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        user.area(),
        3,
        Role::TextInput,
        "User ID",
        "Enter your Matrix user ID",
        Some(user.text()),
        true,
        &[Action::Focus, Action::SetValue],
    );
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        password.area(),
        4,
        Role::PasswordInput,
        "Password",
        "Enter your password",
        None,
        true,
        &[Action::Focus, Action::SetValue],
    );
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        homeserver.area(),
        5,
        Role::UrlInput,
        "Homeserver URL",
        "Optional; defaults to matrix.org",
        Some(homeserver.text()),
        true,
        &[Action::Focus, Action::SetValue],
    );
    let toggle_id = if password_visible {
        ids!(hide_password_button)
    } else {
        ids!(show_password_button)
    };
    let toggle_label = if password_visible {
        "Hide password"
    } else {
        "Show password"
    };
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        view.button(cx, toggle_id).area(),
        6,
        Role::Button,
        toggle_label,
        "",
        None,
        true,
        &[Action::Focus, Action::Click],
    );
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        view.button(cx, ids!(login_button)).area(),
        7,
        Role::DefaultButton,
        "Login",
        "Sign in with user ID and password",
        None,
        true,
        &[Action::Focus, Action::Click],
    );
    for (id, live_id, label) in [
        (8, ids!(apple_button), "Continue with Apple"),
        (9, ids!(facebook_button), "Continue with Facebook"),
        (10, ids!(github_button), "Continue with GitHub"),
        (11, ids!(gitlab_button), "Continue with GitLab"),
        (12, ids!(google_button), "Continue with Google"),
        (13, ids!(twitter_button), "Continue with X"),
    ] {
        push_node(
            &mut nodes,
            &mut children,
            &mut focus,
            cx,
            view.view(cx, live_id).area(),
            id,
            Role::Button,
            label,
            "Single sign-on",
            None,
            !sso_pending,
            &[Action::Focus, Action::Click],
        );
    }
    push_node(
        &mut nodes,
        &mut children,
        &mut focus,
        cx,
        view.button(cx, ids!(signup_button)).area(),
        14,
        Role::Button,
        "Sign up",
        "Open Matrix account creation",
        None,
        true,
        &[Action::Focus, Action::Click],
    );
    publish(cx, finish_tree(root, children, nodes, focus));
}

fn build_home_tree(
    bounds: Rect,
    controls: &[HomeControl],
    focused_control: Option<u64>,
    page_label: &str,
    context_label: &str,
) -> TreeUpdate {
    let mut root = Node::new(Role::Window);
    root.set_label("Hepta");
    root.set_bounds(bounds);
    let mut children = Vec::new();
    let mut nodes = Vec::new();
    let mut focus = ROOT_ID;

    for control in controls {
        push_bounded_node(
            &mut nodes,
            &mut children,
            &mut focus,
            control.bounds,
            control.id,
            Role::Button,
            control.label,
            "Primary navigation",
            None,
            true,
            &[Action::Focus, Action::Click],
            focused_control == Some(control.id),
        );
    }

    push_bounded_node(
        &mut nodes,
        &mut children,
        &mut focus,
        bounds,
        HOME_MAIN_ID,
        Role::Main,
        page_label,
        "Current Hepta workspace",
        None,
        true,
        &[],
        false,
    );
    push_bounded_node(
        &mut nodes,
        &mut children,
        &mut focus,
        bounds,
        HOME_CONTEXT_ID,
        Role::Heading,
        context_label,
        "Current view",
        None,
        true,
        &[],
        false,
    );

    finish_tree(root, children, nodes, focus)
}

pub(crate) fn publish_home_tree(
    cx: &mut Cx,
    view: &View,
    page_label: &str,
    context_label: &str,
    include_toggle_spaces: bool,
) {
    let Some(bounds) = area_bounds(cx, view.area()) else {
        return;
    };
    let candidates = [
        (
            HOME_ALL_ROOMS_ID,
            view.view(cx, ids!(navigation_tab_bar.home_button)).area(),
            "All rooms",
        ),
        (
            HOME_ADD_ROOM_ID,
            view.view(cx, ids!(navigation_tab_bar.add_room_button))
                .area(),
            "Add or join room",
        ),
        (
            HOME_SETTINGS_ID,
            view.view(cx, ids!(navigation_tab_bar.profile_icon)).area(),
            "Settings and profile",
        ),
        (
            HOME_TOGGLE_SPACES_ID,
            view.view(cx, ids!(navigation_tab_bar.toggle_spaces_bar_button))
                .area(),
            "Toggle spaces",
        ),
    ];
    let mut focused_control = None;
    let controls = candidates
        .into_iter()
        .filter(|(id, _, _)| include_toggle_spaces || *id != HOME_TOGGLE_SPACES_ID)
        .filter_map(|(id, area, label)| {
            let bounds = area_bounds(cx, area)?;
            if cx.has_key_focus(area) {
                focused_control = Some(id);
            }
            Some(HomeControl { id, bounds, label })
        })
        .collect::<Vec<_>>();
    publish(
        cx,
        build_home_tree(
            bounds,
            &controls,
            focused_control,
            page_label,
            context_label,
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(role: Role, label: &str, actions: &[Action]) -> Node {
        let mut node = Node::new(role);
        if !label.is_empty() {
            node.set_label(label);
        }
        node.set_bounds(Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 44.0,
            y1: 44.0,
        });
        for action in actions {
            node.add_action(*action);
        }
        node
    }

    fn update_with_child(child: Node) -> TreeUpdate {
        let mut root = node(Role::Window, "Hepta", &[]);
        root.set_children([NodeId(2)]);
        TreeUpdate {
            nodes: vec![(ROOT_ID, root), (NodeId(2), child)],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: ROOT_ID,
        }
    }

    #[test]
    fn semantic_tree_rejects_duplicate_ids_and_unlabeled_actions() {
        let mut update = update_with_child(node(Role::Button, "Login", &[Action::Click]));
        assert!(valid(&update));
        update
            .nodes
            .push((NodeId(2), node(Role::Button, "Duplicate", &[])));
        assert!(!valid(&update));
        update.nodes.pop();
        update.nodes[1].1.clear_label();
        assert!(!valid(&update));
    }

    #[test]
    fn semantic_tree_rejects_orphans_and_cycles() {
        let mut update = update_with_child(node(Role::Button, "Login", &[Action::Click]));
        update
            .nodes
            .push((NodeId(3), node(Role::Label, "Orphan", &[])));
        assert!(!valid(&update));

        update.nodes.pop();
        update.nodes[1].1.set_children([ROOT_ID]);
        assert!(!valid(&update));
    }

    #[test]
    fn password_nodes_never_expose_a_value() {
        let update = update_with_child(node(
            Role::PasswordInput,
            "Password",
            &[Action::Focus, Action::SetValue],
        ));
        assert!(valid(&update));
        let password = &update.nodes[1].1;
        assert_eq!(password.role(), Role::PasswordInput);
        assert_eq!(password.value(), None);
    }

    #[test]
    fn semantic_values_publish_for_plain_inputs_but_not_passwords() {
        let mut user = node(Role::TextInput, "User ID", &[Action::SetValue]);
        set_safe_value(
            &mut user,
            Role::TextInput,
            Some("@alice:example.org".into()),
        );
        assert_eq!(user.value(), Some("@alice:example.org"));

        let mut password = node(Role::PasswordInput, "Password", &[Action::SetValue]);
        set_safe_value(
            &mut password,
            Role::PasswordInput,
            Some("never-publish-this".into()),
        );
        assert_eq!(password.value(), None);
    }

    #[test]
    fn post_login_tree_is_never_a_single_root_node() {
        let bounds = Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 1280.0,
            y1: 800.0,
        };
        let controls = [HomeControl {
            id: HOME_ALL_ROOMS_ID,
            bounds,
            label: "All rooms",
        }];
        let update = build_home_tree(bounds, &controls, None, "Rooms", "General");
        assert!(valid(&update));
        assert_eq!(update.nodes.len(), 4);
        assert!(update.nodes.iter().any(|(id, node)| {
            *id == NodeId(HOME_ALL_ROOMS_ID)
                && node.label() == Some("All rooms")
                && node.supports_action(Action::Click)
        }));
        assert!(update.nodes.iter().any(|(id, node)| {
            *id == NodeId(HOME_MAIN_ID)
                && node.role() == Role::Main
                && node.label() == Some("Rooms")
        }));
        assert!(update.nodes.iter().any(|(id, node)| {
            *id == NodeId(HOME_CONTEXT_ID)
                && node.role() == Role::Heading
                && node.label() == Some("General")
        }));
    }

    #[test]
    fn post_login_tree_tracks_accessibility_focus() {
        let bounds = Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 320.0,
            y1: 640.0,
        };
        let controls = [HomeControl {
            id: HOME_SETTINGS_ID,
            bounds,
            label: "Settings and profile",
        }];
        let update = build_home_tree(
            bounds,
            &controls,
            Some(HOME_SETTINGS_ID),
            "Settings",
            "Settings",
        );
        assert!(valid(&update));
        assert_eq!(update.focus, NodeId(HOME_SETTINGS_ID));
    }
}

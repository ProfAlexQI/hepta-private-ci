//! Quarantined mention-popup selection model adapted from Robrix.
//!
//! The upstream widget relies on its redesigned MentionableTextInput, remote room ranking, avatar
//! fetches, and a global overlay owner. Hepta retains its existing cached, local suggestion path.
//! This module lands the upstream item vocabulary and keyboard-selection semantics without remote
//! lookup, avatar fetching, or message submission.

use std::sync::Arc;

use ruma::{OwnedMxcUri, OwnedRoomAliasId, OwnedUserId};

use crate::{shared::slash_commands::SlashCommand, utils::RoomNameId};

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "quarantined_selection_model_no_remote_lookup";

#[derive(Clone, Debug)]
pub struct RoomMentionCandidate {
    pub room_name_id: RoomNameId,
    pub alias: Option<OwnedRoomAliasId>,
    pub avatar_url: Option<OwnedMxcUri>,
    pub is_space: bool,
}

#[derive(Clone, Debug)]
pub enum MentionItem {
    User {
        user_id: OwnedUserId,
        display_name: String,
        avatar_url: Option<OwnedMxcUri>,
    },
    NotifyRoom {
        room_name: String,
    },
    Room(RoomMentionCandidate),
    Command(&'static SlashCommand),
}

impl MentionItem {
    /// Text inserted by the local composer after an explicit selection.
    pub fn insertion_text(&self) -> String {
        match self {
            Self::User { user_id, .. } => format!("{user_id} "),
            Self::NotifyRoom { .. } => "@room ".to_string(),
            Self::Room(candidate) => candidate.alias.as_ref().map_or_else(
                || format!("{} ", candidate.room_name_id.room_id()),
                |alias| format!("{alias} "),
            ),
            Self::Command(command) => format!("/{} ", command.name),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MentionPopupModel {
    items: Arc<Vec<MentionItem>>,
    selected_index: Option<usize>,
    pub is_loading: bool,
    pub empty_message: String,
}

impl MentionPopupModel {
    pub fn set_results(&mut self, items: Vec<MentionItem>, is_loading: bool) {
        self.items = Arc::new(items);
        self.is_loading = is_loading;
        self.selected_index = (!self.items.is_empty()).then_some(0);
    }

    pub fn items(&self) -> &[MentionItem] {
        self.items.as_slice()
    }

    pub fn selected(&self) -> Option<&MentionItem> {
        self.selected_index.and_then(|index| self.items.get(index))
    }

    pub fn move_selection(&mut self, delta: isize) {
        let count = self.items.len();
        if count == 0 {
            self.selected_index = None;
            return;
        }
        let current = self.selected_index.unwrap_or(0) as isize;
        self.selected_index = Some((current + delta).rem_euclid(count as isize) as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::slash_commands::SLASH_COMMANDS;

    #[test]
    fn robrix_intake_keyboard_selection_wraps_without_side_effects() {
        let mut model = MentionPopupModel::default();
        model.set_results(
            SLASH_COMMANDS.iter().map(MentionItem::Command).collect(),
            false,
        );

        assert_eq!(
            model.selected().map(MentionItem::insertion_text),
            Some("/html ".into())
        );
        model.move_selection(-1);
        assert_eq!(
            model.selected().map(MentionItem::insertion_text),
            Some("/plain ".into())
        );
    }

    #[test]
    fn robrix_intake_empty_results_have_no_selection() {
        let mut model = MentionPopupModel::default();
        model.set_results(Vec::new(), false);
        model.move_selection(1);
        assert!(model.selected().is_none());
    }
}

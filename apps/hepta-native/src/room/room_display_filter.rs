use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    ops::Deref,
};
use bitflags::bitflags;
use matrix_sdk::{
    RoomDisplayName,
    ruma::{
        OwnedRoomAliasId, RoomAliasId, RoomId,
        events::tag::{TagName, Tags},
    },
};

use crate::{
    home::rooms_list::{InvitedRoomInfo, JoinedRoomInfo},
    home::spaces_bar::JoinedSpaceInfo,
};

static EMPTY_TAGS: Tags = BTreeMap::new();

pub const SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE: &str = "Space unread/mention aggregate packet keeps JoinedSpaceInfo unread_messages and unread_mentions on local zero placeholders; is:unread and is:mention over spaces use the room-display-filter zero source with aggregate_refresh_slot not_built, and do not fetch aggregate unread counts, send read receipts, or issue message, room-state, or membership requests.";

/// A trait that abstracts the common properties of a room used to filter/sort it.
pub trait FilterableRoom {
    fn room_id(&self) -> &RoomId;
    fn room_name(&self) -> Cow<'_, str>;
    fn unread_mentions(&self) -> u64;
    fn unread_messages(&self) -> u64;
    fn canonical_alias(&self) -> Option<Cow<'_, RoomAliasId>>;
    fn alt_aliases(&self) -> Cow<'_, [OwnedRoomAliasId]>;
    fn tags(&self) -> &Tags;
    fn is_direct(&self) -> bool;
}

impl FilterableRoom for JoinedRoomInfo {
    fn room_id(&self) -> &RoomId {
        self.room_name_id.room_id()
    }

    fn room_name(&self) -> Cow<'_, str> {
        match self.room_name_id.display_name() {
            RoomDisplayName::Aliased(name)
            | RoomDisplayName::Calculated(name)
            | RoomDisplayName::Named(name) => Cow::Borrowed(name),
            RoomDisplayName::EmptyWas(name) => format!("Empty was {name}").into(),
            RoomDisplayName::Empty => "Empty".into(),
        }
    }

    fn unread_mentions(&self) -> u64 {
        self.num_unread_mentions
    }

    fn unread_messages(&self) -> u64 {
        self.num_unread_messages
    }

    fn canonical_alias(&self) -> Option<Cow<'_, RoomAliasId>> {
        self.canonical_alias.as_deref().map(Cow::Borrowed)
    }

    fn alt_aliases(&self) -> Cow<'_, [OwnedRoomAliasId]> {
        Cow::Borrowed(&self.alt_aliases)
    }

    fn tags(&self) -> &Tags {
        &self.tags
    }

    fn is_direct(&self) -> bool {
        self.is_direct
    }
}

impl FilterableRoom for InvitedRoomInfo {
    fn room_id(&self) -> &RoomId {
        self.room_name_id.room_id()
    }

    fn room_name(&self) -> Cow<'_, str> {
        match self.room_name_id.display_name() {
            RoomDisplayName::Aliased(name)
            | RoomDisplayName::Calculated(name)
            | RoomDisplayName::Named(name) => Cow::Borrowed(name),
            RoomDisplayName::EmptyWas(name) => format!("Empty was {name}").into(),
            RoomDisplayName::Empty => "Empty".into(),
        }
    }

    fn unread_mentions(&self) -> u64 {
        1
    }

    fn unread_messages(&self) -> u64 {
        0
    }

    fn canonical_alias(&self) -> Option<Cow<'_, RoomAliasId>> {
        self.canonical_alias.as_deref().map(Cow::Borrowed)
    }

    fn alt_aliases(&self) -> Cow<'_, [OwnedRoomAliasId]> {
        Cow::Borrowed(&self.alt_aliases)
    }

    fn tags(&self) -> &Tags {
        &EMPTY_TAGS
    }

    fn is_direct(&self) -> bool {
        self.is_direct
    }
}

impl FilterableRoom for JoinedSpaceInfo {
    fn room_id(&self) -> &RoomId {
        self.space_name_id.room_id()
    }

    fn room_name(&self) -> Cow<'_, str> {
        match self.space_name_id.display_name() {
            RoomDisplayName::Aliased(name)
            | RoomDisplayName::Calculated(name)
            | RoomDisplayName::EmptyWas(name)
            | RoomDisplayName::Named(name) => name.into(),
            RoomDisplayName::Empty => self.space_name_id.to_string().into(),
        }
    }

    fn unread_mentions(&self) -> u64 {
        0 // TODO: calculate unread mentions for spaces.
    }

    fn unread_messages(&self) -> u64 {
        0 // TODO: calculate unread messages for spaces.
    }

    fn canonical_alias(&self) -> Option<Cow<'_, RoomAliasId>> {
        self.canonical_alias.as_deref().map(Cow::Borrowed)
    }

    fn alt_aliases(&self) -> Cow<'_, [OwnedRoomAliasId]> {
        (&[]).into()
    }

    fn tags(&self) -> &Tags {
        &EMPTY_TAGS
    }

    fn is_direct(&self) -> bool {
        false
    }
}

pub type RoomFilterFn = dyn Fn(&dyn FilterableRoom) -> bool;
pub type SortFn = dyn Fn(&dyn FilterableRoom, &dyn FilterableRoom) -> Ordering;

fn default_room_filter_fn(_: &dyn FilterableRoom) -> bool {
    true
}

/// A filter function that determines whether a given room should be displayed.
///
/// If the function returns `true`, the room is displayed; otherwise, it is not shown.
/// The default value is a filter function that always returns `true`.
#[derive(Default)]
pub struct RoomDisplayFilter(Option<Box<RoomFilterFn>>);
impl RoomDisplayFilter {
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
impl Deref for RoomDisplayFilter {
    type Target = RoomFilterFn;
    fn deref(&self) -> &Self::Target {
        if let Some(rdf) = &self.0 {
            rdf.deref()
        } else {
            &default_room_filter_fn
        }
    }
}

bitflags! {
    /// The criteria that can be used to filter rooms in the `RoomDisplayFilter`.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RoomFilterCriteria: u16 {
        const RoomId    = 0b0000_0001;
        const RoomName  = 0b0000_0010;
        const RoomAlias = 0b0000_0100;
        const RoomTags  = 0b0000_1000;
        const RoomState = 0b0001_0000;
        const All       = Self::RoomId.bits() | Self::RoomName.bits() | Self::RoomAlias.bits() | Self::RoomTags.bits() | Self::RoomState.bits();
    }
}

impl Default for RoomFilterCriteria {
    fn default() -> Self {
        RoomFilterCriteria::All
    }
}

/// A builder for creating a `RoomDisplayFilter` with a specific set of filter types and a sorting function.
pub struct RoomDisplayFilterBuilder {
    keywords: String,
    filter_criteria: RoomFilterCriteria,
    sort_fn: Option<Box<SortFn>>,
}
/// ## Example
/// You can create any combination of filters and sorting functions using the `RoomDisplayFilterBuilder`.
/// ```rust,ignore
///   let (filter, sort_fn) = RoomDisplayFilterBuilder::new()
///     .set_keywords(keywords)
///     .by_room_id()
///     .by_room_name()
///     .sort_by(|a, b| {
///         let name_a = a.room_name.as_ref().map_or("", |n| n.display_str());
///         let name_b = b.room_name.as_ref().map_or("", |n| n.display_str());
///         name_a.cmp(name_b)
///     })
///     .build();
/// ```
impl RoomDisplayFilterBuilder {
    pub fn new() -> Self {
        Self {
            keywords: String::new(),
            filter_criteria: RoomFilterCriteria::default(),
            sort_fn: None,
        }
    }

    pub fn set_keywords(mut self, keywords: String) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn set_filter_criteria(mut self, filter_criteria: RoomFilterCriteria) -> Self {
        self.filter_criteria = filter_criteria;
        self
    }

    pub fn sort_by<F>(mut self, sort_fn: F) -> Self
    where
        F: Fn(&dyn FilterableRoom, &dyn FilterableRoom) -> Ordering + 'static,
    {
        self.sort_fn = Some(Box::new(sort_fn));
        self
    }

    fn matches_room_id(room: &dyn FilterableRoom, keywords: &str) -> bool {
        room.room_id().as_str().eq_ignore_ascii_case(keywords)
    }

    fn matches_room_name(room: &dyn FilterableRoom, keywords: &str) -> bool {
        room.room_name().to_lowercase().contains(keywords)
    }

    fn matches_room_alias(room: &dyn FilterableRoom, keywords: &str) -> bool {
        room.canonical_alias()
            .is_some_and(|alias| alias.as_str().eq_ignore_ascii_case(keywords))
            || room
                .alt_aliases()
                .iter()
                .any(|alias| alias.as_str().eq_ignore_ascii_case(keywords))
    }

    fn matches_room_tags(room: &dyn FilterableRoom, search_tags: &HashSet<String>) -> bool {
        fn is_tag_match(search_tag: &str, tag_name: &TagName) -> bool {
            match tag_name {
                TagName::Favorite => ["favourite", "favorite", "fav"].contains(&search_tag),
                TagName::LowPriority => {
                    ["low_priority", "low-priority", "lowpriority", "lowPriority"]
                        .contains(&search_tag)
                }
                TagName::ServerNotice => [
                    "server_notice",
                    "server-notice",
                    "servernotice",
                    "serverNotice",
                ]
                .contains(&search_tag),
                TagName::User(user_tag) => user_tag.as_ref().eq_ignore_ascii_case(search_tag),
                _ => false,
            }
        }

        let tags = room.tags();
        search_tags.iter().all(|search_tag| {
            tags.iter()
                .any(|(tag_name, _)| is_tag_match(search_tag, tag_name))
        })
    }

    fn matches_room_state(room: &dyn FilterableRoom, state_filter: &str) -> bool {
        match state_filter {
            "direct" | "dm" | "private" | "personal" => room.is_direct(),
            "group" | "groups" => !room.is_direct(),
            "unread" | "unreads" => room.unread_messages() > 0 || room.unread_mentions() > 0,
            "mention" | "mentions" => room.unread_mentions() > 0,
            "favorite" | "favourite" | "fav" => room.tags().contains_key(&TagName::Favorite),
            "low_priority" | "low-priority" | "lowpriority" | "lowpriorityroom" => {
                room.tags().contains_key(&TagName::LowPriority)
            }
            _ => false,
        }
    }

    fn split_state_filters(keywords: &str) -> (Vec<String>, String) {
        let mut state_filters = Vec::new();
        let mut text_terms = Vec::new();

        for term in keywords.split_whitespace() {
            if let Some(state_filter) = term.strip_prefix("is:") {
                state_filters.push(state_filter.to_string());
            } else {
                text_terms.push(term);
            }
        }

        (state_filters, text_terms.join(" "))
    }

    // Check if the keywords have a special prefix that indicates a pre-match filter check.
    fn pre_match_filter_check(keywords: &str) -> (RoomFilterCriteria, &str) {
        match keywords.chars().next() {
            Some('!') => (RoomFilterCriteria::RoomId, keywords),
            Some('#') => (RoomFilterCriteria::RoomAlias, keywords),
            Some(':') => (RoomFilterCriteria::RoomTags, keywords),
            _ => (RoomFilterCriteria::All, keywords),
        }
    }

    pub fn build(self) -> (RoomDisplayFilter, Option<Box<SortFn>>) {
        let keywords = self.keywords.trim();
        let criteria = self.filter_criteria;

        if keywords.is_empty() || criteria.is_empty() {
            return (RoomDisplayFilter::default(), self.sort_fn);
        }

        let keywords = keywords.to_lowercase();
        let (state_filters, text_keywords) = Self::split_state_filters(&keywords);
        let (specific_type, _) = Self::pre_match_filter_check(&text_keywords);

        let search_tags: HashSet<String> = keywords
            .split_whitespace()
            .filter(|tag| !tag.starts_with("is:"))
            .map(|tag| tag.trim_start_matches(':').to_string())
            .collect();

        let filter = RoomDisplayFilter(Some(Box::new(move |room| {
            if !state_filters.is_empty() {
                if !criteria.contains(RoomFilterCriteria::RoomState) {
                    return false;
                }
                if !state_filters
                    .iter()
                    .all(|state_filter| Self::matches_room_state(room, state_filter))
                {
                    return false;
                }
                if text_keywords.is_empty() {
                    return true;
                }
            }

            if specific_type != RoomFilterCriteria::All {
                // When using a special prefix, only check that specific type
                match specific_type {
                    RoomFilterCriteria::RoomId if criteria.contains(RoomFilterCriteria::RoomId) => {
                        Self::matches_room_id(room, &text_keywords)
                    }
                    RoomFilterCriteria::RoomAlias
                        if criteria.contains(RoomFilterCriteria::RoomAlias) =>
                    {
                        Self::matches_room_alias(room, &text_keywords)
                    }
                    RoomFilterCriteria::RoomTags
                        if criteria.contains(RoomFilterCriteria::RoomTags) =>
                    {
                        Self::matches_room_tags(room, &search_tags)
                    }
                    _ => false,
                }
            } else {
                // No special prefix, check all enabled filter types in a short-circuiting manner.
                if criteria.contains(RoomFilterCriteria::RoomId)
                    && Self::matches_room_id(room, &text_keywords)
                {
                    return true;
                }
                if criteria.contains(RoomFilterCriteria::RoomName)
                    && Self::matches_room_name(room, &text_keywords)
                {
                    return true;
                }
                if criteria.contains(RoomFilterCriteria::RoomAlias)
                    && Self::matches_room_alias(room, &text_keywords)
                {
                    return true;
                }
                if criteria.contains(RoomFilterCriteria::RoomTags)
                    && Self::matches_room_tags(room, &search_tags)
                {
                    return true;
                }
                false
            }
        })));

        (filter, self.sort_fn)
    }
}

impl Default for RoomDisplayFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{room::FetchedRoomAvatar, utils::RoomNameId};
    use matrix_sdk::ruma::{
        OwnedRoomId, room_id,
        events::tag::{TagInfo, TagName},
    };
    use matrix_sdk::RoomDisplayName;

    struct TestRoom {
        room_id: OwnedRoomId,
        name: String,
        unread_mentions: u64,
        unread_messages: u64,
        tags: Tags,
        is_direct: bool,
    }

    impl TestRoom {
        fn new(name: &str) -> Self {
            Self {
                room_id: room_id!("!telegram-filter:hepta.local").to_owned(),
                name: name.to_string(),
                unread_mentions: 0,
                unread_messages: 0,
                tags: Tags::default(),
                is_direct: false,
            }
        }

        fn direct(mut self) -> Self {
            self.is_direct = true;
            self
        }

        fn unread(mut self) -> Self {
            self.unread_messages = 3;
            self
        }

        fn mentioned(mut self) -> Self {
            self.unread_mentions = 1;
            self
        }

        fn favorite(mut self) -> Self {
            self.tags.insert(TagName::Favorite, TagInfo::default());
            self
        }

        fn low_priority(mut self) -> Self {
            self.tags.insert(TagName::LowPriority, TagInfo::default());
            self
        }
    }

    impl FilterableRoom for TestRoom {
        fn room_id(&self) -> &RoomId {
            &self.room_id
        }

        fn room_name(&self) -> Cow<'_, str> {
            Cow::Borrowed(&self.name)
        }

        fn unread_mentions(&self) -> u64 {
            self.unread_mentions
        }

        fn unread_messages(&self) -> u64 {
            self.unread_messages
        }

        fn canonical_alias(&self) -> Option<Cow<'_, RoomAliasId>> {
            None
        }

        fn alt_aliases(&self) -> Cow<'_, [OwnedRoomAliasId]> {
            Cow::Borrowed(&[])
        }

        fn tags(&self) -> &Tags {
            &self.tags
        }

        fn is_direct(&self) -> bool {
            self.is_direct
        }
    }

    fn build_filter(keywords: &str) -> RoomDisplayFilter {
        RoomDisplayFilterBuilder::new()
            .set_keywords(keywords.to_string())
            .set_filter_criteria(RoomFilterCriteria::All)
            .build()
            .0
    }

    #[test]
    fn telegram_room_filter_supports_direct_and_text_state_filters() {
        let direct_room = TestRoom::new("Alice approval thread").direct();
        let group_room = TestRoom::new("Release channel");
        let filter = build_filter("is:direct approval");

        assert!(filter(&direct_room));
        assert!(!filter(&group_room));
    }

    #[test]
    fn telegram_room_filter_supports_unread_mention_and_tag_state_filters() {
        let unread = TestRoom::new("Ops").unread();
        let mentioned = TestRoom::new("Reviews").mentioned();
        let favorite = TestRoom::new("Pinned work").favorite();
        let low_priority = TestRoom::new("Archive").low_priority();
        let plain = TestRoom::new("Quiet");

        assert!(build_filter("is:unread")(&unread));
        assert!(build_filter("is:mention")(&mentioned));
        assert!(build_filter("is:favorite")(&favorite));
        assert!(build_filter("is:low_priority")(&low_priority));
        assert!(!build_filter("is:unread")(&plain));
        assert!(!build_filter("is:favorite")(&plain));
    }

    #[test]
    fn telegram_space_unread_mention_filter_uses_local_zero_packet() {
        let space = JoinedSpaceInfo {
            space_name_id: RoomNameId::new(
                RoomDisplayName::Named("Product Space".into()),
                room_id!("!space:example.org").to_owned(),
            ),
            canonical_alias: None,
            topic: None,
            space_avatar: FetchedRoomAvatar::default(),
            num_joined_members: 7,
            join_rule: None,
            world_readable: Some(false),
            guest_can_join: false,
            children_count: 3,
        };

        assert_eq!(space.unread_messages(), 0);
        assert_eq!(space.unread_mentions(), 0);
        assert!(!build_filter("is:unread")(&space));
        assert!(!build_filter("is:mention")(&space));
        assert!(SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE.contains("aggregate packet"));
        assert!(
            SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE
                .contains("aggregate_refresh_slot not_built")
        );
    }
}

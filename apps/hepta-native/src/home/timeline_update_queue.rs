//! Bounded, non-blocking delivery of timeline updates to the Makepad UI thread.
//!
//! Timeline producers run on Tokio tasks while the consumer runs synchronously on the UI
//! thread. Blocking either side is therefore unsafe. The queue keeps critical transitions in a
//! dedicated bounded FIFO and stores high-rate state snapshots in a bounded set of coalescing
//! slots. Non-critical traffic can never consume capacity reserved for terminal, membership, or
//! authority-relevant updates.

use std::{
    collections::{HashMap, VecDeque},
    fmt,
    sync::{Arc, Mutex},
};

use super::room_screen::TimelineUpdate;
use crate::shared::file_upload_modal::FileUploadAttemptId;

/// Critical transitions waiting for one timeline UI.
pub(crate) const TIMELINE_CRITICAL_CAPACITY: usize = 256;
/// Critical transitions retained after the primary FIFO reaches capacity.
pub(crate) const TIMELINE_CRITICAL_RETRY_CAPACITY: usize = 256;
/// Distinct latest-value streams waiting for one timeline UI.
pub(crate) const TIMELINE_COALESCED_CAPACITY: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum CoalescingKey {
    TimelineItems,
    UnreadCount,
    TypingUsers,
    PinnedEvents,
    OwnReadReceipt,
    FileUploadProgress(FileUploadAttemptId),
    LinkPreview,
}

fn coalescing_key(update: &TimelineUpdate) -> Option<CoalescingKey> {
    match update {
        TimelineUpdate::NewItems { .. } => Some(CoalescingKey::TimelineItems),
        TimelineUpdate::NewUnreadMessagesCount(_) => Some(CoalescingKey::UnreadCount),
        TimelineUpdate::TypingUsers { .. } => Some(CoalescingKey::TypingUsers),
        TimelineUpdate::PinnedEvents(_) => Some(CoalescingKey::PinnedEvents),
        TimelineUpdate::OwnUserReadReceipt(_) => Some(CoalescingKey::OwnReadReceipt),
        TimelineUpdate::FileUploadUpdate { upload_id, .. } => {
            Some(CoalescingKey::FileUploadProgress(*upload_id))
        }
        TimelineUpdate::LinkPreviewFetched => Some(CoalescingKey::LinkPreview),
        // First/terminal results, membership, encryption, power, tombstone, target-event,
        // and transfer lifecycle updates must stay as individually ordered transitions.
        _ => None,
    }
}

/// Merge a newer latest-value update into an existing slot.
fn merge_coalesced(existing: TimelineUpdate, incoming: TimelineUpdate) -> TimelineUpdate {
    match (existing, incoming) {
        (TimelineUpdate::NewItems { .. }, TimelineUpdate::NewItems { new_items, .. }) => {
            // Folding diffs across snapshots cannot safely union index ranges: the latest
            // timeline may be shorter than an older snapshot. Treat the latest value as an exact
            // cache-clearing snapshot so `changed_indices` is always in bounds.
            let changed_indices = 0..new_items.len();
            TimelineUpdate::NewItems {
                new_items,
                changed_indices,
                is_append: false,
                clear_cache: true,
            }
        }
        (_, incoming) => incoming,
    }
}

struct SequencedUpdate {
    sequence: u64,
    update: TimelineUpdate,
}

struct QueueState {
    next_sequence: u64,
    critical: VecDeque<SequencedUpdate>,
    /// Once non-empty, later critical updates join this FIFO until it is drained. This prevents
    /// a later update from entering the primary lane and overtaking an earlier deferred update.
    critical_retry: VecDeque<SequencedUpdate>,
    coalesced: HashMap<CoalescingKey, SequencedUpdate>,
    sender_count: usize,
    receiver_alive: bool,
    /// A coalesced latest-state producer could not enqueue an update, so the UI must request an
    /// exact snapshot. Critical loss uses the separate fatal `delivery_lost` marker.
    resync_needed: bool,
    /// Both critical FIFOs filled. At least one non-reconstructable transition was rejected, so
    /// this transport generation is no longer trustworthy and must be destroyed and rebuilt.
    delivery_lost: bool,
}

struct SharedQueue {
    state: Mutex<QueueState>,
    critical_capacity: usize,
    critical_retry_capacity: usize,
    coalesced_capacity: usize,
}

/// Outcome of a non-blocking timeline update submission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimelineUpdateSendError {
    /// The UI has dropped its endpoint for this timeline.
    Closed,
    /// A coalesced latest-state lane is full. The producer must retry or request a resync.
    Backpressured,
    /// A non-reconstructable critical transition could not be retained. This transport
    /// generation is fenced until the UI destroys it and creates a fresh channel.
    DeliveryLost,
}

impl fmt::Display for TimelineUpdateSendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed => f.write_str("timeline update receiver is closed"),
            Self::Backpressured => f.write_str("timeline update queue is full"),
            Self::DeliveryLost => f.write_str("timeline update delivery was lost"),
        }
    }
}

/// Result of polling the synchronous UI endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimelineUpdateTryRecvError {
    Empty,
    Disconnected,
}

/// Cloneable endpoint used by async timeline producers.
pub struct TimelineUpdateSender {
    shared: Arc<SharedQueue>,
}

impl Clone for TimelineUpdateSender {
    fn clone(&self) -> Self {
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        state.sender_count = state.sender_count.saturating_add(1);
        drop(state);
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl Drop for TimelineUpdateSender {
    fn drop(&mut self) {
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        state.sender_count = state.sender_count.saturating_sub(1);
    }
}

impl TimelineUpdateSender {
    /// Submit without blocking a Tokio worker or the Makepad UI thread.
    pub(crate) fn send(&self, update: TimelineUpdate) -> Result<(), TimelineUpdateSendError> {
        let key = coalescing_key(&update);
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        if !state.receiver_alive {
            return Err(TimelineUpdateSendError::Closed);
        }
        if state.delivery_lost {
            return Err(TimelineUpdateSendError::DeliveryLost);
        }

        if let Some(key) = key {
            if state.coalesced.contains_key(&key) {
                if let Some(existing) = state.coalesced.get_mut(&key) {
                    let placeholder = TimelineUpdate::LinkPreviewFetched;
                    let old = std::mem::replace(&mut existing.update, placeholder);
                    existing.update = merge_coalesced(old, update);
                    // Preserve the slot's original queue position. Moving a conflated NewItems
                    // slot behind a later TargetEventFound transition would let the UI observe
                    // the target before the authoritative items snapshot needed to locate it.
                    return Ok(());
                }
            }
            if state.coalesced.len() >= self.shared.coalesced_capacity {
                state.resync_needed = true;
                return Err(TimelineUpdateSendError::Backpressured);
            }
            let sequence = state.next_sequence;
            state.next_sequence = state.next_sequence.wrapping_add(1);
            state
                .coalesced
                .insert(key, SequencedUpdate { sequence, update });
            return Ok(());
        }

        let sequence = state.next_sequence;
        state.next_sequence = state.next_sequence.wrapping_add(1);

        // Preserve critical FIFO ordering across both bounded lanes. Once one update is
        // deferred, every later critical update joins the retry FIFO until the receiver drains
        // it. This is still fully non-blocking for Tokio producers.
        if !state.critical_retry.is_empty() || state.critical.len() >= self.shared.critical_capacity
        {
            if state.critical_retry.len() >= self.shared.critical_retry_capacity {
                state.delivery_lost = true;
                return Err(TimelineUpdateSendError::DeliveryLost);
            }
            state
                .critical_retry
                .push_back(SequencedUpdate { sequence, update });
        } else {
            state
                .critical
                .push_back(SequencedUpdate { sequence, update });
        }
        Ok(())
    }
}

/// Singleton UI endpoint for one room or thread timeline.
pub struct TimelineUpdateReceiver {
    shared: Arc<SharedQueue>,
}

impl Drop for TimelineUpdateReceiver {
    fn drop(&mut self) {
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        state.receiver_alive = false;
        state.critical.clear();
        state.critical_retry.clear();
        state.coalesced.clear();
    }
}

impl TimelineUpdateReceiver {
    pub(crate) fn try_recv(&self) -> Result<TimelineUpdate, TimelineUpdateTryRecvError> {
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());

        let critical_sequence = state.critical.front().map(|entry| entry.sequence);
        let critical_retry_sequence = state.critical_retry.front().map(|entry| entry.sequence);
        let coalesced_key = state
            .coalesced
            .iter()
            .min_by_key(|(_, entry)| entry.sequence)
            .map(|(key, _)| key.clone());
        let coalesced_sequence = coalesced_key
            .as_ref()
            .and_then(|key| state.coalesced.get(key))
            .map(|entry| entry.sequence);

        let next_sequence = [
            critical_sequence,
            critical_retry_sequence,
            coalesced_sequence,
        ]
        .into_iter()
        .flatten()
        .min();
        let update = if next_sequence == critical_sequence {
            state.critical.pop_front().map(|entry| entry.update)
        } else if next_sequence == critical_retry_sequence {
            state.critical_retry.pop_front().map(|entry| entry.update)
        } else if next_sequence == coalesced_sequence {
            coalesced_key
                .and_then(|key| state.coalesced.remove(&key))
                .map(|entry| entry.update)
        } else {
            None
        };

        update.ok_or_else(|| {
            if state.sender_count == 0 {
                TimelineUpdateTryRecvError::Disconnected
            } else {
                TimelineUpdateTryRecvError::Empty
            }
        })
    }

    pub(crate) fn has_pending(&self) -> bool {
        let state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        !state.critical.is_empty()
            || !state.critical_retry.is_empty()
            || !state.coalesced.is_empty()
    }

    /// Atomically consume the recovery marker set by any rejected producer.
    ///
    /// RoomScreen turns this into a new `TimelineRequest::resync_generation`. If the resulting
    /// exact snapshot is itself rejected, the sender sets this marker again.
    pub(crate) fn take_resync_needed(&self) -> bool {
        let mut state = self.shared.state.lock().unwrap_or_else(|p| p.into_inner());
        std::mem::take(&mut state.resync_needed)
    }

    /// Whether this transport generation rejected a non-reconstructable critical transition.
    ///
    /// This is deliberately sticky. It is cleared only by destroying this receiver and creating
    /// a fresh channel, which fences every producer still holding an old-generation sender.
    pub(crate) fn delivery_lost(&self) -> bool {
        self.shared
            .state
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .delivery_lost
    }
}

pub(crate) fn timeline_update_channel() -> (TimelineUpdateSender, TimelineUpdateReceiver) {
    timeline_update_channel_with_limits(
        TIMELINE_CRITICAL_CAPACITY,
        TIMELINE_CRITICAL_RETRY_CAPACITY,
        TIMELINE_COALESCED_CAPACITY,
    )
}

fn timeline_update_channel_with_limits(
    critical_capacity: usize,
    critical_retry_capacity: usize,
    coalesced_capacity: usize,
) -> (TimelineUpdateSender, TimelineUpdateReceiver) {
    assert!(critical_capacity > 0);
    assert!(critical_retry_capacity > 0);
    assert!(coalesced_capacity > 0);
    let shared = Arc::new(SharedQueue {
        state: Mutex::new(QueueState {
            next_sequence: 0,
            critical: VecDeque::with_capacity(critical_capacity),
            critical_retry: VecDeque::with_capacity(critical_retry_capacity),
            coalesced: HashMap::with_capacity(coalesced_capacity),
            sender_count: 1,
            receiver_alive: true,
            resync_needed: false,
            delivery_lost: false,
        }),
        critical_capacity,
        critical_retry_capacity,
        coalesced_capacity,
    });
    (
        TimelineUpdateSender {
            shared: shared.clone(),
        },
        TimelineUpdateReceiver { shared },
    )
}

#[cfg(test)]
mod tests {
    use imbl::Vector;

    use super::*;

    #[test]
    fn ten_thousand_ephemeral_updates_use_one_slot_and_keep_latest() {
        let (sender, receiver) = timeline_update_channel_with_limits(4, 4, 2);
        for i in 0..10_000 {
            sender
                .send(TimelineUpdate::TypingUsers {
                    users: vec![format!("user-{i}")],
                })
                .unwrap();
        }

        match receiver.try_recv().unwrap() {
            TimelineUpdate::TypingUsers { users } => {
                assert_eq!(users, vec!["user-9999".to_owned()]);
            }
            _ => panic!("expected coalesced typing update"),
        }
        assert!(matches!(
            receiver.try_recv(),
            Err(TimelineUpdateTryRecvError::Empty)
        ));
    }

    #[test]
    fn critical_updates_are_not_displaced_by_noncritical_flood() {
        let (sender, receiver) = timeline_update_channel_with_limits(4, 4, 2);
        sender
            .send(TimelineUpdate::NewItems {
                new_items: Vector::new(),
                changed_indices: 0..0,
                is_append: true,
                clear_cache: false,
            })
            .unwrap();
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        for i in 0..10_000 {
            sender
                .send(TimelineUpdate::TypingUsers {
                    users: vec![i.to_string()],
                })
                .unwrap();
        }
        sender
            .send(TimelineUpdate::FileUploadComplete { upload_id: 7 })
            .unwrap();

        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::NewItems { .. })
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::TypingUsers { .. })
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::FileUploadComplete { upload_id: 7 })
        ));
    }

    #[test]
    fn replacing_a_coalesced_snapshot_does_not_cross_a_later_critical_transition() {
        let (sender, receiver) = timeline_update_channel_with_limits(2, 2, 1);
        sender
            .send(TimelineUpdate::NewItems {
                new_items: Vector::new(),
                changed_indices: 0..0,
                is_append: true,
                clear_cache: false,
            })
            .unwrap();
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        sender
            .send(TimelineUpdate::NewItems {
                new_items: Vector::new(),
                changed_indices: 0..0,
                is_append: false,
                clear_cache: true,
            })
            .unwrap();

        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::NewItems {
                clear_cache: true,
                is_append: false,
                ..
            })
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
    }

    #[test]
    fn coalesced_new_items_shrink_is_exact_and_never_keeps_an_old_end_index() {
        let old = TimelineUpdate::NewItems {
            // The old metadata represents a longer previous snapshot.
            new_items: Vector::new(),
            changed_indices: 0..17,
            is_append: true,
            clear_cache: false,
        };
        let latest = TimelineUpdate::NewItems {
            // Shrink all the way to empty.
            new_items: Vector::new(),
            changed_indices: 0..0,
            is_append: false,
            clear_cache: false,
        };

        assert!(matches!(
            merge_coalesced(old, latest),
            TimelineUpdate::NewItems {
                new_items,
                changed_indices,
                clear_cache: true,
                is_append: false,
            } if new_items.is_empty() && changed_indices == (0..0)
        ));
    }

    #[test]
    fn critical_primary_and_retry_fifos_preserve_order() {
        let (sender, receiver) = timeline_update_channel_with_limits(2, 2, 1);
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        sender.send(TimelineUpdate::RoomEncrypted).unwrap();
        sender
            .send(TimelineUpdate::FileUploadComplete { upload_id: 1 })
            .unwrap();
        sender
            .send(TimelineUpdate::FileUploadComplete { upload_id: 2 })
            .unwrap();

        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomEncrypted)
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::FileUploadComplete { upload_id: 1 })
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::FileUploadComplete { upload_id: 2 })
        ));
        assert!(!receiver.delivery_lost());
    }

    #[test]
    fn coalesced_slot_overflow_is_explicit_and_cannot_consume_critical_capacity() {
        let (sender, receiver) = timeline_update_channel_with_limits(1, 1, 1);
        sender
            .send(TimelineUpdate::TypingUsers {
                users: vec!["typing".to_owned()],
            })
            .unwrap();
        assert_eq!(
            sender.send(TimelineUpdate::LinkPreviewFetched),
            Err(TimelineUpdateSendError::Backpressured),
        );
        assert!(receiver.take_resync_needed());
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();

        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::TypingUsers { .. })
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
    }

    #[test]
    fn overflow_marker_recovers_with_a_later_exact_snapshot() {
        let (sender, receiver) = timeline_update_channel_with_limits(1, 1, 1);
        sender
            .send(TimelineUpdate::TypingUsers {
                users: vec!["typing".to_owned()],
            })
            .unwrap();
        assert_eq!(
            sender.send(TimelineUpdate::LinkPreviewFetched),
            Err(TimelineUpdateSendError::Backpressured),
        );
        assert!(receiver.take_resync_needed());
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::TypingUsers { .. })
        ));

        // This is the same cache-clearing exact update emitted by the subscriber after
        // RoomScreen increments TimelineRequest::resync_generation.
        sender
            .send(TimelineUpdate::NewItems {
                new_items: Vector::new(),
                changed_indices: 0..0,
                is_append: false,
                clear_cache: true,
            })
            .unwrap();
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::NewItems {
                clear_cache: true,
                ..
            })
        ));
        assert!(!receiver.take_resync_needed());
    }

    #[test]
    fn receiver_close_never_panics_producers() {
        let (sender, receiver) = timeline_update_channel_with_limits(2, 2, 1);
        drop(receiver);
        assert_eq!(
            sender.send(TimelineUpdate::RoomMembersSynced),
            Err(TimelineUpdateSendError::Closed),
        );
    }

    #[test]
    fn sender_disconnect_is_reported_without_panic() {
        let (sender, receiver) = timeline_update_channel_with_limits(2, 2, 1);
        drop(sender);
        assert!(matches!(
            receiver.try_recv(),
            Err(TimelineUpdateTryRecvError::Disconnected),
        ));
    }

    #[test]
    fn both_critical_layers_full_sets_sticky_delivery_lost() {
        let (sender, receiver) = timeline_update_channel_with_limits(1, 1, 1);
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        sender.send(TimelineUpdate::RoomEncrypted).unwrap();
        assert_eq!(
            sender.send(TimelineUpdate::FileUploadComplete { upload_id: 7 }),
            Err(TimelineUpdateSendError::DeliveryLost),
        );
        assert!(receiver.delivery_lost());
        assert!(!receiver.take_resync_needed());
        assert_eq!(
            sender.send(TimelineUpdate::RoomMembersSynced),
            Err(TimelineUpdateSendError::DeliveryLost),
        );
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
        assert!(matches!(
            receiver.try_recv(),
            Ok(TimelineUpdate::RoomEncrypted)
        ));
        assert!(receiver.delivery_lost());
    }

    #[test]
    fn rebuilt_generation_starts_clean_and_fences_old_senders() {
        let (old_sender, old_receiver) = timeline_update_channel_with_limits(1, 1, 1);
        old_sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        old_sender.send(TimelineUpdate::RoomEncrypted).unwrap();
        assert_eq!(
            old_sender.send(TimelineUpdate::FileUploadComplete { upload_id: 7 }),
            Err(TimelineUpdateSendError::DeliveryLost),
        );
        assert!(old_receiver.delivery_lost());

        // RoomScreen destroys the failed receiver before the backend installs a fresh channel.
        drop(old_receiver);
        assert_eq!(
            old_sender.send(TimelineUpdate::RoomMembersSynced),
            Err(TimelineUpdateSendError::Closed),
        );

        let (new_sender, new_receiver) = timeline_update_channel_with_limits(1, 1, 1);
        assert!(!new_receiver.delivery_lost());
        assert!(!new_receiver.take_resync_needed());
        new_sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        assert!(matches!(
            new_receiver.try_recv(),
            Ok(TimelineUpdate::RoomMembersSynced)
        ));
    }

    #[test]
    fn receiver_close_after_delivery_loss_is_still_nonblocking_and_explicit() {
        let (sender, receiver) = timeline_update_channel_with_limits(1, 1, 1);
        sender.send(TimelineUpdate::RoomMembersSynced).unwrap();
        sender.send(TimelineUpdate::RoomEncrypted).unwrap();
        assert_eq!(
            sender.send(TimelineUpdate::FileUploadComplete { upload_id: 1 }),
            Err(TimelineUpdateSendError::DeliveryLost),
        );
        assert!(receiver.delivery_lost());
        drop(receiver);
        assert_eq!(
            sender.send(TimelineUpdate::RoomEncrypted),
            Err(TimelineUpdateSendError::Closed)
        );
    }
}

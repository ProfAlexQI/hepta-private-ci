//! Bounded request transport between the synchronous Makepad UI and the Matrix Tokio worker.
//!
//! User actions and lifecycle transitions have a capacity-reserved critical lane. Regular work
//! has its own bounded lane, while high-rate latest-value requests are kept in coalescing slots.
//! This prevents typing/read-count traffic from starving login, room membership, message sends,
//! cancellation, or other user-visible state transitions.

use std::{
    collections::{HashMap, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use matrix_sdk::ruma::OwnedRoomId;
use tokio::sync::{Notify, mpsc};

use super::{MatrixRequest, TimelineKind};

pub(super) const MATRIX_CRITICAL_REQUEST_CAPACITY: usize = 64;
/// Critical requests that arrived while the primary lane was full.
pub(super) const MATRIX_CRITICAL_RETRY_CAPACITY: usize = 64;
pub(super) const MATRIX_REGULAR_REQUEST_CAPACITY: usize = 256;
pub(super) const MATRIX_COALESCED_REQUEST_CAPACITY: usize = 128;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum CoalescingKey {
    Typing(OwnedRoomId),
    UnreadCount(TimelineKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RequestLane {
    Critical,
    Regular,
}

fn coalescing_key(request: &MatrixRequest) -> Option<CoalescingKey> {
    match request {
        MatrixRequest::SendTypingNotice { room_id, .. } => {
            Some(CoalescingKey::Typing(room_id.clone()))
        }
        MatrixRequest::GetNumberUnreadMessages { timeline_kind } => {
            Some(CoalescingKey::UnreadCount(timeline_kind.clone()))
        }
        _ => None,
    }
}

fn request_lane(request: &MatrixRequest) -> RequestLane {
    match request {
        MatrixRequest::Login(_)
        | MatrixRequest::Logout { .. }
        | MatrixRequest::PaginateTimeline { .. }
        | MatrixRequest::EditMessage { .. }
        | MatrixRequest::SyncRoomMemberList { .. }
        | MatrixRequest::CreateThreadTimeline { .. }
        | MatrixRequest::CloseThreadTimeline { .. }
        | MatrixRequest::Knock { .. }
        | MatrixRequest::InviteUser { .. }
        | MatrixRequest::JoinRoom { .. }
        | MatrixRequest::LeaveRoom { .. }
        | MatrixRequest::GetRoomMembers { .. }
        | MatrixRequest::GetSuccessorRoomDetails { .. }
        | MatrixRequest::OpenOrCreateDirectMessage { .. }
        | MatrixRequest::SetUnreadFlag { .. }
        | MatrixRequest::SetIsFavorite { .. }
        | MatrixRequest::SetIsLowPriority { .. }
        | MatrixRequest::IgnoreUser { .. }
        | MatrixRequest::SetAvatar { .. }
        | MatrixRequest::SetDisplayName { .. }
        | MatrixRequest::RequestSelfVerification
        | MatrixRequest::SendMessage { .. }
        | MatrixRequest::SendAttachment { .. }
        | MatrixRequest::SpawnSSOServer { .. }
        | MatrixRequest::SubscribeToTypingNotices { .. }
        | MatrixRequest::SubscribeToOwnUserReadReceiptsChanged { .. }
        | MatrixRequest::SubscribeToPinnedEvents { .. }
        | MatrixRequest::ReadReceipt { .. }
        | MatrixRequest::GetRoomPowerLevels { .. }
        | MatrixRequest::GenerateMatrixLink { .. }
        | MatrixRequest::ToggleReaction { .. }
        | MatrixRequest::RedactMessage { .. }
        | MatrixRequest::PinEvent { .. }
        | MatrixRequest::DownloadMedia { .. }
        | MatrixRequest::CancelDownload(_) => RequestLane::Critical,
        _ => RequestLane::Regular,
    }
}

pub(super) fn is_critical_request(request: &MatrixRequest) -> bool {
    request_lane(request) == RequestLane::Critical
}

struct BufferedState {
    /// Once this contains an item, later critical requests join it so FIFO order is preserved
    /// across the primary channel and the retry lane.
    critical_retry: VecDeque<MatrixRequest>,
    requests: HashMap<CoalescingKey, MatrixRequest>,
}

struct SharedState {
    buffered: Mutex<BufferedState>,
    notify: Notify,
    receiver_alive: AtomicBool,
    critical_retry_capacity: usize,
    coalesced_capacity: usize,
}

/// Successful submission disposition, exposed for diagnostics and focused tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MatrixRequestSubmitDisposition {
    Enqueued,
    /// The primary critical lane was full, but the action is retained in bounded FIFO storage.
    Deferred,
    Coalesced,
}

/// Explicit non-blocking backpressure result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MatrixRequestSubmitError {
    Closed,
    /// Both capacity-reserved critical lanes are full. The caller must visibly reject/retry.
    CriticalRejected,
    NonCriticalBackpressured,
}

#[derive(Clone)]
pub(super) struct MatrixRequestSender {
    critical: mpsc::Sender<MatrixRequest>,
    regular: mpsc::Sender<MatrixRequest>,
    shared: Arc<SharedState>,
}

impl MatrixRequestSender {
    pub(super) fn same_channel(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }

    pub(super) fn try_submit(
        &self,
        request: MatrixRequest,
    ) -> Result<MatrixRequestSubmitDisposition, MatrixRequestSubmitError> {
        if !self.shared.receiver_alive.load(Ordering::Acquire) {
            return Err(MatrixRequestSubmitError::Closed);
        }

        if let Some(key) = coalescing_key(&request) {
            let mut state = self
                .shared
                .buffered
                .lock()
                .unwrap_or_else(|p| p.into_inner());
            if !self.shared.receiver_alive.load(Ordering::Acquire) {
                return Err(MatrixRequestSubmitError::Closed);
            }
            if !state.requests.contains_key(&key)
                && state.requests.len() >= self.shared.coalesced_capacity
            {
                return Err(MatrixRequestSubmitError::NonCriticalBackpressured);
            }
            state.requests.insert(key, request);
            drop(state);
            self.shared.notify.notify_one();
            return Ok(MatrixRequestSubmitDisposition::Coalesced);
        }

        let lane = request_lane(&request);
        if lane == RequestLane::Critical {
            let mut state = self
                .shared
                .buffered
                .lock()
                .unwrap_or_else(|p| p.into_inner());
            if !self.shared.receiver_alive.load(Ordering::Acquire) {
                return Err(MatrixRequestSubmitError::Closed);
            }

            // Preserve critical FIFO ordering: after one request is deferred, every later
            // critical request joins the same retry lane until the receiver drains it.
            if !state.critical_retry.is_empty() {
                if state.critical_retry.len() >= self.shared.critical_retry_capacity {
                    return Err(MatrixRequestSubmitError::CriticalRejected);
                }
                state.critical_retry.push_back(request);
                drop(state);
                self.shared.notify.notify_one();
                return Ok(MatrixRequestSubmitDisposition::Deferred);
            }

            match self.critical.try_send(request) {
                Ok(()) => return Ok(MatrixRequestSubmitDisposition::Enqueued),
                Err(mpsc::error::TrySendError::Full(request)) => {
                    if state.critical_retry.len() >= self.shared.critical_retry_capacity {
                        return Err(MatrixRequestSubmitError::CriticalRejected);
                    }
                    state.critical_retry.push_back(request);
                    drop(state);
                    self.shared.notify.notify_one();
                    return Ok(MatrixRequestSubmitDisposition::Deferred);
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    return Err(MatrixRequestSubmitError::Closed);
                }
            }
        }

        let sender = match lane {
            RequestLane::Critical => unreachable!("critical requests returned above"),
            RequestLane::Regular => &self.regular,
        };
        match sender.try_send(request) {
            Ok(()) => Ok(MatrixRequestSubmitDisposition::Enqueued),
            Err(mpsc::error::TrySendError::Full(_)) => {
                Err(MatrixRequestSubmitError::NonCriticalBackpressured)
            }
            Err(mpsc::error::TrySendError::Closed(_)) => Err(MatrixRequestSubmitError::Closed),
        }
    }
}

pub(super) struct MatrixRequestReceiver {
    critical: mpsc::Receiver<MatrixRequest>,
    regular: mpsc::Receiver<MatrixRequest>,
    shared: Arc<SharedState>,
    critical_closed: bool,
    regular_closed: bool,
}

impl Drop for MatrixRequestReceiver {
    fn drop(&mut self) {
        self.shared.receiver_alive.store(false, Ordering::Release);
        let mut state = self
            .shared
            .buffered
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        state.critical_retry.clear();
        state.requests.clear();
        drop(state);
        self.shared.notify.notify_waiters();
    }
}

impl MatrixRequestReceiver {
    fn take_coalesced(&self) -> Option<MatrixRequest> {
        let mut state = self
            .shared
            .buffered
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        let key = state.requests.keys().next().cloned()?;
        state.requests.remove(&key)
    }

    fn take_critical_retry(&self) -> Option<MatrixRequest> {
        self.shared
            .buffered
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .critical_retry
            .pop_front()
    }

    /// Receive with strict preference for capacity-reserved critical requests.
    pub(super) async fn recv(&mut self) -> Option<MatrixRequest> {
        loop {
            if !self.critical_closed {
                match self.critical.try_recv() {
                    Ok(request) => return Some(request),
                    Err(mpsc::error::TryRecvError::Disconnected) => self.critical_closed = true,
                    Err(mpsc::error::TryRecvError::Empty) => {}
                }
            }
            if let Some(request) = self.take_critical_retry() {
                return Some(request);
            }
            if let Some(request) = self.take_coalesced() {
                return Some(request);
            }
            if !self.regular_closed {
                match self.regular.try_recv() {
                    Ok(request) => return Some(request),
                    Err(mpsc::error::TryRecvError::Disconnected) => self.regular_closed = true,
                    Err(mpsc::error::TryRecvError::Empty) => {}
                }
            }
            if self.critical_closed && self.regular_closed {
                return None;
            }

            let shared = self.shared.clone();
            tokio::select! {
                biased;
                request = self.critical.recv(), if !self.critical_closed => {
                    match request {
                        Some(request) => return Some(request),
                        None => self.critical_closed = true,
                    }
                }
                _ = shared.notify.notified() => {}
                request = self.regular.recv(), if !self.regular_closed => {
                    match request {
                        Some(request) => return Some(request),
                        None => self.regular_closed = true,
                    }
                }
            }
        }
    }
}

pub(super) fn matrix_request_channel() -> (MatrixRequestSender, MatrixRequestReceiver) {
    matrix_request_channel_with_limits(
        MATRIX_CRITICAL_REQUEST_CAPACITY,
        MATRIX_CRITICAL_RETRY_CAPACITY,
        MATRIX_REGULAR_REQUEST_CAPACITY,
        MATRIX_COALESCED_REQUEST_CAPACITY,
    )
}

fn matrix_request_channel_with_limits(
    critical_capacity: usize,
    critical_retry_capacity: usize,
    regular_capacity: usize,
    coalesced_capacity: usize,
) -> (MatrixRequestSender, MatrixRequestReceiver) {
    assert!(critical_capacity > 0);
    assert!(critical_retry_capacity > 0);
    assert!(regular_capacity > 0);
    assert!(coalesced_capacity > 0);
    let (critical_sender, critical_receiver) = mpsc::channel(critical_capacity);
    let (regular_sender, regular_receiver) = mpsc::channel(regular_capacity);
    let shared = Arc::new(SharedState {
        buffered: Mutex::new(BufferedState {
            critical_retry: VecDeque::with_capacity(critical_retry_capacity),
            requests: HashMap::with_capacity(coalesced_capacity),
        }),
        notify: Notify::new(),
        receiver_alive: AtomicBool::new(true),
        critical_retry_capacity,
        coalesced_capacity,
    });
    (
        MatrixRequestSender {
            critical: critical_sender,
            regular: regular_sender,
            shared: shared.clone(),
        },
        MatrixRequestReceiver {
            critical: critical_receiver,
            regular: regular_receiver,
            shared,
            critical_closed: false,
            regular_closed: false,
        },
    )
}

#[cfg(test)]
mod tests {
    use matrix_sdk::ruma::{RoomId, events::room::message::RoomMessageEventContent};

    use super::*;

    fn room_id(localpart: &str) -> OwnedRoomId {
        RoomId::parse(format!("!{localpart}:example.org")).unwrap()
    }

    fn send_message_request(localpart: &str) -> MatrixRequest {
        MatrixRequest::SendMessage {
            timeline_kind: TimelineKind::MainRoom {
                room_id: room_id(localpart),
            },
            message: RoomMessageEventContent::text_plain("hello"),
            replied_to: None,
            #[cfg(feature = "tsp")]
            sign_with_tsp: false,
        }
    }

    #[tokio::test]
    async fn ten_thousand_typing_requests_coalesce_without_starving_membership() {
        let (sender, mut receiver) = matrix_request_channel_with_limits(2, 2, 2, 2);
        let typing_room = room_id("typing");
        for i in 0..10_000 {
            sender
                .try_submit(MatrixRequest::SendTypingNotice {
                    room_id: typing_room.clone(),
                    typing: i % 2 == 0,
                })
                .unwrap();
        }
        let joined_room = room_id("joined");
        sender
            .try_submit(MatrixRequest::JoinRoom {
                room_id: joined_room.clone(),
            })
            .unwrap();

        assert!(matches!(
            receiver.recv().await,
            Some(MatrixRequest::JoinRoom { room_id }) if room_id == joined_room
        ));
        assert!(matches!(
            receiver.recv().await,
            Some(MatrixRequest::SendTypingNotice { room_id, typing: false })
                if room_id == typing_room
        ));
    }

    #[tokio::test]
    async fn critical_overflow_uses_bounded_retry_without_loss_or_reordering() {
        let (sender, mut receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        assert_eq!(
            sender.try_submit(MatrixRequest::Logout { is_desktop: true }),
            Ok(MatrixRequestSubmitDisposition::Enqueued),
        );
        let joined_room = room_id("deferred");
        assert_eq!(
            sender.try_submit(MatrixRequest::JoinRoom {
                room_id: joined_room.clone(),
            }),
            Ok(MatrixRequestSubmitDisposition::Deferred),
        );
        assert_eq!(
            sender.try_submit(MatrixRequest::LeaveRoom {
                room_id: room_id("rejected"),
            }),
            Err(MatrixRequestSubmitError::CriticalRejected),
        );
        assert!(matches!(
            receiver.recv().await,
            Some(MatrixRequest::Logout { is_desktop: true })
        ));
        assert!(matches!(
            receiver.recv().await,
            Some(MatrixRequest::JoinRoom { room_id }) if room_id == joined_room
        ));

        // Once the retry lane recovers, a later critical action can enter and complete normally.
        sender
            .try_submit(MatrixRequest::Logout { is_desktop: false })
            .unwrap();
        assert!(matches!(
            receiver.recv().await,
            Some(MatrixRequest::Logout { is_desktop: false })
        ));
    }

    #[test]
    fn saturated_send_message_is_explicitly_rejected_never_silent_success() {
        let (sender, _receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        sender
            .try_submit(MatrixRequest::Logout { is_desktop: true })
            .unwrap();
        sender
            .try_submit(MatrixRequest::JoinRoom {
                room_id: room_id("deferred"),
            })
            .unwrap();

        let send_message = send_message_request("message");
        assert!(is_critical_request(&send_message));
        assert_eq!(
            sender.try_submit(send_message),
            Err(MatrixRequestSubmitError::CriticalRejected),
        );
    }

    #[test]
    fn coalesced_overflow_is_bounded_and_explicit() {
        let (sender, _receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        sender
            .try_submit(MatrixRequest::SendTypingNotice {
                room_id: room_id("typing"),
                typing: true,
            })
            .unwrap();
        assert_eq!(
            sender.try_submit(MatrixRequest::GetNumberUnreadMessages {
                timeline_kind: TimelineKind::MainRoom {
                    room_id: room_id("unread"),
                },
            }),
            Err(MatrixRequestSubmitError::NonCriticalBackpressured),
        );
    }

    #[test]
    fn receiver_close_is_reported_without_panic() {
        let (sender, receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        drop(receiver);
        assert_eq!(
            sender.try_submit(MatrixRequest::Logout { is_desktop: false }),
            Err(MatrixRequestSubmitError::Closed),
        );
    }

    #[test]
    fn receiver_close_discards_buffered_retry_and_all_later_submits_are_closed() {
        let (sender, receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        sender
            .try_submit(MatrixRequest::Logout { is_desktop: true })
            .unwrap();
        assert_eq!(
            sender.try_submit(MatrixRequest::JoinRoom {
                room_id: room_id("deferred"),
            }),
            Ok(MatrixRequestSubmitDisposition::Deferred),
        );

        drop(receiver);
        assert_eq!(
            sender.try_submit(MatrixRequest::Logout { is_desktop: false }),
            Err(MatrixRequestSubmitError::Closed),
        );
    }

    #[test]
    fn channel_identity_distinguishes_replacement_generations() {
        let (sender, _receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        let sender_clone = sender.clone();
        let (replacement, _replacement_receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);

        assert!(sender.same_channel(&sender_clone));
        assert!(!sender.same_channel(&replacement));
    }

    #[tokio::test]
    async fn sender_disconnect_ends_receiver_without_panic() {
        let (sender, mut receiver) = matrix_request_channel_with_limits(1, 1, 1, 1);
        drop(sender);
        assert!(receiver.recv().await.is_none());
    }
}

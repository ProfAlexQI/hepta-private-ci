use codex_protocol::protocol::InterAgentCommunication;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;
use tokio::sync::watch;

#[cfg(test)]
use codex_protocol::AgentPath;

pub(crate) struct Mailbox {
    tx: mpsc::UnboundedSender<MailboxDelivery>,
    next_seq: AtomicU64,
    seq_tx: watch::Sender<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MailboxDelivery {
    pub(crate) sequence: u64,
    pub(crate) communication: InterAgentCommunication,
    pub(crate) parent_turn_id: Option<String>,
}

pub(crate) struct MailboxReceiver {
    rx: mpsc::UnboundedReceiver<MailboxDelivery>,
    pending_mails: VecDeque<MailboxDelivery>,
}

impl Mailbox {
    pub(crate) fn new() -> (Self, MailboxReceiver) {
        let (tx, rx) = mpsc::unbounded_channel();
        let (seq_tx, _) = watch::channel(0);
        (
            Self {
                tx,
                next_seq: AtomicU64::new(0),
                seq_tx,
            },
            MailboxReceiver {
                rx,
                pending_mails: VecDeque::new(),
            },
        )
    }

    pub(crate) fn subscribe(&self) -> watch::Receiver<u64> {
        self.seq_tx.subscribe()
    }

    pub(crate) fn send(&self, communication: InterAgentCommunication) -> u64 {
        self.send_with_parent(communication, /*parent_turn_id*/ None)
    }

    pub(crate) fn send_with_parent(
        &self,
        communication: InterAgentCommunication,
        parent_turn_id: Option<String>,
    ) -> u64 {
        let seq = self.next_seq.fetch_add(1, Ordering::Relaxed) + 1;
        let _ = self.tx.send(MailboxDelivery {
            sequence: seq,
            communication,
            parent_turn_id,
        });
        self.seq_tx.send_replace(seq);
        seq
    }
}

impl MailboxReceiver {
    fn sync_pending_mails(&mut self) {
        while let Ok(delivery) = self.rx.try_recv() {
            self.pending_mails.push_back(delivery);
        }
    }

    pub(crate) fn has_pending(&mut self) -> bool {
        self.sync_pending_mails();
        !self.pending_mails.is_empty()
    }

    pub(crate) fn has_pending_trigger_turn(&mut self) -> bool {
        self.sync_pending_mails();
        self.pending_mails
            .iter()
            .any(|mail| mail.communication.trigger_turn)
    }

    pub(crate) fn unambiguous_trigger_parent_turn_id(&mut self) -> Option<String> {
        self.sync_pending_mails();
        let mut trigger_turn_mails = self
            .pending_mails
            .iter()
            .filter(|mail| mail.communication.trigger_turn);
        let parent_turn_id = trigger_turn_mails.next()?.parent_turn_id.as_ref()?;
        trigger_turn_mails
            .all(|mail| mail.parent_turn_id.as_ref() == Some(parent_turn_id))
            .then(|| parent_turn_id.clone())
    }

    #[cfg(test)]
    pub(crate) fn drain(&mut self) -> Vec<InterAgentCommunication> {
        self.drain_deliveries()
            .into_iter()
            .map(|delivery| delivery.communication)
            .collect()
    }

    pub(crate) fn drain_deliveries(&mut self) -> Vec<MailboxDelivery> {
        self.sync_pending_mails();
        self.pending_mails.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn make_mail(
        author: AgentPath,
        recipient: AgentPath,
        content: &str,
        trigger_turn: bool,
    ) -> InterAgentCommunication {
        InterAgentCommunication::new(
            author,
            recipient,
            Vec::new(),
            content.to_string(),
            trigger_turn,
        )
    }

    #[tokio::test]
    async fn mailbox_assigns_monotonic_sequence_numbers() {
        let (mailbox, _receiver) = Mailbox::new();
        let mut seq_rx = mailbox.subscribe();

        let seq_a = mailbox.send(make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "one",
            /*trigger_turn*/ false,
        ));
        let seq_b = mailbox.send(make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "two",
            /*trigger_turn*/ false,
        ));

        seq_rx.changed().await.expect("first seq update");
        assert_eq!(*seq_rx.borrow(), seq_b);
        assert_eq!(seq_a, 1);
        assert_eq!(seq_b, 2);
    }

    #[tokio::test]
    async fn mailbox_drains_in_delivery_order() {
        let (mailbox, mut receiver) = Mailbox::new();
        let mail_one = make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "one",
            /*trigger_turn*/ false,
        );
        let mail_two = make_mail(
            AgentPath::try_from("/root/worker").expect("agent path"),
            AgentPath::root(),
            "two",
            /*trigger_turn*/ false,
        );

        mailbox.send(mail_one.clone());
        mailbox.send(mail_two.clone());

        assert_eq!(
            receiver.drain_deliveries(),
            vec![
                MailboxDelivery {
                    sequence: 1,
                    communication: mail_one,
                    parent_turn_id: None,
                },
                MailboxDelivery {
                    sequence: 2,
                    communication: mail_two,
                    parent_turn_id: None,
                },
            ]
        );
        assert!(!receiver.has_pending());
    }

    #[tokio::test]
    async fn mailbox_drains_legacy_messages_in_delivery_order() {
        let (mailbox, mut receiver) = Mailbox::new();
        let mail_one = make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "one",
            /*trigger_turn*/ false,
        );
        let mail_two = make_mail(
            AgentPath::try_from("/root/worker").expect("agent path"),
            AgentPath::root(),
            "two",
            /*trigger_turn*/ false,
        );

        mailbox.send(mail_one.clone());
        mailbox.send(mail_two.clone());

        assert_eq!(receiver.drain(), vec![mail_one, mail_two]);
        assert!(!receiver.has_pending());
    }

    #[tokio::test]
    async fn mailbox_tracks_pending_trigger_turn_mail() {
        let (mailbox, mut receiver) = Mailbox::new();

        mailbox.send(make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "queued",
            /*trigger_turn*/ false,
        ));
        assert!(!receiver.has_pending_trigger_turn());

        mailbox.send(make_mail(
            AgentPath::root(),
            AgentPath::try_from("/root/worker").expect("agent path"),
            "wake",
            /*trigger_turn*/ true,
        ));
        assert!(receiver.has_pending_trigger_turn());
    }

    #[tokio::test]
    async fn mailbox_reports_parent_turn_only_when_trigger_sources_are_unambiguous() {
        let (mailbox, mut receiver) = Mailbox::new();
        mailbox.send_with_parent(
            make_mail(
                AgentPath::root(),
                AgentPath::try_from("/root/worker").expect("agent path"),
                "wake",
                /*trigger_turn*/ true,
            ),
            Some("parent-a".to_string()),
        );
        assert_eq!(
            receiver.unambiguous_trigger_parent_turn_id().as_deref(),
            Some("parent-a")
        );

        mailbox.send_with_parent(
            make_mail(
                AgentPath::root(),
                AgentPath::try_from("/root/worker").expect("agent path"),
                "different parent",
                /*trigger_turn*/ true,
            ),
            Some("parent-b".to_string()),
        );
        assert_eq!(receiver.unambiguous_trigger_parent_turn_id(), None);
    }
}

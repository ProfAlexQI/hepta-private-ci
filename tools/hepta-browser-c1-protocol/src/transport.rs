use std::io::Read;
use std::io::Write;

use crate::ProtocolError;
use crate::protocol::EstablishedBinding;
use crate::protocol::HostAck;
use crate::protocol::HostExpectedWorker;
use crate::protocol::Message;
use crate::protocol::WorkerConfirm;
use crate::read_message;
use crate::write_message;

pub fn host_handshake<T: Read + Write>(
    io: &mut T,
    expected: &HostExpectedWorker,
) -> Result<EstablishedBinding, ProtocolError> {
    let hello = match read_message(io)? {
        Message::WorkerHello(hello) => hello,
        _ => return Err(ProtocolError::UnexpectedMessage),
    };
    if hello.identity != expected.identity {
        return Err(ProtocolError::StaleFence);
    }
    if hello.source_pin != expected.source_pin
        || !hello.startup_capability_matches(&expected.startup_capability)
    {
        return Err(ProtocolError::AuthenticationFailed);
    }
    hello.authority.validate()?;

    write_message(
        io,
        &Message::HostAck(HostAck::accepted(expected.identity, expected.host_nonce)?),
    )?;

    let confirm = match read_message(io)? {
        Message::WorkerConfirm(confirm) => confirm,
        _ => return Err(ProtocolError::UnexpectedMessage),
    };
    if confirm.identity != expected.identity {
        return Err(ProtocolError::StaleFence);
    }
    if !confirm.host_nonce_matches(expected.host_nonce()) {
        return Err(ProtocolError::AuthenticationFailed);
    }

    Ok(EstablishedBinding {
        identity: expected.identity,
        source_pin: expected.source_pin,
        authority: hello.authority,
    })
}

pub fn worker_handshake<T: Read + Write>(
    io: &mut T,
    expected: &HostExpectedWorker,
) -> Result<EstablishedBinding, ProtocolError> {
    let hello = expected.worker_hello()?;
    write_message(io, &Message::WorkerHello(hello.clone()))?;

    let acknowledgement = match read_message(io)? {
        Message::HostAck(acknowledgement) => acknowledgement,
        _ => return Err(ProtocolError::UnexpectedMessage),
    };
    if !acknowledgement.accepted || acknowledgement.identity != expected.identity {
        return Err(ProtocolError::StaleFence);
    }
    if !acknowledgement.host_nonce_matches(expected.host_nonce()) {
        return Err(ProtocolError::AuthenticationFailed);
    }

    write_message(
        io,
        &Message::WorkerConfirm(WorkerConfirm::new(
            expected.identity,
            expected.host_nonce,
        )?),
    )?;

    Ok(EstablishedBinding {
        identity: expected.identity,
        source_pin: expected.source_pin,
        authority: hello.authority,
    })
}

pub struct FramedChannel<T> {
    io: T,
    binding: EstablishedBinding,
}

impl<T> FramedChannel<T> {
    pub fn new(io: T, binding: EstablishedBinding) -> Self {
        Self { io, binding }
    }

    pub fn binding(&self) -> EstablishedBinding {
        self.binding
    }

    pub fn into_inner(self) -> T {
        self.io
    }
}

impl<T: Read + Write> FramedChannel<T> {
    pub fn send(&mut self, message: &Message) -> Result<(), ProtocolError> {
        require_message_binding(message, self.binding)?;
        write_message(&mut self.io, message)
    }

    pub fn receive(&mut self) -> Result<Message, ProtocolError> {
        let message = read_message(&mut self.io)?;
        require_message_binding(&message, self.binding)?;
        Ok(message)
    }
}

fn require_message_binding(
    message: &Message,
    binding: EstablishedBinding,
) -> Result<(), ProtocolError> {
    let identity = match message {
        Message::WorkerHello(hello) => hello.identity,
        Message::HostAck(acknowledgement) => acknowledgement.identity,
        Message::WorkerConfirm(confirm) => confirm.identity,
        Message::Command(command) => command.identity,
        Message::Outcome(outcome) => outcome.identity,
    };
    if identity != binding.identity {
        return Err(ProtocolError::StaleFence);
    }
    Ok(())
}

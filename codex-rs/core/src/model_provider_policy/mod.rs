// This binding slice is registered before the transport callers so its
// identity rules can be reviewed independently. The allowance is removed by
// the caller-composition slice that consumes every exported operation.
#[allow(dead_code)]
mod binding;

// Registered ahead of the transport caller for the same staged-review reason
// as `binding`; the caller-composition slice removes this allowance.
#[allow(dead_code)]
mod lifecycle;

// Registered before the HTTP/WS callsites so cancellation ownership can be
// reviewed and tested independently.
#[allow(dead_code)]
mod attempt_owner;

// Registered before response-stream plumbing so terminal framing remains a
// focused review slice.
#[allow(dead_code)]
mod response_terminal;

// Registered before HTTP/WS caller composition so wire identity can be
// reviewed independently from transport control flow.
#[allow(dead_code)]
mod transport;

pub(crate) use attempt_owner::ProviderAttemptOwner;
#[cfg(test)]
pub(crate) use binding::bytes_sha256;
#[cfg(test)]
pub(crate) use binding::canonical_sha256;
pub(crate) use response_terminal::ProviderResponseTerminal;
pub(crate) use transport::ProviderRoutingHint;

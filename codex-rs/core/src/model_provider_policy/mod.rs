// This binding slice is registered before the transport callers so its
// identity rules can be reviewed independently. The allowance is removed by
// the caller-composition slice that consumes every exported operation.
#[allow(dead_code)]
mod binding;

// Registered ahead of the transport caller for the same staged-review reason
// as `binding`; the caller-composition slice removes this allowance.
#[allow(dead_code)]
mod lifecycle;

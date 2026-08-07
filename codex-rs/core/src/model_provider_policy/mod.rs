// This binding slice is registered before the transport callers so its
// identity rules can be reviewed independently. The allowance is removed by
// the caller-composition slice that consumes every exported operation.
#[allow(dead_code)]
mod binding;

#[derive(Debug, thiserror::Error)]
pub enum NativeErrorV8 {
    #[error("invalid Linux v8 native operation: {0}")]
    Invalid(String),

    #[error("Linux v8 native I/O failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Linux v8 native syscall boundary failed: {0}")]
    Sys(#[from] crate::NativeSysErrorV8),
}

pub(crate) fn invalid(message: impl Into<String>) -> NativeErrorV8 {
    NativeErrorV8::Invalid(message.into())
}

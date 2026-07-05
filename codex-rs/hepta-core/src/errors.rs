use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaError(pub String);

impl Display for HeptaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for HeptaError {}

macro_rules! define_error {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(pub String);

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Error for $name {}
    };
}

define_error!(ModelError);
define_error!(ToolError);
define_error!(MemoryError);
define_error!(PolicyError);
define_error!(ChannelError);
define_error!(PluginError);

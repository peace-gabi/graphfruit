use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error returned by the `connect` and `disconnect` methods of `Graph`.
#[derive(Debug)]
pub enum ConnectError {
    /// An invalid source node ID was provided.
    InvalidSrcNodeId,
    /// An invalid destination node ID was provided.
    InvalidDstNodeId,
    /// An invalid relation ID was provided.
    InvalidRelationId,
}

impl Error for ConnectError {}

impl Display for ConnectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSrcNodeId => write!(f, "Invalid source node ID"),
            Self::InvalidDstNodeId => write!(f, "Invalid destination node ID"),
            Self::InvalidRelationId => write!(f, "Invalid relation ID"),
        }
    }
}

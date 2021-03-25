use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// Node in a graph.
pub struct Node {
    id: NodeId,
    info: Box<dyn NodeInfo>,
}

impl Node {
    /// Create a `Node` with an id and info.
    pub fn new<I>(id: NodeId, info: I) -> Self
    where
        I: NodeInfo,
    {
        Self {
            id,
            info: Box::new(info),
        }
    }

    /// Get the id of the node.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Get the type erased info.
    pub fn info(&self) -> &dyn NodeInfo {
        Box::as_ref(&self.info)
    }
}

/// Uniquely identifies a node withing a graph.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(u64);

impl NodeId {
    /// Create a new `NodeId`.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Deref for NodeId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait NodeInfo
where
    Self: Send + Sync + 'static,
{
}

impl NodeInfo for i32 {}

impl NodeInfo for u32 {}

impl NodeInfo for String {}

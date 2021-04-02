use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// Node in a graph.
pub struct Node {
    id: NodeId,
    info: AnyNodeInfo,
}

impl Node {
    /// Create a `Node` with an id and info.
    pub fn new<I>(id: NodeId, info: I) -> Self
    where
        I: Into<AnyNodeInfo>,
    {
        Self {
            id,
            info: info.into(),
        }
    }

    /// Get the id of the node.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Get a shared reference to the type erased info.
    pub fn info(&self) -> &dyn NodeInfo {
        self.info.deref()
    }

    /// Get an exclusive reference to the type erased info.
    pub fn info_mut(&mut self) -> &mut dyn NodeInfo {
        self.info.deref_mut()
    }

    /// Consume the node and return its info.
    pub fn into_info(self) -> AnyNodeInfo {
        self.info
    }
}

/// Type erased container for a node info.
pub struct AnyNodeInfo(Box<dyn NodeInfo>);

impl<I> From<I> for AnyNodeInfo
where
    I: NodeInfo,
{
    fn from(info: I) -> Self {
        Self(Box::new(info))
    }
}

impl Deref for AnyNodeInfo {
    type Target = dyn NodeInfo;

    fn deref(&self) -> &Self::Target {
        Box::as_ref(&self.0)
    }
}

impl DerefMut for AnyNodeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Box::as_mut(&mut self.0)
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

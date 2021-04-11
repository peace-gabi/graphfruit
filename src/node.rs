use downcast_rs::{impl_downcast, DowncastSync};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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

/// Trait implemented by types which can be stored in `Nodes`.
pub trait NodeInfo
where
    Self: DowncastSync,
{
}

impl_downcast!(sync NodeInfo);

impl NodeInfo for () {}

impl NodeInfo for i32 {}

impl NodeInfo for u32 {}

impl NodeInfo for String {}

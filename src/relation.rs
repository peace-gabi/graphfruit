use crate::node::NodeId;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// Models a relation between 2 nodes in a graph.
pub struct Relation {
    id: RelationId,
    src: NodeId,
    dst: NodeId,
    info: Box<dyn RelationInfo>,
}

impl Relation {
    /// Create a new `Relation` with an id, source, destination and info.
    pub fn new<I>(id: RelationId, src: NodeId, dst: NodeId, info: I) -> Self
    where
        I: RelationInfo,
    {
        Self {
            id,
            src,
            dst,
            info: Box::new(info),
        }
    }

    /// Get the relation id.
    pub fn id(&self) -> RelationId {
        self.id
    }

    /// Get the source node id.
    pub fn src(&self) -> NodeId {
        self.src
    }

    /// Get the destination node id.
    pub fn dst(&self) -> NodeId {
        self.dst
    }

    /// Get the type erased info.
    pub fn info(&self) -> &dyn RelationInfo {
        Box::as_ref(&self.info)
    }
}

/// Uniquely identifies a relation within a graph.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RelationId(u64);

impl RelationId {
    /// Create a new `RelationId`.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Deref for RelationId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RelationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait RelationInfo
where
    Self: Send + Sync + 'static,
{
}

impl RelationInfo for i32 {}

impl RelationInfo for u32 {}

impl RelationInfo for String {}

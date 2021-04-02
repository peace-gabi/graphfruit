use crate::node::NodeId;
use downcast_rs::{impl_downcast, DowncastSync};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};

/// Models a relation between 2 nodes in a graph.
pub struct Relation {
    id: RelationId,
    src: NodeId,
    dst: NodeId,
    info: AnyRelationInfo,
}

impl Relation {
    /// Create a `Relation` with an id, source, destination and info.
    pub fn new<I>(id: RelationId, src: NodeId, dst: NodeId, info: I) -> Self
    where
        I: RelationInfo,
    {
        Self {
            id,
            src,
            dst,
            info: info.into(),
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

    /// Get a shared reference to the type erased info.
    pub fn info(&self) -> &dyn RelationInfo {
        &*self.info
    }

    /// Get an exclusive reference to the type erased info.
    pub fn info_mut(&mut self) -> &mut dyn RelationInfo {
        &mut *self.info
    }

    /// Consume the relation and return its info.
    pub fn into_info(self) -> AnyRelationInfo {
        self.info
    }
}

/// Type erased container for a relation info.
pub struct AnyRelationInfo(Box<dyn RelationInfo>);

impl<I> From<I> for AnyRelationInfo
where
    I: RelationInfo,
{
    fn from(info: I) -> Self {
        AnyRelationInfo(Box::new(info))
    }
}

impl Deref for AnyRelationInfo {
    type Target = dyn RelationInfo;

    fn deref(&self) -> &Self::Target {
        Box::as_ref(&self.0)
    }
}

impl DerefMut for AnyRelationInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Box::as_mut(&mut self.0)
    }
}

/// Uniquely identifies a relation within a graph.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RelationId(NonZeroU64);

impl RelationId {
    /// Create a new `RelationId` with the given id.
    /// The id must be non zero.
    pub fn new(id: u64) -> Self {
        Self(NonZeroU64::new(id).unwrap())
    }

    /// Get the numeric value of the id.
    pub fn get(&self) -> u64 {
        self.0.get()
    }
}

impl Display for RelationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

pub trait RelationInfo
where
    Self: DowncastSync,
{
}

impl_downcast!(sync RelationInfo);

impl RelationInfo for i32 {}

impl RelationInfo for u32 {}

impl RelationInfo for String {}

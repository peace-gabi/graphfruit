use crate::edge::Edge;
use downcast_rs::{impl_downcast, DowncastSync};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};

/// Models a relation between nodes in a graph.
pub struct Relation {
    info: AnyRelationInfo,
    edges: HashSet<Edge>,
}

impl Relation {
    /// Create a `Relation` with an id, and info.
    pub fn new<I>(info: I) -> Self
    where
        I: Into<AnyRelationInfo>,
    {
        Self {
            info: info.into(),
            edges: HashSet::new(),
        }
    }

    /// Get a shared reference to the type erased info.
    pub fn info(&self) -> &dyn RelationInfo {
        &*self.info
    }

    /// Get an exclusive reference to the type erased info.
    pub fn info_mut(&mut self) -> &mut dyn RelationInfo {
        &mut *self.info
    }

    /// Insert a new edge to the relation and return whether
    /// or not the edge did not exist previously.
    pub fn insert_edge(&mut self, edge: Edge) -> bool {
        self.edges.insert(edge)
    }

    /// Remove an edge from the relation and return whether
    /// or not there was anything to remove.
    pub fn remove_edge(&mut self, edge: &Edge) -> bool {
        self.edges.remove(edge)
    }

    /// Check if the relation contains an edge.
    pub fn contains_edge(&self, edge: &Edge) -> bool {
        self.edges.contains(edge)
    }

    /// Get an iterator over all the edges that belong to the relation.
    pub fn iter_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
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

/// Trait implemented by types which can be stored in `Relations`.
pub trait RelationInfo
where
    Self: DowncastSync,
{
}

impl_downcast!(sync RelationInfo);

impl RelationInfo for i32 {}

impl RelationInfo for u32 {}

impl RelationInfo for String {}

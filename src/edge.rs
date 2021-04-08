use crate::node::NodeId;

/// Represents a connection between two `Nodes`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Edge {
    src: NodeId,
    dst: NodeId,
}

impl Edge {
    /// Create an `Edge`.
    pub fn new(src: NodeId, dst: NodeId) -> Self {
        Self { src, dst }
    }

    /// `NodeId` of source node.
    pub fn src(&self) -> NodeId {
        self.src
    }

    /// `NodeId` of destination node.
    pub fn dst(&self) -> NodeId {
        self.dst
    }
}

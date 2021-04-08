use crate::node::NodeId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Edge {
    src: NodeId,
    dst: NodeId,
}

impl Edge {
    pub fn new(src: NodeId, dst: NodeId) -> Self {
        Self { src, dst }
    }

    pub fn src(&self) -> NodeId {
        self.src
    }

    pub fn dst(&self) -> NodeId {
        self.dst
    }
}

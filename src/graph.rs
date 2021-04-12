use crate::edge::Edge;
use crate::errors::ConnectError;
use crate::id::IdGenerator;
use crate::node::{AnyNodeInfo, NodeId};
use crate::relation::{AnyRelationInfo, Relation, RelationId};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Graph {
    next_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    prev_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    node_info: HashMap<NodeId, AnyNodeInfo>,
    relations: HashMap<RelationId, Relation>,
    node_id_generator: IdGenerator,
    relation_id_generator: IdGenerator,
}

impl Graph {
    /// Create an empty `Graph`.
    pub fn new() -> Graph {
        Self::default()
    }

    fn generate_node_id(&mut self) -> NodeId {
        NodeId::new(self.node_id_generator.generate_id_sync())
    }

    fn generate_relation_id(&mut self) -> RelationId {
        RelationId::new(self.relation_id_generator.generate_id_sync())
    }

    /// Create a `Node` in the graph with `info` and return its `NodeId`.
    pub fn add_node<I>(&mut self, info: I) -> NodeId
    where
        I: Into<AnyNodeInfo>,
    {
        let id = self.generate_node_id();
        self.next_nodes.insert(id, HashMap::new());
        self.prev_nodes.insert(id, HashMap::new());
        self.node_info.insert(id, info.into());
        id
    }

    /// Remove the `Node` at `node_id` and return its info if it was removed.
    pub fn remove_node(&mut self, node_id: NodeId) -> Option<AnyNodeInfo> {
        let info = self.node_info.remove(&node_id)?;

        let prev_nodes = self.prev_nodes.remove(&node_id).unwrap();

        for src_id in prev_nodes.keys() {
            // Get all ids of relations with node as source
            let relation_ids = self.next_nodes.get_mut(src_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relations
                    .get_mut(relation_id)?
                    .remove_edge(&Edge::new(*src_id, node_id));
            }
        }

        let next_nodes = self.next_nodes.remove(&node_id).unwrap();

        for dst_id in next_nodes.keys() {
            // Get all ids of relations with node as destination
            let relation_ids = self.prev_nodes.get_mut(dst_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relations
                    .get_mut(relation_id)?
                    .remove_edge(&Edge::new(node_id, *dst_id));
            }
        }

        Some(info)
    }

    /// Create a `Relation` in the graph with `info` and return its `RelationId`.
    pub fn add_relation<I>(&mut self, info: I) -> RelationId
    where
        I: Into<AnyRelationInfo>,
    {
        let id = self.generate_relation_id();
        self.relations.insert(id, Relation::new(info));
        id
    }

    /// Remove the `Relation` at `relation_id` and return its info if it was removed.
    pub fn remove_relation(&mut self, relation_id: RelationId) -> Option<AnyRelationInfo> {
        let relation = self.relations.remove(&relation_id)?;

        for edge in relation.iter_edges() {
            self.prev_nodes
                .get_mut(&edge.dst())?
                .get_mut(&edge.src())?
                .remove(&relation_id);

            self.next_nodes
                .get_mut(&edge.src())?
                .get_mut(&edge.dst())?
                .remove(&relation_id);
        }

        Some(relation.into_info())
    }

    /// Connect two `Nodes` in the graph with a `Relation`.
    pub fn connect(
        &mut self,
        src: NodeId,
        dst: NodeId,
        relation_id: RelationId,
    ) -> Result<bool, ConnectError> {
        let in_nodes = self
            .prev_nodes
            .get_mut(&dst)
            .ok_or(ConnectError::InvalidDstNodeId)?;

        let out_nodes = self
            .next_nodes
            .get_mut(&src)
            .ok_or(ConnectError::InvalidSrcNodeId)?;

        let relations = self
            .relations
            .get_mut(&relation_id)
            .ok_or(ConnectError::InvalidRelationId)?;

        in_nodes.entry(src).or_default().insert(relation_id);
        out_nodes.entry(dst).or_default().insert(relation_id);
        Ok(relations.insert_edge(Edge::new(src, dst)))
    }

    /// Disconnect the `Relation` between two `Nodes`.
    pub fn disconnect(
        &mut self,
        src: NodeId,
        dst: NodeId,
        relation_id: RelationId,
    ) -> Result<bool, ConnectError> {
        let in_nodes = self
            .prev_nodes
            .get_mut(&dst)
            .ok_or(ConnectError::InvalidDstNodeId)?;

        let out_nodes = self
            .next_nodes
            .get_mut(&src)
            .ok_or(ConnectError::InvalidSrcNodeId)?;

        let relation_data = self
            .relations
            .get_mut(&relation_id)
            .ok_or(ConnectError::InvalidRelationId)?;

        if relation_data.remove_edge(&Edge::new(src, dst)) {
            in_nodes.get_mut(&src).unwrap().remove(&relation_id);
            out_nodes.get_mut(&dst).unwrap().remove(&relation_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get the number of `Nodes` in the graph.
    pub fn nr_nodes(&self) -> usize {
        self.node_info.len()
    }

    /// Get the number of `Relations` in the graph.
    pub fn nr_relations(&self) -> usize {
        self.relations.len()
    }

    /// Get the in degree of a `Node`.
    pub fn in_degree_of(&self, node_id: NodeId) -> Option<usize> {
        Some(
            self.prev_nodes
                .get(&node_id)?
                .values()
                .map(|r| r.len())
                .sum(),
        )
    }

    /// Get the out degree of a `Node`.
    pub fn out_degree_of(&self, node_id: NodeId) -> Option<usize> {
        Some(
            self.next_nodes
                .get(&node_id)?
                .values()
                .map(|r| r.len())
                .sum(),
        )
    }

    /// Get an iterator over all `Nodes` in the graph.
    pub fn iter_nodes(&self) -> impl Iterator<Item = &AnyNodeInfo> {
        self.node_info.values()
    }

    /// Get an iterator over all `RelationIds` and `Relations` in the graph.
    pub fn iter_relations(&self) -> impl Iterator<Item = (RelationId, &Relation)> {
        self.relations.iter().map(|(k, v)| (*k, v))
    }

    /// Get an iterator over all edges with `relation_id`.
    pub fn iter_relation_edges(
        &self,
        relation_id: RelationId,
    ) -> Option<impl Iterator<Item = &Edge>> {
        self.relations.get(&relation_id).map(|r| r.iter_edges())
    }
}

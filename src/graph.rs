use crate::edge::Edge;
use crate::errors::ConnectError;
use crate::id::IdGenerator;
use crate::node::{AnyNodeInfo, NodeId};
use crate::relation::{AnyRelationInfo, Relation, RelationId};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Graph {
    in_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    out_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    node_info: HashMap<NodeId, AnyNodeInfo>,
    relations: HashMap<RelationId, Relation>,
    node_id_generator: IdGenerator,
    relation_id_generator: IdGenerator,
}

impl Graph {
    /// Create an empty `Graph`.
    pub fn new() -> Graph {
        Graph {
            in_nodes: HashMap::new(),
            out_nodes: HashMap::new(),
            node_info: HashMap::new(),
            relations: HashMap::new(),
            node_id_generator: IdGenerator::default(),
            relation_id_generator: IdGenerator::default(),
        }
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
        self.in_nodes.insert(id, HashMap::new());
        self.out_nodes.insert(id, HashMap::new());
        self.node_info.insert(id, info.into());
        id
    }

    /// Remove the `Node` at `node_id` and return its info if it was removed.
    pub fn remove_node(&mut self, node_id: NodeId) -> Option<AnyNodeInfo> {
        let info = self.node_info.remove(&node_id)?;

        for src_id in self.in_nodes[&node_id].keys() {
            // Get all ids of relations with node as source
            let relation_ids = self.out_nodes.get_mut(src_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relations
                    .get_mut(relation_id)?
                    .remove_edge(&Edge::new(*src_id, node_id));
            }
        }
        self.in_nodes.remove(&node_id)?;
        for dst_id in self.out_nodes[&node_id].keys() {
            // Get all ids of relations with node as destination
            let relation_ids = self.in_nodes.get_mut(dst_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relations
                    .get_mut(relation_id)?
                    .remove_edge(&Edge::new(node_id, *dst_id));
            }
        }
        self.out_nodes.remove(&node_id)?;
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
            self.in_nodes
                .get_mut(&edge.dst())?
                .get_mut(&edge.src())?
                .remove(&relation_id);

            self.out_nodes
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
    ) -> Result<(), ConnectError> {
        let in_nodes = self
            .in_nodes
            .get_mut(&src)
            .ok_or(ConnectError::InvalidSrcNodeId)?;

        let out_nodes = self
            .out_nodes
            .get_mut(&dst)
            .ok_or(ConnectError::InvalidDstNodeId)?;

        let relation_data = self
            .relations
            .get_mut(&relation_id)
            .ok_or(ConnectError::InvalidRelationId)?;

        in_nodes.entry(dst).or_default().insert(relation_id);
        out_nodes.entry(src).or_default().insert(relation_id);
        relation_data.insert_edge(Edge::new(src, dst));

        Ok(())
    }

    /// Disconnect the `Relation` between two `Nodes`.
    pub fn disconnect(
        &mut self,
        src: NodeId,
        dst: NodeId,
        relation_id: RelationId,
    ) -> Result<bool, ConnectError> {
        let in_nodes = self
            .in_nodes
            .get_mut(&src)
            .ok_or(ConnectError::InvalidSrcNodeId)?;

        let out_nodes = self
            .out_nodes
            .get_mut(&dst)
            .ok_or(ConnectError::InvalidDstNodeId)?;

        let relation_data = self
            .relations
            .get_mut(&relation_id)
            .ok_or(ConnectError::InvalidRelationId)?;

        if relation_data.remove_edge(&Edge::new(src, dst)) {
            in_nodes.get_mut(&dst).unwrap().remove(&relation_id);
            out_nodes.get_mut(&src).unwrap().remove(&relation_id);
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
        self.in_nodes.get(&node_id).map(|n| n.len())
    }

    /// Get the out degree of a `Node`.
    pub fn out_degree_of(&self, node_id: NodeId) -> Option<usize> {
        self.out_nodes.get(&node_id).map(|n| n.len())
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

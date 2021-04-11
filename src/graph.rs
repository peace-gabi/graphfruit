use crate::edge::Edge;
use crate::errors::ConnectError;
use crate::node::{AnyNodeInfo, NodeId};
use crate::relation::{AnyRelationInfo, Relation, RelationId};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Graph {
    in_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    out_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    node_info: HashMap<NodeId, AnyNodeInfo>,
    relations: HashMap<RelationId, Relation>,
    node_counter: u64,
    rel_counter: u64,
}

impl Graph {
    /// Create an empty `Graph`.
    pub fn new() -> Graph {
        Graph {
            in_nodes: HashMap::new(),
            out_nodes: HashMap::new(),
            node_info: HashMap::new(),
            relations: HashMap::new(),
            node_counter: 0,
            rel_counter: 0,
        }
    }

    fn generate_node_id(&mut self) -> NodeId {
        self.node_counter += 1;
        NodeId::new(self.node_counter)
    }

    fn generate_rel_id(&mut self) -> RelationId {
        self.rel_counter += 1;
        RelationId::new(self.rel_counter)
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

        Some(info)
    }

    /// Create a `Relation` in the graph with `info` and return its `RelationId`.
    pub fn add_relation<I>(&mut self, info: I) -> RelationId
    where
        I: Into<AnyRelationInfo>,
    {
        let relation_id = self.generate_rel_id();
        self.relations.insert(relation_id, Relation::new(info));
        relation_id
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

        in_nodes.get_mut(&dst).unwrap().remove(&relation_id);
        out_nodes.get_mut(&src).unwrap().remove(&relation_id);
        relation_data.remove_edge(&Edge::new(src, dst));

        Ok(())
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
    pub fn in_degree(&self, node_id: NodeId) -> Option<usize> {
        if self.in_nodes.contains_key(&node_id) {
            Some(self.in_nodes[&node_id].len())
        } else {
            None
        }
    }

    /// Get the out degree of a `Node`.
    pub fn out_degree(&self, node_id: NodeId) -> Option<usize> {
        if self.out_nodes.contains_key(&node_id) {
            Some(self.out_nodes[&node_id].len())
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        let mut v = HashSet::new();

        assert_eq!(graph.nr_nodes(), 0);
        assert!(v.insert(graph.add_node(100)));
        assert_eq!(graph.nr_nodes(), 1);
        assert!(v.insert(graph.add_node("lmao".to_string())));
        assert_eq!(graph.nr_nodes(), 2);
        assert!(v.insert(graph.add_node(235)));
        assert_eq!(graph.nr_nodes(), 3);
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::new();
        let mut v = Vec::new();

        for _ in 0..100 {
            v.push(graph.add_node("info".to_string()));
        }
        for &node_id in &v[..10] {
            graph.remove_node(node_id);
        }
        assert_eq!(graph.nr_nodes(), 90);

        for &node_id in &v[..10] {
            assert!(matches!(graph.remove_node(node_id), None));
        }
        for &node_id in &v[10..] {
            assert!(graph.remove_node(node_id).is_some());
        }
        assert_eq!(graph.nr_nodes(), 0);
    }
}

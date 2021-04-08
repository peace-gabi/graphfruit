use crate::edge::Edge;
use crate::node::{AnyNodeInfo, Node, NodeId};
use crate::relation::{AnyRelationInfo, Relation, RelationId};
use std::collections::{HashMap, HashSet};

struct RelationData {
    relation: Relation,
    edges: HashSet<Edge>,
}

pub struct Graph {
    in_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    out_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    nodes: HashMap<NodeId, Node>,
    relation_data: HashMap<RelationId, RelationData>,
    node_counter: u64,
    rel_counter: u64,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            in_nodes: HashMap::new(),
            out_nodes: HashMap::new(),
            nodes: HashMap::new(),
            relation_data: HashMap::new(),
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

    pub fn add_node<I>(&mut self, info: I) -> NodeId
    where
        I: Into<AnyNodeInfo>,
    {
        let id = self.generate_node_id();
        self.in_nodes.insert(id, HashMap::new());
        self.out_nodes.insert(id, HashMap::new());
        self.nodes.insert(id, Node::new(id, info));
        id
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> Option<AnyNodeInfo> {
        if !self.nodes.contains_key(&node_id) {
            return None;
        }

        for src_id in self.in_nodes[&node_id].keys() {
            // Get all ids of relations with node as source
            let relation_ids = self.out_nodes.get_mut(src_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relation_data
                    .get_mut(relation_id)?
                    .edges
                    .remove(&Edge::new(*src_id, node_id));
            }
        }

        for dst_id in self.out_nodes[&node_id].keys() {
            // Get all ids of relations with node as destination
            let relation_ids = self.in_nodes.get_mut(dst_id)?.remove(&node_id)?;

            // Remove the edge from all relations
            for relation_id in &relation_ids {
                self.relation_data
                    .get_mut(relation_id)?
                    .edges
                    .remove(&Edge::new(node_id, *dst_id));
            }
        }

        self.nodes.remove(&node_id).map(|n| n.into_info())
    }

    fn add_relation(&mut self, info: AnyRelationInfo) -> RelationId {
        let relation_id = self.generate_rel_id();

        self.relation_data.insert(
            relation_id,
            RelationData {
                relation: Relation::new(relation_id, info),
                edges: HashSet::new(),
            },
        );

        relation_id
    }

    fn remove_relation(&mut self, relation_id: RelationId) -> Option<AnyRelationInfo> {
        let relation_data = self.relation_data.remove(&relation_id)?;

        for edge in &relation_data.edges {
            self.in_nodes
                .get_mut(&edge.dst())?
                .get_mut(&edge.src())?
                .remove(&relation_id);

            self.out_nodes
                .get_mut(&edge.src())?
                .get_mut(&edge.dst())?
                .remove(&relation_id);
        }

        Some(relation_data.relation.into_info())
    }

    fn connect(&mut self, src: NodeId, dst: NodeId, relation_id: RelationId) -> Result<(), ()> {
        if !self.nodes.contains_key(&src) {
            return Err(());
        }

        if !self.nodes.contains_key(&dst) {
            return Err(());
        }

        if !self.relation_data.contains_key(&relation_id) {
            return Err(());
        }

        self.in_nodes
            .get_mut(&src)
            .unwrap()
            .entry(dst)
            .or_default()
            .insert(relation_id);

        self.out_nodes
            .get_mut(&dst)
            .unwrap()
            .entry(src)
            .or_default()
            .insert(relation_id);

        self.relation_data
            .get_mut(&relation_id)
            .unwrap()
            .edges
            .insert(Edge::new(src, dst));

        Ok(())
    }

    fn disconnect(&mut self, src: NodeId, dst: NodeId, relation_id: RelationId) -> Result<(), ()> {
        if !self.nodes.contains_key(&src) {
            return Err(());
        }

        if !self.nodes.contains_key(&dst) {
            return Err(());
        }

        if !self.relation_data.contains_key(&relation_id) {
            return Err(());
        }

        self.in_nodes
            .get_mut(&src)
            .unwrap()
            .get_mut(&dst)
            .unwrap()
            .remove(&relation_id);

        self.out_nodes
            .get_mut(&dst)
            .unwrap()
            .get_mut(&src)
            .unwrap()
            .remove(&relation_id);

        self.relation_data
            .get_mut(&relation_id)
            .unwrap()
            .edges
            .remove(&Edge::new(src, dst));

        Ok(())
    }

    pub fn nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn nr_relations(&self) -> usize {
        self.relation_data.len()
    }

    pub fn in_degree(&self, node_id: NodeId) -> Option<usize> {
        if self.in_nodes.contains_key(&node_id) {
            Some(self.in_nodes[&node_id].len())
        } else {
            None
        }
    }

    pub fn out_degree(&self, node_id: NodeId) -> Option<usize> {
        if self.out_nodes.contains_key(&node_id) {
            Some(self.out_nodes[&node_id].len())
        } else {
            None
        }
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn iter_relations(&self) -> impl Iterator<Item = &Relation> {
        self.relation_data.values().map(|r| &r.relation)
    }

    pub fn iter_relation_edges(
        &self,
        relation_id: RelationId,
    ) -> Option<impl Iterator<Item = &Edge>> {
        self.relation_data.get(&relation_id).map(|r| r.edges.iter())
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

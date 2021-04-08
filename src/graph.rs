use crate::node::{AnyNodeInfo, Node, NodeId};
use crate::relation::{Relation, RelationId};
use std::collections::{HashMap, HashSet};

struct Graph {
    in_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    out_nodes: HashMap<NodeId, HashMap<NodeId, HashSet<RelationId>>>,
    nodes: HashMap<NodeId, Node>,
    relations: HashMap<RelationId, Relation>,
    node_counter: u64,
    rel_counter: u64,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            in_nodes: HashMap::new(),
            out_nodes: HashMap::new(),
            nodes: HashMap::new(),
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

    pub fn remove_node(&mut self, node_id: &NodeId) -> Option<AnyNodeInfo> {
        if self.nodes.contains_key(node_id) {
            for node in self.in_nodes[node_id].keys() {
                if let Some(relations) = self.out_nodes.get_mut(node) {
                    relations.remove(node_id);
                }
            }
            self.in_nodes.remove(node_id);
            for node in self.out_nodes[node_id].keys() {
                if let Some(relations) = self.in_nodes.get_mut(node) {
                    relations.remove(node_id);
                }
            }
            self.out_nodes.remove(node_id);
            Some(self.nodes.remove(node_id)?.into_info())
        } else {
            None
        }
    }

    pub fn nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn nr_relations(&self) -> usize {
        self.relations.len()
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

    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node>  {
        self.nodes.values()
    }

    pub fn iter_relations(&self) -> impl Iterator<Item = &Relation> {
        self.relations.values()
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
        for node_id in &v[..10] {
            graph.remove_node(node_id);
        }
        assert_eq!(graph.nr_nodes(), 90);

        for node_id in &v[..10] {
            assert!(matches!(graph.remove_node(node_id), None));
        }
        for node_id in &v[10..] {
            assert!(graph.remove_node(node_id).is_some());
        }
        assert_eq!(graph.nr_nodes(), 0);
    }
}

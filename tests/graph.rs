// Used for integration testing. Don't touch this for now :)
use graphfruit::errors::ConnectError;
use graphfruit::graph::Graph;

#[test]
fn short_tests() {
    let mut graph = Graph::new();

    assert_eq!(graph.nr_nodes(), 0);
    let n1 = graph.add_node("Node1".to_string());
    assert_eq!(graph.nr_nodes(), 1);
    let n2 = graph.add_node("Node2".to_string());
    assert_eq!(graph.nr_nodes(), 2);
    assert!(n1 != n2);
    let n3 = graph.add_node("Node3".to_string());
    assert!(n1 != n3 && n2 != n3);
    assert_eq!(graph.nr_nodes(), 3);
    let n4 = graph.add_node("Node4".to_string());
    assert!(n1 != n4 && n2 != n4 && n3 != n4);
    assert_eq!(graph.nr_nodes(), 4);
    assert_eq!(graph.nr_relations(), 0);
    let r1 = graph.add_relation("Relation1".to_string());
    assert_eq!(graph.nr_relations(), 1);
    let r2 = graph.add_relation("Relation2".to_string());
    assert_eq!(graph.nr_relations(), 2);
    assert!(r1 != r2);

    assert!(graph.connect(n1, n2, r1).is_ok());
    assert!(graph.connect(n1, n4, r2).is_ok());
    assert!(graph.connect(n1, n4, r1).is_ok());

    assert_eq!(graph.in_degree_of(n1).unwrap(), 3);
    assert_eq!(graph.out_degree_of(n1).unwrap(), 0);
    assert_eq!(graph.out_degree_of(n2).unwrap(), 1);
    assert_eq!(graph.out_degree_of(n4).unwrap(), 2);

    assert!(graph.disconnect(n1, n4, r1).is_ok());
    assert_eq!(graph.in_degree_of(n1).unwrap(), 2);
    assert_eq!(graph.out_degree_of(n4).unwrap(), 1);
    assert_eq!(graph.disconnect(n1, n4, r1).unwrap(), false);
    assert_eq!(graph.disconnect(n3, n2, r1).unwrap(), false);

    assert!(graph.remove_node(n2).is_some());
    assert!(graph.remove_node(n2).is_none());
    assert_eq!(graph.in_degree_of(n1).unwrap(), 1);
    assert!(graph.in_degree_of(n2).is_none());
    assert!(graph.out_degree_of(n2).is_none());

    let mut count = 0;
    for elem in graph.iter_nodes() {
        count += 1;
    }
    assert_eq!(count, 3);

    count = 0;
    for rel in graph.iter_relations() {
        count += 1;
    }
    assert_eq!(count, 2);

    assert_eq!(graph.nr_relations(), 2);
    assert!(graph.remove_relation(r2).is_some());
    assert_eq!(graph.nr_relations(), 1);
    assert!(graph.remove_relation(r2).is_none());
    assert!(graph.connect(n1, n2, r1).is_err());
    assert!(graph.connect(n2, n3, r1).is_err());
    assert!(graph.connect(n1, n3, r2).is_err());
}

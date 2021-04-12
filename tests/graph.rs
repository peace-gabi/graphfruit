// Used for integration testing. Don't touch this for now :)
use graphfruit::errors::ConnectError;
use graphfruit::graph::Graph;
use std::collections::HashSet;

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

    assert_eq!(graph.out_degree_of(n1).unwrap(), 3);
    assert_eq!(graph.in_degree_of(n1).unwrap(), 0);
    assert_eq!(graph.in_degree_of(n2).unwrap(), 1);
    assert_eq!(graph.in_degree_of(n4).unwrap(), 2);

    assert!(graph.disconnect(n1, n4, r1).is_ok());
    assert_eq!(graph.out_degree_of(n1).unwrap(), 2);
    assert_eq!(graph.in_degree_of(n4).unwrap(), 1);
    assert_eq!(graph.disconnect(n1, n4, r1).unwrap(), false);
    assert_eq!(graph.disconnect(n3, n2, r1).unwrap(), false);

    assert!(graph.remove_node(n2).is_some());
    assert!(graph.remove_node(n2).is_none());
    assert_eq!(graph.out_degree_of(n1).unwrap(), 1);
    assert!(graph.in_degree_of(n2).is_none());
    assert!(graph.out_degree_of(n2).is_none());

    let mut count = 0;
    for _ in graph.iter_nodes() {
        count += 1;
    }
    assert_eq!(count, 3);

    count = 0;
    for _ in graph.iter_relations() {
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

#[test]
fn test_add_node() {
    let mut graph = Graph::new();
    let mut node_ids = HashSet::new();
    for i in 0..1000 {
        assert!(node_ids.insert(graph.add_node(i)));
    }
    assert_eq!(graph.nr_nodes(), 1000);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += node_info.downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 1000);
    assert_eq!(sum, 499500);
}

#[test]
fn test_add_relation() {
    let mut graph = Graph::new();
    let mut rel_ids = HashSet::new();
    for i in 0..1000 {
        assert!(rel_ids.insert(graph.add_relation(i)));
    }
    assert_eq!(graph.nr_relations(), 1000);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += relation.1.info().downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 1000);
    assert_eq!(sum, 499500);
}

#[test]
fn test_connect() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..100 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 100);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += node_info.downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    let mut rel_ids = Vec::new();
    for i in 0..100 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 100);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += relation.1.info().downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    for i in 0..100 {
        for j in 0..100 {
            assert_eq!(
                graph
                    .connect(node_ids[i], node_ids[j], rel_ids[(i + j) % 10])
                    .unwrap(),
                true
            );
        }
    }
    for i in 0..100 {
        assert_eq!(
            graph.in_degree_of(node_ids[i]),
            graph.out_degree_of(node_ids[i])
        );
    }
    for i in 0..100 {
        for j in 0..100 {
            assert_eq!(
                graph
                    .connect(node_ids[i], node_ids[j], rel_ids[(i + j) % 10])
                    .unwrap(),
                false
            );
            assert_eq!(
                graph
                    .connect(node_ids[i], node_ids[j], rel_ids[(i + j) % 10 + 1])
                    .unwrap(),
                true
            );
        }
    }

    graph.remove_node(node_ids[0]);
    graph.remove_relation(rel_ids[0]);
    for i in 1..100 {
        assert!(matches!(
            graph.connect(node_ids[i], node_ids[0], rel_ids[11]),
            Err(ConnectError::InvalidDstNodeId)
        ));
        assert!(matches!(
            graph.connect(node_ids[0], node_ids[i], rel_ids[11]),
            Err(ConnectError::InvalidSrcNodeId)
        ));
        for j in 1..100 {
            assert!(matches!(
                graph.connect(node_ids[i], node_ids[j], rel_ids[0]),
                Err(ConnectError::InvalidRelationId)
            ));
        }
    }
}

#[test]
fn test_disconnect() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..100 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 100);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += node_info.downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    let mut rel_ids = Vec::new();
    for i in 0..100 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 100);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += relation.1.info().downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    for i in 0..100 {
        assert_eq!(
            graph.connect(node_ids[i], node_ids[0], rel_ids[0]).unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[i], node_ids[10], rel_ids[10])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[i], node_ids[20], rel_ids[20])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[i], node_ids[30], rel_ids[30])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[i], node_ids[40], rel_ids[40])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[0], node_ids[(i + 15) % 100], rel_ids[5])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[10], node_ids[(i + 15) % 100], rel_ids[15])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[20], node_ids[(i + 15) % 100], rel_ids[25])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[30], node_ids[(i + 15) % 100], rel_ids[35])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .connect(node_ids[40], node_ids[(i + 15) % 100], rel_ids[45])
                .unwrap(),
            true
        );
    }
    assert_eq!(graph.in_degree_of(node_ids[0]).unwrap(), 105);
    assert_eq!(graph.in_degree_of(node_ids[10]).unwrap(), 105);
    assert_eq!(graph.in_degree_of(node_ids[20]).unwrap(), 105);
    assert_eq!(graph.in_degree_of(node_ids[30]).unwrap(), 105);
    assert_eq!(graph.in_degree_of(node_ids[40]).unwrap(), 105);
    assert_eq!(graph.out_degree_of(node_ids[0]).unwrap(), 105);
    assert_eq!(graph.out_degree_of(node_ids[10]).unwrap(), 105);
    assert_eq!(graph.out_degree_of(node_ids[20]).unwrap(), 105);
    assert_eq!(graph.out_degree_of(node_ids[30]).unwrap(), 105);
    assert_eq!(graph.out_degree_of(node_ids[40]).unwrap(), 105);
    for i in 0..100 {
        assert_eq!(
            graph
                .disconnect(node_ids[i], node_ids[0], rel_ids[0])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .disconnect(node_ids[0], node_ids[(i + 15) % 100], rel_ids[5])
                .unwrap(),
            true
        );
        assert_eq!(
            graph
                .disconnect(node_ids[i], node_ids[0], rel_ids[0])
                .unwrap(),
            false
        );
        assert_eq!(
            graph
                .disconnect(node_ids[0], node_ids[(i + 15) % 100], rel_ids[5])
                .unwrap(),
            false
        );
        assert_eq!(
            graph
                .disconnect(node_ids[i], node_ids[0], rel_ids[1])
                .unwrap(),
            false
        );
        assert_eq!(
            graph
                .disconnect(node_ids[0], node_ids[(i + 15) % 100], rel_ids[1])
                .unwrap(),
            false
        );
    }
    assert_eq!(graph.in_degree_of(node_ids[0]).unwrap(), 4);
    assert_eq!(graph.out_degree_of(node_ids[0]).unwrap(), 4);
    for i in 1..10 {
        for j in 1..10 {
            for k in 1..100 {
                assert_eq!(
                    graph
                        .disconnect(node_ids[i], node_ids[j], rel_ids[k])
                        .unwrap(),
                    false
                );
            }
        }
    }
    assert!(graph.remove_node(node_ids[0]).is_some());
    assert!(graph.remove_relation(rel_ids[5]).is_some());
    for i in 1..100 {
        assert!(matches!(
            graph.connect(node_ids[i], node_ids[0], rel_ids[11]),
            Err(ConnectError::InvalidDstNodeId)
        ));
        assert!(matches!(
            graph.connect(node_ids[0], node_ids[i], rel_ids[11]),
            Err(ConnectError::InvalidSrcNodeId)
        ));
        for j in 1..100 {
            assert!(matches!(
                graph.connect(node_ids[i], node_ids[j], rel_ids[5]),
                Err(ConnectError::InvalidRelationId)
            ));
        }
    }
}

#[test]
fn test_remove_node() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..100 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 100);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += node_info.downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    let mut rel_ids = Vec::new();
    for i in 0..10 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 10);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += relation.1.info().downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 10);
    assert_eq!(sum, 45);
    for i in 0..90 {
        for j in (i / 10 + 1) * 10..(i / 10 + 2) * 10 {
            assert_eq!(
                graph
                    .connect(node_ids[i], node_ids[j], rel_ids[i / 10])
                    .unwrap(),
                true
            );
        }
    }
    for i in 0..10 {
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 10);
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 0);
        assert_eq!(graph.out_degree_of(node_ids[i + 90]).unwrap(), 0);
        assert_eq!(graph.in_degree_of(node_ids[i + 90]).unwrap(), 10);
    }
    for i in 10..90 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 10);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 10);
    }
    assert!(graph.remove_node(node_ids[0]).is_some());
    assert!(graph.out_degree_of(node_ids[0]).is_none());
    assert!(graph.in_degree_of(node_ids[0]).is_none());
    assert!(graph.remove_node(node_ids[0]).is_none());
    for i in 10..20 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 9);
    }
    for i in (10..90).step_by(10) {
        assert!(graph.remove_node(node_ids[i + i / 10]).is_some());
        assert!(graph.out_degree_of(node_ids[i + i / 10]).is_none());
        assert!(graph.in_degree_of(node_ids[i + i / 10]).is_none());
        assert!(graph.remove_node(node_ids[i + i / 10]).is_none());
        for j in (i + 10)..(i + 20) {
            assert_eq!(graph.in_degree_of(node_ids[j]).unwrap(), 9);
        }
    }
    assert!(graph.remove_node(node_ids[99]).is_some());
    assert!(graph.out_degree_of(node_ids[99]).is_none());
    assert!(graph.in_degree_of(node_ids[99]).is_none());
    assert!(graph.remove_node(node_ids[99]).is_none());
}

#[test]
fn test_remove_relation() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..10 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 10);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += node_info.downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 10);
    assert_eq!(sum, 45);
    let mut rel_ids = Vec::new();
    for i in 0..100 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 100);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += relation.1.info().downcast_ref::<i32>().unwrap();
    }
    assert_eq!(count, 100);
    assert_eq!(sum, 4950);
    let mut in_deg: [usize; 10] = [0; 10];
    let mut out_deg: [usize; 10] = [0; 10];
    for i in 0..9 {
        for j in i * 10..(i * 10 + 10) {
            for k in (i + 1)..10 {
                assert_eq!(
                    graph.connect(node_ids[i], node_ids[k], rel_ids[j]).unwrap(),
                    true
                );
                out_deg[i] += 1;
                in_deg[k] += 1;
            }
        }
    }
    for i in 0..10 {
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), out_deg[i]);
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), in_deg[i]);
    }
    for i in (0..100).step_by(11) {
        assert!(graph.remove_relation(rel_ids[i]).is_some());
        assert!(graph.remove_relation(rel_ids[i]).is_none());
        out_deg[i / 10] -= 9 - i / 10;
        for j in (i / 10 + 1)..10 {
            in_deg[j] -= 1;
        }
        for i in 0..10 {
            assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), out_deg[i]);
            assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), in_deg[i]);
        }
    }
}

#[test]
fn test_quantity() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..100000 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 100000);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += *node_info.downcast_ref::<i32>().unwrap() as i64;
    }
    assert_eq!(count, 100000);
    assert_eq!(sum, 4999950000);
    let mut rel_ids = Vec::new();
    for i in 0..100000 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 100000);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += *relation.1.info().downcast_ref::<i32>().unwrap() as i64;
    }
    assert_eq!(count, 100000);
    assert_eq!(sum, 4999950000);
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                assert_eq!(
                    graph.connect(node_ids[i], node_ids[j], rel_ids[k]).unwrap(),
                    true
                );
            }
        }
    }
    for i in 0..100 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 10000);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 10000);
    }
    for i in 0..50 {
        assert!(graph.remove_relation(rel_ids[i]).is_some());
        assert!(graph.remove_relation(rel_ids[i]).is_none());
    }
    for i in 0..100 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 5000);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 5000);
    }
    for i in 100..200 {
        for j in 0..100 {
            for k in 1000..1050 {
                assert_eq!(
                    graph.connect(node_ids[i], node_ids[j], rel_ids[k]).unwrap(),
                    true
                );
            }
        }
    }
    for i in 0..100 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 10000);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 5000);
    }
    for i in 100..200 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 0);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 5000);
    }
    for i in 200..100000 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 0);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 0);
    }
    for i in 0..50 {
        assert!(graph.remove_node(node_ids[i]).is_some());
        assert!(graph.remove_node(node_ids[i]).is_none());
    }
    for i in 50..100 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 7500);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 2500);
    }
    for i in 100..200 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 0);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 2500);
    }
    for i in 100..200 {
        for j in 50..100 {
            for k in 1000..1050 {
                assert_eq!(
                    graph
                        .disconnect(node_ids[i], node_ids[j], rel_ids[k])
                        .unwrap(),
                    true
                );
            }
        }
    }
    for i in 50..100 {
        assert_eq!(graph.in_degree_of(node_ids[i]).unwrap(), 2500);
        assert_eq!(graph.out_degree_of(node_ids[i]).unwrap(), 2500);
    }
}

#[test]
fn test_iterator() {
    let mut graph = Graph::new();
    let mut node_ids = Vec::new();
    for i in 0..100000 {
        node_ids.push(graph.add_node(i));
    }
    assert_eq!(graph.nr_nodes(), 100000);
    let mut count = 0;
    let mut sum = 0;
    for node_info in graph.iter_nodes() {
        count += 1;
        sum += *node_info.downcast_ref::<i32>().unwrap() as i64;
    }
    assert_eq!(count, 100000);
    assert_eq!(sum, 4999950000);
    let mut rel_ids = Vec::new();
    for i in 0..100000 {
        rel_ids.push(graph.add_relation(i));
    }
    assert_eq!(graph.nr_relations(), 100000);
    let mut count = 0;
    let mut sum = 0;
    for relation in graph.iter_relations() {
        count += 1;
        sum += *relation.1.info().downcast_ref::<i32>().unwrap() as i64;
    }
    assert_eq!(count, 100000);
    assert_eq!(sum, 4999950000);
    for k in 0..100 {
        for i in 0..10 {
            for j in 1000..1020 {
                assert_eq!(
                    graph.connect(node_ids[i], node_ids[j], rel_ids[k]).unwrap(),
                    true
                );
            }
        }
    }
    for k in 0..100 {
        let mut count = 0;
        for _ in graph.iter_relation_edges(rel_ids[k]).unwrap() {
            count += 1;
        }
        assert_eq!(count, 200);
    }
}

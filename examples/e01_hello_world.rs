use graphfruit::graph::Graph;

fn main() {
    let mut graph = Graph::new();
    let n1 = graph.add_node("hello".to_string());
    let n2 = graph.add_node("world".to_string());
    let r1 = graph.add_relation(10);

    graph.connect(n1, n2, r1).unwrap();
    graph.connect(n1, n1, r1).unwrap();

    graph.disconnect(n1, n2, r1).unwrap();

    for edge in graph.iter_relation_edges(r1).unwrap() {
        println!("{} -> {}", edge.src(), edge.dst());
    }
}

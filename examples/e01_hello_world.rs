use graphfruit::node::{Node, NodeId};
use graphfruit::relation::{Relation, RelationId};

fn main() {
    let n1 = Node::new(NodeId::new(10), 100);
    let n2 = Node::new(NodeId::new(20), "Eyy".to_string());

    let r1 = Relation::new(RelationId::new(1), n1.id(), n2.id(), 1);
    let r2 = Relation::new(RelationId::new(2), n2.id(), n1.id(), "Yoo".to_string());

    println!(
        "Relation 1: Id: {}, Src: {}, Dst: {}",
        r1.id(),
        r1.src(),
        r1.dst()
    );
    println!(
        "Relation 2: Id: {}, Src: {}, Dst: {}",
        r2.id(),
        r2.src(),
        r2.dst()
    );
}

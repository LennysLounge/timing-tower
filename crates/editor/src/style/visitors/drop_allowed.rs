use backend::style::iterator::Node;

pub fn drop_allowed(target: Node, dragged: Node) -> bool {
    match (target, dragged) {
        (Node::VariableFolder(_), Node::VariableFolder(_)) => true,
        (Node::VariableFolder(_), Node::Variable(_)) => true,

        (Node::AssetFolder(_), Node::AssetFolder(_)) => true,
        (Node::AssetFolder(_), Node::Asset(_)) => true,

        (Node::TimingTowerRow(_), Node::TimingTowerColumnFolder(_)) => true,
        (Node::TimingTowerRow(_), Node::TimingTowerColumn(_)) => true,

        (Node::TimingTowerColumnFolder(_), Node::TimingTowerColumnFolder(_)) => true,
        (Node::TimingTowerColumnFolder(_), Node::TimingTowerColumn(_)) => true,

        _ => false,
    }
}

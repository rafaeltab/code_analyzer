pub trait FromTreeSitterNode {
    fn from_ts_node(ts_node: tree_sitter::Node) -> Self;
}

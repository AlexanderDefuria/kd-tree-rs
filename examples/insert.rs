extern crate kd_tree_rs;

use kd_tree_rs::KdNode;
use kd_tree_rs::KdNode::Empty;
use kd_tree_rs::point::Point;

fn main() {
    let mut node: KdNode<i32> = KdNode::new();
    assert_eq!(node, Empty);

    // Tree Root
    node.insert(1, 1);
    node.insert(2, 2);
    node.insert(2, -12);

    println!("{:?}", node);
    println!("{:?}", node.nearest_neighbor_x_y(1, 1, 1.0));
    println!("{:?}", node.nearest_neighbor(Point{x: 1, y: 1}, 1.0));
}
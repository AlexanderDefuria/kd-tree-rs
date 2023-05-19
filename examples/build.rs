extern crate kd_tree_rs;

use kd_tree_rs::KdNode;
use kd_tree_rs::point::Point;

fn main() {
    let points: Vec<Point<i32>> = vec![
        Point { x: 1, y: 8 },
        Point { x: 2, y: 2 },
        Point { x: 3, y: 6 },
        Point { x: 4, y: 9 },
        Point { x: 7, y: 3 },
        Point { x: 8, y: 8 },
        Point { x: 9, y: 1 },
        Point { x: 9, y: 9 },
    ];

    let node: KdNode<i32> = KdNode::build(points);

    let radius: f64 = 1.5;
    let origin: Point<i32> = Point { x: 8, y: 8 };
    let nearest = node.nearest_neighbor(origin, radius);
    assert_eq!(
        nearest,
        vec![Point { x: 8, y: 8 }, Point { x: 9, y: 9 }]
    );
    println!("Neighbours within 1.5 units of (1,1): {:?}", nearest);
}
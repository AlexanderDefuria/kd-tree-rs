#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use crate::*;
    const TEST_POINTS: &[Point<i32>] = &[
        Point { x: 1, y: 8 }, // P1
        Point { x: 2, y: 2 }, // P2
        Point { x: 3, y: 6 }, // P3
        Point { x: 4, y: 9 }, // P4
        Point { x: 7, y: 3 }, // P5
        Point { x: 8, y: 8 }, // P6
        Point { x: 9, y: 1 }, // P7
        Point { x: 9, y: 9 }, // P8
    ];

    #[test]
    fn test_insert_empty() {
        let mut node: KdNode<i32> = KdNode::new();
        assert_eq!(node, Empty);

        // Tree Root
        node.insert(Point{ x: 1, y: 1 });
        // Second level of tree (sorted on Y)
        node.insert(Point{ x: 2, y: 2 });
        node.insert(Point{ x: 2, y: -12 });
        assert_eq!(node, Node {
            point: Point { x: 1, y: 1 },
            dim: Dim::X,
            right: Box::new(Node {
                point: Point { x: 2, y: -12 },
                dim: Dim::Y,
                left: Box::new(Empty),
                right: Box::new(Empty)
            }),
            left: Box::new(Node {
                point: Point { x: 2, y: 2 },
                dim: Dim::Y,
                left: Box::new(Empty),
                right: Box::new(Empty)
            })
        });
    }

    #[test]
    fn test_build() {
        // TODO Link this to the example in the README
        // https://www.notion.so/Kd-Tree-05fbc8e0c3b74831b81505082c51dcb3?pvs=4#bf8bbcc5e46645808424747cd4874480
        let points: Vec<Point<i32>> = TEST_POINTS.to_vec();

        let node: KdNode<i32> = KdNode::build(points, 1);

        assert_eq!(node, Node {
            point: Point { x: 4, y: 9 },
            dim: Dim::X,
            right: Box::new(Node {
                point: Point { x: 3, y: 6 },
                dim: Dim::Y,
                left: Box::new(Node {
                    point: Point { x: 1, y: 8 },
                    dim: Dim::X,
                    left: Box::new(Empty),
                    right: Box::new(Empty)
                }),
                right: Box::new(Node {
                    point: Point { x: 2, y: 2 },
                    dim: Dim::X,
                    left: Box::new(Empty),
                    right: Box::new(Empty)
                })
            }),
            left: Box::new(Node {
                point: Point { x: 7, y: 3 },
                dim: Dim::Y,
                right: Box::new( Node {
                    point: Point { x: 9, y: 1 },
                    dim: Dim::X,
                    left: Box::new(KdNode::Empty),
                    right: Box::new(KdNode::Empty)
                }),
                left: Box::new(Node {
                    point: Point { x: 8, y: 8 },
                    dim: Dim::X,
                    right: Box::new(KdNode::Empty),
                    left: Box::new(Node {
                        point: Point { x: 9, y: 9 },
                        dim: Dim::Y,
                        left: Box::new(KdNode::Empty),
                        right: Box::new(KdNode::Empty)
                    })
                })
            })
        })
    }

    #[test]
    fn test_cmp_points() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = Point { x: 1, y: 2 };
        let p4 = Point { x: 2, y: 1 };

        assert_eq!(p1.cmp(&p2, &Dim::X), Ordering::Less);
        assert_eq!(p1.cmp(&p2, &Dim::Y), Ordering::Less);
        assert_eq!(p1.cmp(&p3, &Dim::X), Ordering::Equal);
        assert_eq!(p1.cmp(&p3, &Dim::Y), Ordering::Less);
        assert_eq!(p1.cmp(&p4, &Dim::X), Ordering::Less);
        assert_eq!(p1.cmp(&p4, &Dim::Y), Ordering::Equal);
    }

    #[test]
    fn test_split_on_median() {
        let points: Vec<Point<i32>> = TEST_POINTS.to_vec();

        let (median, left, right): (Point<i32>, Vec<Point<i32>>, Vec<Point<i32>>)
            = KdNode::_split_on_median(points, &Dim::X);

        assert_eq!(median, Point { x: 4, y: 9 });
        assert_eq!(right, vec![
            Point { x: 1, y: 8 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 6 },
        ]);
        assert_eq!(left, vec![
            Point { x: 7, y: 3 },
            Point { x: 8, y: 8 },
            Point { x: 9, y: 1 },
            Point { x: 9, y: 9 },
        ]);

        let (median_left, left_left, right_left): (Point<i32>, Vec<Point<i32>>, Vec<Point<i32>>)
            = KdNode::_split_on_median(right, &Dim::Y);
        assert_eq!(median_left, Point { x: 3, y: 6 });
        assert_eq!(left_left, vec![Point { x: 1, y: 8 }]);
        assert_eq!(right_left, vec![Point { x: 2, y: 2 }]);
    }
}

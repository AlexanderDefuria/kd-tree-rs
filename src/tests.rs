#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use crate::dim::Dim;
    use crate::point::Point;
    use crate::*;

    const TEST_POINTS_I32: &[Point<i32>] = &[
        Point { x: 1, y: 8 }, // P1
        Point { x: 2, y: 2 }, // P2
        Point { x: 3, y: 6 }, // P3
        Point { x: 4, y: 9 }, // P4
        Point { x: 7, y: 3 }, // P5
        Point { x: 8, y: 8 }, // P6
        Point { x: 9, y: 1 }, // P7
        Point { x: 9, y: 9 }, // P8
    ];

    const TEST_POINTS_F64: &[Point<f64>] = &[
        Point { x: 1., y: 8. },
        Point { x: 2., y: 2. },
        Point { x: 3., y: 6. },
        Point { x: 4., y: 9. },
        Point { x: 7., y: 3. },
        Point { x: 8., y: 8. },
        Point { x: 9., y: 1. },
        Point { x: 9., y: 9. },
    ];

    #[test]
    fn test_insert_empty() {
        let mut node: KdNode<i32> = KdNode::new();
        assert_eq!(node, Empty);

        // Tree Root
        node.insert(1, 1);
        // Second level of tree (sorted on Y)
        node.insert_point(Point { x: 2, y: 2 });
        node.insert_point(Point { x: 2, y: -12 });
        assert_eq!(
            node,
            Node {
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
            }
        );
    }

    #[test]
    fn test_build() {
        // TODO Link this to the example in the README
        // https://www.notion.so/Kd-Tree-05fbc8e0c3b74831b81505082c51dcb3?pvs=4#bf8bbcc5e46645808424747cd4874480
        let points: Vec<Point<i32>> = TEST_POINTS_I32.to_vec();

        let node: KdNode<i32> = KdNode::build(points);

        assert_eq!(
            node,
            Node {
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
                    right: Box::new(Node {
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
            }
        )
    }

    #[test]
    fn test_split_on_median() {
        let points: Vec<Point<i32>> = TEST_POINTS_I32.to_vec();

        let (median, left, right): (Point<i32>, Vec<Point<i32>>, Vec<Point<i32>>) =
            KdNode::split_on_median(points, &Dim::X);

        assert_eq!(median, Point { x: 4, y: 9 });
        assert_eq!(
            right,
            vec![Point { x: 1, y: 8 }, Point { x: 2, y: 2 }, Point { x: 3, y: 6 },]
        );
        assert_eq!(
            left,
            vec![
                Point { x: 7, y: 3 },
                Point { x: 8, y: 8 },
                Point { x: 9, y: 1 },
                Point { x: 9, y: 9 },
            ]
        );

        let (median_left, left_left, right_left): (Point<i32>, Vec<Point<i32>>, Vec<Point<i32>>) =
            KdNode::split_on_median(right, &Dim::Y);
        assert_eq!(median_left, Point { x: 3, y: 6 });
        assert_eq!(left_left, vec![Point { x: 1, y: 8 }]);
        assert_eq!(right_left, vec![Point { x: 2, y: 2 }]);
    }

    #[test]
    fn test_nearest_neighbor() {
        let node: KdNode<f64> = KdNode::build(TEST_POINTS_F64.to_vec());
        let mut origin: Point<f64> = Point { x: 1.5, y: 2. };
        let mut radius: f64 = 1.;
        let mut nearest: Vec<Point<f64>> = node.nearest_neighbor(origin, radius);
        assert_eq!(nearest, vec![Point { x: 2., y: 2. }]); // This is the best node

        radius = 1.5;
        origin = Point { x: 8.1, y: 8.1 };
        nearest = node.nearest_neighbor(origin, radius);
        assert_eq!(
            nearest,
            vec![Point { x: 8., y: 8. }, Point { x: 9., y: 9. }]
        );

        radius = 100.;
        origin = Point { x: 0., y: 0. };
        nearest = node.nearest_neighbor(origin, radius);
        for point in TEST_POINTS_F64.to_vec() {
            assert!(nearest.contains(&point))
        }

        nearest = node.n_nearest_neighbor(origin, 4);
        for point in nearest.clone() {
            assert!(TEST_POINTS_F64.to_vec().contains(&point))
        }
        assert_eq!(nearest.len(), 4);


    }

    #[test]
    fn test_insert_sorted() {
        let mut points: Vec<(&KdNode<f64>, f64)> = vec![(&Empty, 1.), (&Empty, 2.), (&Empty, 3.)];
        KdNode::insert_sorted(&mut points, (&Empty, 0.));
        KdNode::insert_sorted(&mut points, (&Empty, 2.5));
        KdNode::insert_sorted(&mut points, (&Empty, 3.5));
        KdNode::insert_sorted(&mut points, (&Empty, 5.));
        assert_eq!(
            points,
            vec![
                (&Empty, 0.),
                (&Empty, 1.),
                (&Empty, 2.),
                (&Empty, 2.5),
                (&Empty, 3.),
                (&Empty, 3.5),
                (&Empty, 5.),
            ]
        );
    }

    #[test]
    fn test_drill_down() {
        let node: KdNode<f64> = KdNode::build(TEST_POINTS_F64.to_vec());

        match node.drill_down(Point { x: 0., y: 0. }).pop().unwrap() {
            Empty => {
                panic!()
            }
            Node { point, .. } => {
                assert_eq!(*point, Point { x: 2., y: 2. });
            }
        }
    }
}

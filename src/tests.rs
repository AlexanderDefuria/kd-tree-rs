#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_median() {
        let mut point_set: Vec<Point<usize>> = vec![Point { x: 1, y: 1 }];
        assert_eq!(*median_of_points(X, &mut point_set).0, Point { x: 1, y: 1 });
        assert_eq!(*median_of_points(Y, &mut point_set).0, Point { x: 1, y: 1 });

        point_set = vec![
            Point { x: 0, y: 2 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 1 },
        ];
        assert_eq!(*median_of_points(X, &mut point_set).0, Point { x: 1, y: 1 });
        assert_eq!(*median_of_points(Y, &mut point_set).0, Point { x: 2, y: 1 });

        point_set = vec![
            Point { x: 5, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 7 },
            Point { x: 2, y: 1 },
        ];
        assert_eq!(*median_of_points(X, &mut point_set).0, Point { x: 2, y: 1 });
        assert_eq!(*median_of_points(Y, &mut point_set).0, Point { x: 5, y: 2 });
    }

    #[test]
    fn test_median_sort() {
        let mut point_set: Vec<Point<i32>> = vec![
            Point { x: 5, y: 2 },
            Point { x: -1, y: 2 },
            Point { x: 1, y: 7 },
            Point { x: 2, y: 1 },
        ];
        let q = median_of_points(X, &mut point_set);
        assert_eq!(
            point_set,
            vec![
                Point { x: -1, y: 2 },
                Point { x: 1, y: 7 },
                Point { x: 2, y: 1 },
                Point { x: 5, y: 2 }
            ]
        )
    }

    #[test]
    fn test_split() {
        let mut point_set: Vec<Point<i32>> = vec![
            Point { x: 5, y: 2 },
            Point { x: -1, y: 2 },
            Point { x: 1, y: 7 },
        ];
        assert_eq!(
            (
                vec![Point { x: 5, y: 2 }],
                vec![Point { x: -1, y: 2 }, Point { x: 1, y: 7 }]
            ),
            split_point_set(1, X, &mut point_set)
        );
        point_set = vec![
            Point { x: -1, y: 2 },
            Point { x: 1, y: 7 },
            Point { x: 2, y: 1 },
            Point { x: 5, y: 2 },
        ];
        assert_eq!(
            (
                vec![Point { x: -1, y: 2 }, Point { x: 1, y: 7 }],
                vec![Point { x: 2, y: 1 }, Point { x: 5, y: 2 }]
            ),
            split_point_set(2, X, &mut point_set)
        );
    }
}

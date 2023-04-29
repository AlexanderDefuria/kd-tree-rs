mod tests;

use std::cmp::Ordering;
use crate::KdNode::{Empty, Node};

#[derive(Debug, PartialEq)]
enum KdNode<T: PartialEq> {
    Empty,
    Node {
        point: Point<T>,
        dim: Dim,
        left: Box<KdNode<T>>,
        right: Box<KdNode<T>>,
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point<T: PartialEq> {
    x: T,
    y: T
}

#[derive(Debug, PartialEq)]
enum Dim {
    X,
    Y,
}


impl Dim {
    const DIMENSIONS: usize = 2;

    fn from_depth(mut n: usize) -> Dim {
        n = n.rem_euclid(Dim::DIMENSIONS);
        assert!(n < Dim::DIMENSIONS);
        match n {
            0 => Dim::X,
            1 => Dim::Y,
            _ => panic!("Higher dimensions not implemented")
        }
    }
}

impl<T: PartialEq + PartialOrd> Point<T> {
    fn gt(&self, rs: &Point<T>, dim: &Dim) -> bool {
        match dim {
            Dim::X => self.x > rs.x,
            Dim::Y => self.y > rs.y,
        }
    }

    fn cmp(&self, rs: &Point<T>, dim: &Dim) -> Ordering {
        let ls_value: &T = self.get_dim_value(dim);
        let rs_value: &T = rs.get_dim_value(dim);
        ls_value.partial_cmp(&rs_value).unwrap()
    }

    fn get_dim_value(&self, dim: &Dim) -> &T {
        match dim {
            Dim::X => &self.x,
            Dim::Y => &self.y,
        }
    }
}

impl<T: PartialEq + PartialOrd> KdNode<T> {
    fn new() -> Self {
        Empty
    }

    fn insert(&mut self, item: Point<T>) -> &Self {
        self._insert(item, 0);
        return self;
    }

    fn _insert(&mut self, item: Point<T>, depth: usize) -> &Self {
        *self = match self {
            Empty => Node {
                point: item,
                dim: Dim::from_depth(depth),
                left: Box::new(Empty),
                right: Box::new(Empty),
            },
            Node {point, left, right, ..} => {
                let next_depth: usize = depth + 1;
                if point.gt(&item, &Dim::from_depth(next_depth)) {
                    right._insert(item, next_depth);
                } else {
                    left._insert(item, next_depth);
                }
                return self
            },
        };

        self
    }
}

impl<T: PartialEq + PartialOrd + Clone> KdNode<T> {
    fn build( points: Vec<Point<T>>, depth: usize) -> Self {
        if points.is_empty() {
            return Empty;
        } else if points.len() == 1 {
            return Node {
                point: points[0].clone(),
                dim: Dim::X,
                left: Box::new(Empty),
                right: Box::new(Empty),
            };
        }
        let next_depth: usize = depth + 1;

        // Choose axis
        let axis = Dim::from_depth(next_depth);

        // Get Median
        let (median, left, right): (Point<T>, Vec<Point<T>>, Vec<Point<T>>)
            = KdNode::_split_on_median(points, &axis);

        Node {
            point: median,
            dim: axis,
            left: Box::from(Self::build(left, next_depth)),
            right: Box::from(Self::build(right, next_depth)),
        }
    }

    fn _split_on_median(mut points: Vec<Point<T>>, axis: &Dim) -> (Point<T>, Vec<Point<T>>, Vec<Point<T>>) {
        points.sort_by(|a: &Point<T>, b: &Point<T>| a.cmp(&b, &axis));
        let median_index: usize = if points.len() % 2 == 0 { points.len() / 2 - 1 } else { points.len() / 2 };
        let median: Point<T> = points[median_index].clone();
        let right: Vec<Point<T>> = points.drain(..median_index).collect();
        let left: Vec<Point<T>> = points.drain(1..).collect();
        (median, left, right)
    }
}

extern crate core;

mod dim;
mod point;
mod tests;

use crate::dim::Dim;
use crate::point::{distance, Point};
use crate::KdNode::{Empty, Node};
use std::cmp::Ordering;
use std::ops::{Add, Deref, Mul, Sub};

trait KDT: PartialEq + PartialOrd + Copy + Mul + Sub + Add + Into<f64> {}
impl<T> KDT for T where
    T: PartialEq
        + PartialOrd
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Into<f64>
{
}

#[derive(Debug, PartialEq)]
enum KdNode<T: KDT> {
    Empty,
    Node {
        point: Point<T>,
        dim: Dim,
        left: Box<KdNode<T>>,
        right: Box<KdNode<T>>,
    },
}

impl<T: KDT + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> KdNode<T> {
    /// Create a new empty tree
    fn new() -> Self {
        Empty
    }

    /// Insert a new item into the tree
    ///
    /// This should used sparingly as it can unbalance the tree
    /// and reduce performance. If there is a large change to the dataset
    /// it is better to create a new tree.
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
            Node {
                point, left, right, ..
            } => {
                let next_depth: usize = depth + 1;
                if point.gt(&item, &Dim::from_depth(next_depth)) {
                    right._insert(item, next_depth);
                } else {
                    left._insert(item, next_depth);
                }
                return self;
            }
        };

        self
    }

    /// Find the nearest neighbors to the origin point
    fn nearest_neighbor<'a>(&self, origin: Point<T>, radius: f64) -> Vec<Point<T>> {
        let mut best_queue: Vec<(&KdNode<T>, f64)> = Vec::new();
        let mut best_node: &KdNode<T> = self;

        // Find the best leaf node.
        let mut parents: Vec<&KdNode<T>> = self.drill_down(origin.clone());
        best_node = parents[0];


        // 4. Traverse the tree upwards from the leaf node
        while let Some(parent) = parents.pop() {
            match parent {
                Empty => {
                    break;
                }
                Node { point, left, right, .. } => {
                    // 4.a. Put at proper point in the best_queue
                    KdNode::insert_sorted(&mut best_queue, (parent, distance(&origin, point)));
                    // 4.b. Check if there could be better points on the other side of the parent
                    // This means check if the radius extends over the parent's split plane
                    // 4.b.i. Check if the radius extends over the parent's split plane
                    let (split_plane, split_plane_value, origin_value) = match parent {
                        Node {
                            dim: Dim::X, point, ..
                        } => (Dim::X, point.x.into(), origin.x.into()),
                        Node {
                            dim: Dim::Y, point, ..
                        } => (Dim::Y, point.y.into(), origin.y.into()),
                        _ => panic!("Higher dimensions not implemented"),
                    };



                    // parents.push(parent);
                    // parents.push(other_side);

                }
            }
        }

        best_queue.retain(|(_, dist)| *dist <= radius);
        return best_queue
            .iter()
            .map(|(node, _)| match node {
                Node { point, .. } => point.clone(),
                _ => panic!("Empty node in best queue"),
            })
            .collect();
    }

    fn drill_down(&self, origin: Point<T>) -> Vec<&KdNode<T>> {
        let mut parents: Vec<&KdNode<T>> = Vec::new();
        let mut best_node: &KdNode<T> = self;
        while let Node {
            point,
            left,
            right,
            dim,
        } = best_node
        {
            parents.push(best_node);
            if *left == Box::new(Empty) && *right == Box::new(Empty) {
                break;
            }

            match point.cmp(&origin, dim) {
                Ordering::Less => best_node = left,
                _ => best_node = right,
            }
        }
        return parents
    }

    fn insert_sorted<'a>(points: &mut Vec<(&'a KdNode<T>, f64)>, point: (&'a KdNode<T>, f64)) {
        let mut index: usize = 0;
        for (i, (_, dist)) in points.iter().enumerate() {
            if *dist < point.1 {
                index = i + 1;
            }
        }
        points.insert(index, point);
    }

    fn build(points: Vec<Point<T>>) -> Self {
        KdNode::_build(points, 1)
    }

    fn _build(points: Vec<Point<T>>, depth: usize) -> Self {
        // Increment the dimension
        let next_depth: usize = depth + 1;

        // End recursion if there are one or no points
        if points.is_empty() {
            return Empty;
        } else if points.len() == 1 {
            return Node {
                point: points[0].clone(),
                dim: Dim::from_depth(next_depth),
                left: Box::new(Empty),
                right: Box::new(Empty),
            };
        }

        // Choose axis
        let axis = Dim::from_depth(next_depth);

        // Get Median
        let (median, left, right): (Point<T>, Vec<Point<T>>, Vec<Point<T>>) =
            KdNode::split_on_median(points, &axis);

        Node {
            point: median,
            dim: axis,
            left: Box::from(Self::_build(left, next_depth)),
            right: Box::from(Self::_build(right, next_depth)),
        }
    }

    /// Split the points into two vectors based on the median
    ///
    /// The median is chosen based on the axis and returned along with
    /// two separate vectors of points, the left and right of the median.
    fn split_on_median(
        mut points: Vec<Point<T>>,
        axis: &Dim,
    ) -> (Point<T>, Vec<Point<T>>, Vec<Point<T>>) {
        points.sort_by(|a: &Point<T>, b: &Point<T>| a.cmp(&b, &axis));
        let median_index: usize = if points.len() % 2 == 0 {
            points.len() / 2 - 1
        } else {
            points.len() / 2
        };
        let median: Point<T> = points[median_index].clone();
        let right: Vec<Point<T>> = points.drain(..median_index).collect();
        let left: Vec<Point<T>> = points.drain(1..).collect();
        (median, left, right)
    }
}

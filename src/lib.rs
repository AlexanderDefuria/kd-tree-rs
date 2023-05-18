extern crate core;

mod dim;
mod point;
mod tests;

use crate::dim::Dim;
use crate::point::{distance, Point};
use crate::KdNode::{Empty, Node};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

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

impl<T: KDT + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + std::fmt::Debug> KdNode<T> {
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
        assert!(radius >= 0.0, "Radius must be positive");

        let mut best_queue: Vec<(&KdNode<T>, f64)> = Vec::new();
        let mut parent_queue: Vec<&KdNode<T>> = self.drill_down(origin);
        let deepest: &KdNode<T> = parent_queue.get(0).unwrap();

        deepest._nearest_neighbor(origin, radius, &mut best_queue, &mut parent_queue);

        best_queue.retain(|(_, dist)| *dist <= radius);
        return best_queue
            .iter()
            .map(|(node, _)| match node {
                Node { point, .. } => point.clone(),
                _ => panic!("Empty node in best queue"),
            })
            .collect();
    }

    fn _nearest_neighbor<'a>(
        &'a self,
        origin: Point<T>,
        radius: f64,
        best_queue: &mut Vec<(&'a KdNode<T>, f64)>,
        parent_queue: &mut Vec<&'a KdNode<T>>,
    ) -> Vec<(&KdNode<T>, f64)> {
        let parent = parent_queue.pop();
        if parent.is_none() {
            return best_queue.clone();
        }

        match parent.unwrap() {
            Empty => {}
            Node {
                left,
                right,
                point,
                dim,
            } => {
                // Add node point if in range.
                let dis = distance(&origin, point);
                if dis <= radius {
                    KdNode::insert_sorted(best_queue, (parent.unwrap(), distance(&origin, point)));
                }

                for side_node in [left, right] {
                    if !best_queue
                        .iter()
                        .find(|(a, _)| *a == side_node.as_ref())
                        .is_some()
                    {
                        // Check if the radius actually overlaps the node children.
                        match side_node.as_ref() {
                            Empty => {}
                            Node { point, .. } => {
                                match dim {
                                    Dim::X
                                    if (origin.x.into() + radius > point.x.into()
                                        && origin.x.into() < point.x.into())
                                        || (origin.x.into() - radius < point.x.into()
                                        && origin.x.into() > point.x.into()) => {}
                                    Dim::Y
                                    if (origin.y.into() + radius > point.y.into()
                                        && origin.y.into() < point.y.into())
                                        || (origin.y.into() - radius < point.y.into()
                                        && origin.y.into() > point.y.into()) => {}
                                    _ => {
                                        continue;
                                    }
                                }
                            },
                        }

                        parent_queue.push(side_node.as_ref());
                        let temp =
                            side_node._nearest_neighbor(origin, radius, best_queue, parent_queue);
                        for (node, dist) in temp {
                            KdNode::insert_sorted(best_queue, (node, dist));
                        }
                    }
                }

                parent
                    .unwrap()
                    ._nearest_neighbor(origin, radius, best_queue, parent_queue);
            }
        }

        best_queue.clone()
    }

    /// Drill down the tree to find appropriate node and return the parents.
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
        return parents;
    }

    /// Insert a point into a sorted list if it is not already in the list.
    fn insert_sorted<'a>(
        points: &mut Vec<(&'a KdNode<T>, f64)>,
        point: (&'a KdNode<T>, f64),
    ) -> () {
        let mut index: usize = 0;
        for (i, (node, dist)) in points.iter().enumerate() {
            if *dist < point.1 {
                index = i + 1;
            }
            if *node == point.0 && *node != &Empty {
                return;
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
        let median: Point<T> = points[median_index];
        let right: Vec<Point<T>> = points.drain(..median_index).collect();
        let left: Vec<Point<T>> = points.drain(1..).collect();
        (median, left, right)
    }
}

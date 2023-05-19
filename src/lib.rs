//! # KdTree
//!
//! A simple k-d tree implementation in Rust.
//!
//! Data structure for efficiently finding points in a k-dimensional space.
//!
//! This is an under development implementation of a KD Tree in Rust.
//! Below is a list of features that are currently implemented and features that are planned to be implemented.
//!
//! * [x] Build Tree
//! * [x] Find All Points Within A Radius
//! * [x] Find Nearest Neighbor
//! * [x] Insert New Point
//! * [x] Find **N** Nearest Neighbors
//! * [ ] Delete Point
//! * [ ] Re-Balance Tree
//! * [ ] Serialize Tree
//! * [ ] Publish Crate
//! * [ ] Add **K** dimensions **(Currently only 2D)**
//! * [x] Add Examples
//!
//! This was developed initially as a way to learn Rust and to implement a KD Tree for a boids simulation although the
//! simulation is in progress. I plan to continue to work on this project as I learn more about Rust and as I have time.
//!
//! ## Usage
//!
//! [`KdNode`](struct.KdNode.html) is the main data structure for the KD Tree. It is a generic struct that takes a type
//!
//! [`Point`](point/struct.Point.html) which is a struct that contains the x and y coordinates of a point in 2D space.
//!
//! The type of the x and y coordinates can be any type that can implement the [`KDT`](trait.KDT.html) trait.
//! This trait is implemented for all types that implement the following traits:
//! [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html),
//! [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html),
//! [`Into<f64>`](https://doc.rust-lang.org/std/convert/trait.Into.html),
//! [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html),
//! [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html),
//! [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html),
//! [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html).
//!
//!
//! ```rust
//! extern crate kd_tree_rs;
//!
//! use kd_tree_rs::KdNode;
//! use kd_tree_rs::KdNode::Empty;
//! use kd_tree_rs::point::Point;
//!
//! fn main() {
//!    let mut node: KdNode<i32> = KdNode::new();
//!
//!    node.insert(1, 1);
//!    node.insert(2, 2);
//!
//!    assert_eq!(node.nearest_neighbor(Point{x: 1, y: 1}, 1.0), vec![Point{x: 1, y: 1}]);
//! }
//! ```
//!
//! ## References
//!
//! * [KD Tree](https://en.wikipedia.org/wiki/K-d_tree)
//! * [KD Tree Visualization](https://www.cs.usfca.edu/~galles/visualization/KDTree.html)
//! * [KD Tree Nearest Neighbor](https://www.cs.cmu.edu/~ckingsf/bioinfo-lectures/kdtrees.pdf)
//! * [Proof for neighborhood computation in expected logarithmic time - Martin Skrodzki](https://arxiv.org/pdf/1903.04936.pdf)
//! * [Introduction to a KD Tree](https://yasenh.github.io/post/kd-tree/)


extern crate core;

pub mod dim;
pub mod point;
mod tests;

pub use crate::dim::Dim;
pub use crate::point::Point;
pub use crate::KdNode::{Empty, Node};
use crate::point::distance;
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

pub trait KDT: PartialEq + PartialOrd + Copy + Mul + Sub + Add + Into<f64> {}
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
pub enum KdNode<T: KDT> {
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
    pub fn new() -> Self {
        Empty
    }

    /// Insert a new item into the tree
    ///
    /// This should used sparingly as it can unbalance the tree
    /// and reduce performance. If there is a large change to the dataset
    /// it is better to create a new tree. This effect has not been tested
    /// though and could be totally fine in terms of performance for a large
    /// number of inserts. A good rule of thumb may be if the tree size is
    /// going to increase by more than 10% it may be better to create a new
    /// tree.
    pub fn insert(&mut self, x: T, y: T) -> &Self {
        self.insert_point(Point{ x, y })
    }

    /// Insert a new item into the tree
    ///
    /// This is the same as `insert` but takes a `Point` instead of `x` and `y`
    pub fn insert_point(&mut self, item: Point<T>) -> &Self {
        self._insert(item, 0)
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
    ///
    /// This will return a vector of points that are within the radius of the origin point.
    /// The radius is inclusive so if a point is exactly on the radius it will be included.
    ///
    pub fn nearest_neighbor<'a>(&self, origin: Point<T>, radius: f64) -> Vec<Point<T>> {
        assert!(radius >= 0.0, "Radius must be positive");

        let mut best_queue: Vec<(&KdNode<T>, f64)> = Vec::new();
        let mut parent_queue: Vec<&KdNode<T>> = self.drill_down(origin);
        let deepest: &KdNode<T> = parent_queue.get(0).unwrap();

        deepest._nearest_neighbor(origin, radius, &mut best_queue, &mut parent_queue, None);

        best_queue.retain(|(_, dist)| *dist <= radius);
        return best_queue
            .iter()
            .map(|(node, _)| match node {
                Node { point, .. } => point.clone(),
                _ => panic!("Empty node in best queue"),
            })
            .collect();
    }

    /// Insert a new item into the tree
    ///
    /// This is the same as `insert` but takes a `Point` instead of `x` and `y`
    pub fn nearest_neighbor_x_y<'a>(&self, x: T, y: T, radius: f64) -> Vec<Point<T>> {
        self.nearest_neighbor(Point { x, y }, radius)
    }

    /// Find the nearest neighbors to the origin point
    ///
    /// This will return a vector of points that are within the radius of the origin point.
    /// This is the same as `nearest_neighbor` but will only return the `max` number of points.
    pub fn n_nearest_neighbor<'a>(&self, origin: Point<T>, max: usize) -> Vec<Point<T>> {
        let mut best_queue: Vec<(&KdNode<T>, f64)> = Vec::new();
        let mut parent_queue: Vec<&KdNode<T>> = self.drill_down(origin);
        let deepest: &KdNode<T> = parent_queue.get(0).unwrap();

        // TODO Should use just an option instead of `f64::MAX`.
        deepest._nearest_neighbor(origin, f64::MAX, &mut best_queue, &mut parent_queue, Some(max));

        return best_queue
            .iter()
            .map(|(node, _)| match node {
                Node { point, .. } => point.clone(),
                _ => panic!("Empty node in best queue"),
            })
            .collect();
    }

    /// Find the nearest neighbors to the origin point
    ///
    /// This is a recursive function that will recursively work its way up the tree
    /// collecting all neighbours within the radius provided.
    fn _nearest_neighbor<'a>(
        &'a self,
        origin: Point<T>,
        radius: f64,
        best_queue: &mut Vec<(&'a KdNode<T>, f64)>,
        parent_queue: &mut Vec<&'a KdNode<T>>,
        max: Option<usize>,
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
                ..
            } => {
                if let Some(max) = max {
                    if best_queue.len() >= max {
                        return vec![];
                    }
                }

                // Add node point if in range.
                let dis = distance(&origin, point);
                if dis <= radius {
                    KdNode::insert_sorted(best_queue, (parent.unwrap(), distance(&origin, point)));
                }

                for side_node in [left, right] {
                    if !best_queue.iter()
                        .find(|(a, _)| *a == side_node.as_ref())
                        .is_some()
                    {
                        // Check if the radius actually overlaps the node children.
                        match side_node.as_ref() {
                            Node { point, dim, .. } => {
                                if !point.in_radius(&origin, dim, radius) {
                                    continue;
                                }
                            },
                            _ => {}
                        }

                        parent_queue.push(side_node.as_ref());
                        let temp =
                            side_node._nearest_neighbor(origin, radius, best_queue, parent_queue, max);
                        for (node, dist) in temp {
                            if dist <= radius {
                                if let Some(max) = max {
                                    if best_queue.len() >= max {
                                        return vec![];
                                    }
                                }
                                KdNode::insert_sorted(best_queue, (node, dist));
                            }
                        }
                    }
                }

                parent.unwrap()._nearest_neighbor(origin, radius, best_queue, parent_queue, max);
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

    pub fn build(points: Vec<Point<T>>) -> Self {
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

mod tests;

use std::ops::Div;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use crate::Dim::{X, Y};

struct KdNode<T> {
    point: Point<T>,
    left: Box<KdNode<T>>,
    right: Box<KdNode<T>>
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct Point<T>{
    x: T,
    y: T
}

#[derive(Clone, Copy)]
enum Dim {
    X,
    Y
}


fn split_point_set<T: Clone>(q: usize, dim: Dim, point_set: &Vec<Point<T>>) -> (Vec<Point<T>>, Vec<Point<T>>) {
    if point_set.is_empty() {
        panic!("Empty point set");
    }

    let mut out: (Vec<Point<T>>, Vec<Point<T>>) = (vec![], vec![]);
    out.0 = point_set[0..q].to_owned();
    out.1 = point_set[q..point_set.len()].to_owned();

    return out;
}


fn median_of_points<T: Ord>(dim: Dim, point_set: &mut Vec<Point<T>>) -> (&Point<T>, usize)  {
    if point_set.is_empty() {
        panic!("Empty point set");
    }
    point_set.sort_by(|a, b|
        match dim {
            X => a.x.cmp(&b.x),
            Y => a.y.cmp(&b.y)
        });

    let q: usize = point_set.len() / 2;

    return (point_set.get(q).unwrap(), q);
}


fn build<T: Ord + Clone>(mut point_set: Vec<Point<T>>) -> KdNode<T> {
    let dim: Dim = Y;
    let q: (&Point<T>, usize) = median_of_points(dim, &mut point_set);
    let mut split: (Vec<Point<T>>, Vec<Point<T>>)  = split_point_set(q.1, dim, &point_set);
    let mut l_node: KdNode<T> = build(split.0);
    let mut r_node: KdNode<T> = build(split.1);

    KdNode {
        point: q.0.clone(),
        left: Box::new(l_node),
        right: Box::new(r_node)
    }
}


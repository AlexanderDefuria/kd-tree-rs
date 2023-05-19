use crate::dim::Dim;

use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point<T: PartialEq> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq + PartialOrd + Into<f64> + Copy> Point<T> {
    pub(crate) fn gt(&self, rs: &Point<T>, dim: &Dim) -> bool {
        match dim {
            Dim::X => self.x > rs.x,
            Dim::Y => self.y > rs.y,
        }
    }

    pub(crate) fn in_radius(&self, rs: &Point<T>, dim: &Dim, radius: f64) -> bool {
        let origin: &T = match dim {
            Dim::X => &self.x,
            Dim::Y => &self.y,
        };

        let point: &T = match dim {
            Dim::X => &rs.x,
            Dim::Y => &rs.y,
        };

        let origin: f64 = (*origin).into();
        let point: f64 = (*point).into();

        (origin + radius > point && origin < point) || (origin - radius < point && origin > point)
    }

    pub(crate) fn cmp(&self, rs: &Point<T>, dim: &Dim) -> Ordering {
        let ls_value: &T = self.get_dim_value(dim);
        let rs_value: &T = rs.get_dim_value(dim);
        ls_value.partial_cmp(rs_value).unwrap()
    }

    pub(crate) fn get_dim_value(&self, dim: &Dim) -> &T {
        match dim {
            Dim::X => &self.x,
            Dim::Y => &self.y,
        }
    }
}

pub fn distance<T>(ls: &Point<T>, rs: &Point<T>) -> f64
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Copy
        + PartialEq
        + Into<f64>
        + Mul<Output = T>,
{
    let x: T = ls.x - rs.x;
    let y: T = ls.y - rs.y;
    let diff: T = x * x + y * y;
    diff.into().sqrt()
}

#[test]
fn test_distance() {
    let p1 = Point { x: 1., y: 1. };
    let p2 = Point { x: 2., y: 2. };
    let p3 = Point { x: 1., y: 2. };
    let p4 = Point { x: 2., y: 1. };

    assert_eq!(distance(&p1, &p2), 2f64.sqrt());
    assert_eq!(distance(&p1, &p3), 1f64);
    assert_eq!(distance(&p1, &p4), 1f64);
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

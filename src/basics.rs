use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }

    pub fn from_pair((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn blow_rectangle(self, size: Self) -> Bounds {
        Bounds::pull(self, size)
    }

    pub fn mul(self, c: i32) -> Self {
        Self::new(self.x * c, self.y * c)
    }

    pub fn div(self, c: i32) -> Self {
        Self::new(self.x / c, self.y / c)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point::from_pair(value)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn from_intervals(x: Interval, y: Interval) -> Self {
        Self {
            min: Point::new(x.min, y.min),
            max: Point::new(x.max, y.max),
        }
    }

    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self::from_points(Point::new(x1, y1), Point::new(x2, y2))
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self { min: p1, max: p2 }
    }

    pub fn from_size(size: Point) -> Self {
        Self::from_points(Point::zero(), size)
    }

    pub fn zero() -> Self {
        Self::from_points(Point::zero(), Point::zero())
    }

    pub fn fixed(self) -> Self {
        let (x1, x2) = min_max(self.min.x, self.max.x);
        let (y1, y2) = min_max(self.min.y, self.max.y);
        Self::new(x1, y1, x2, y2)
    }

    pub fn x(self) -> Interval {
        Interval::from_bounds(self.min.x, self.max.x)
    }

    pub fn y(self) -> Interval {
        Interval::from_bounds(self.min.y, self.max.y)
    }

    pub fn width(self) -> i32 {
        self.x().length()
    }

    pub fn height(self) -> i32 {
        self.y().length()
    }

    pub fn size(self) -> Point {
        Point::new(self.width(), self.height())
    }

    pub fn contains(self, v: Point) -> bool {
        self.min.x <= v.x && v.x < self.max.x && self.min.y <= v.y && v.y < self.max.y
    }

    pub fn center(self) -> Point {
        (self.min + self.max).div(2)
    }

    pub fn xywh(self) -> (f32, f32, f32, f32) {
        (
            self.min.x as f32,
            self.min.y as f32,
            self.width() as f32,
            self.height() as f32,
        )
    }

    pub fn add(self, v: Point) -> Self {
        Self {
            min: self.min + v,
            max: self.max + v,
        }
    }

    pub fn shrink(self, (left, top, right, bottom): (i32, i32, i32, i32)) -> Self {
        Self::from_points(
            self.min + Point::new(left, top),
            self.max - Point::new(right, bottom),
        )
    }

    pub fn pull(p: Point, v: Point) -> Self {
        Self::from_points(p, p + v)
    }
}

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: i32,
    pub max: i32,
}

impl Interval {
    pub fn from_bounds(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn fixed(self) -> Self {
        let (min, max) = min_max(self.min, self.max);
        Self::from_bounds(min, max)
    }

    pub fn from_size(origin: i32, size: i32) -> Self {
        Self::from_bounds(origin, origin + size)
    }

    pub fn zero() -> Self {
        Self::from_bounds(0, 0)
    }

    pub fn length(self) -> i32 {
        self.max - self.min
    }

    pub fn add(self, v: i32) -> Self {
        Self::from_bounds(self.min + v, self.max + v)
    }

    pub fn contains(self, c: i32) -> bool {
        self.min <= c && c < self.max
    }

    pub fn center(self) -> i32 {
        (self.min + self.max) / 2
    }

    pub fn shrink(self, v: i32) -> Self {
        Self::from_bounds(self.min + v, self.max - v)
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub struct NDInterval<const N: usize> {
    min: NDPoint<N>,
    max: NDPoint<N>,
}

pub struct NDPoint<const N: usize> {
    coordinates: [i32; N],
}

impl<const N: usize> NDPoint<N> {
    fn new(coordinates: [i32; N]) -> Self {
        Self { coordinates }
    }

    fn zero() -> Self {
        Self {
            coordinates: [0; N],
        }
    }

    fn mul(self, c: i32) -> Self {
        Self {
            coordinates: self.coordinates.map(|x| x * c),
        }
    }

    fn div(self, c: i32) -> Self {
        Self {
            coordinates: self.coordinates.map(|x| x / c),
        }
    }
}

impl<const N: usize> Add for NDPoint<N> {
    type Output = NDPoint<N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut coordinates = [0; N];
        for i in 0..N {
            coordinates[i] = self.coordinates[i] + rhs.coordinates[i];
        }
        NDPoint { coordinates }
    }
}

impl<const N: usize> Neg for NDPoint<N> {
    type Output = NDPoint<N>;
    fn neg(self) -> Self::Output {
        NDPoint {
            coordinates: self.coordinates.map(|a| -a),
        }
    }
}

impl<const N: usize> Sub for NDPoint<N> {
    type Output = NDPoint<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

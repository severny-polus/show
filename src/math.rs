use std::ops::{Add, Div, Mul, Neg, Range, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: Mul<Output = T> + Copy,
{
    pub fn mul(self, c: T) -> Self {
        Self::new(self.x * c, self.y * c)
    }
}

impl<T> Point<T>
where
    T: Div<Output = T> + Copy,
{
    pub fn div(self, c: T) -> Self {
        Self::new(self.x / c, self.y / c)
    }
}

impl<T> Neg for Point<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Point<i32> {
    pub fn zero() -> Self {
        Point::new(0, 0)
    }

    pub fn from_pair((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }

    pub fn to_floats(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }

    pub fn to_f32(self) -> Point<f32> {
        Point::new(self.x as f32, self.y as f32)
    }
}

impl Point<f32> {
    pub fn zero() -> Self {
        Self::new(0., 0.)
    }

    pub fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Default for Point<i32> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
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

    pub fn range(self) -> Range<i32> {
        self.min..self.max
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min: Point<i32>,
    pub max: Point<i32>,
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

    pub fn from_points(p1: Point<i32>, p2: Point<i32>) -> Self {
        Self { min: p1, max: p2 }
    }

    pub fn from_size(size: Point<i32>) -> Self {
        Self::from_points(Point::<i32>::zero(), size)
    }

    pub fn zero() -> Self {
        Self::from_points(Point::<i32>::zero(), Point::<i32>::zero())
    }

    pub fn fix(self) -> Self {
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

    pub fn size(self) -> Point<i32> {
        Point::new(self.width(), self.height())
    }

    pub fn contains(self, v: Point<i32>) -> bool {
        self.min.x <= v.x && v.x < self.max.x && self.min.y <= v.y && v.y < self.max.y
    }

    pub fn center(self) -> Point<i32> {
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

    pub fn pull(p: Point, v: Point) -> Self {
        Self::from_points(p, p + v)
    }

    pub fn min_max(self) -> Point {
        Point::new(self.min.x, self.max.y)
    }

    pub fn max_min(self) -> Point {
        Point::new(self.max.x, self.min.y)
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self::zero()
    }
}

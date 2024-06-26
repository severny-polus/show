use crate::math::{Bounds, Point};

#[derive(Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone)]
pub enum Length {
    Pixels(u32),
    Fill,
    FillPortion(f64),
}

impl Default for Length {
    fn default() -> Self {
        Self::Fill
    }
}

impl Length {
    pub fn pixels(self, parent_length: u32, total_portions: f64) -> u32 {
        match self {
            Length::Pixels(pixels) => pixels,
            Length::Fill => Length::FillPortion(1.).pixels(parent_length, total_portions),
            Length::FillPortion(portion) => (parent_length as f64 * portion / total_portions) as u32,
        }
    }

    pub fn count_portions(lengths: impl Iterator<Item = Length>) -> f64 {
        lengths
            .map(|length| match length {
                Length::Fill => 1.,
                Length::FillPortion(portion) => portion,
                _ => 0.,
            })
            .sum()
    }
}

#[derive(Copy, Clone)]
pub struct Indents {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Indents {
    pub fn new(left: u32, top: u32, right: u32, bottom: u32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn equal(value: u32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub fn axis(horizontal: u32, vertical: u32) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }

    pub fn zero() -> Self {
        Self::equal(0)
    }

    pub fn shrink(&self, bounds: Bounds) -> Bounds {
        Bounds::from_points(
            bounds.min + Point::new(self.left as i32, self.top as i32),
            bounds.max - Point::new(self.right as i32, self.bottom as i32),
        )
    }
}

impl Default for Indents {
    fn default() -> Self {
        Self::zero()
    }
}

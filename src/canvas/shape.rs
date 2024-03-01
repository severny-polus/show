use crate::basics::Bounds;

use super::{color::Color, Canvas};

#[derive(Clone)]
pub enum Shape {
    Rectangle { color: Color },
    Border { color: Color, line_width: usize },
    Text { string: String, color: Color },
}

impl Shape {
    pub fn draw(&self, canvas: &mut Canvas, bounds: Bounds) {
        match self {
            Shape::Rectangle { color } => canvas.fill_rectangle(color, bounds),
            Shape::Border { color, line_width } => {
                canvas.stroke_rectangle(*line_width as i32, color, bounds)
            }
            Shape::Text { string, color } => {}
        }
    }
}

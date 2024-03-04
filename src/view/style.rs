use crate::{Bounds, Canvas, Color, Indents, Length};

pub struct Border {
    width: u32,
    color: Color,
}

impl Border {
    fn new(width: u32, color: Color) -> Self {
        Self { width, color }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new(0, Color::transparent())
    }
}

pub struct Style {
    pub width: Length,
    pub height: Length,
    pub margin: Indents,
    pub padding: Indents,
    pub backdround: Color,
    pub border: Border,
}

impl Style {
    pub fn draw(&self, canvas: &mut Canvas, bounds: Bounds) {
        canvas.fill_rectangle(
            self.backdround,
            Indents::equal(self.border.width).shrink(bounds),
        );
        canvas.stroke_rectangle(self.border.width, self.border.color, bounds);
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            margin: Indents::zero(),
            padding: Indents::zero(),
            backdround: Color::transparent(),
            border: Border::default(),
        }
    }
}

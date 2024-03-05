use crate::{Bounds, Canvas, Color, Indents, Length};

pub struct Border {
    width: u32,
    color: Color,
}

impl Border {
    pub fn new(width: u32, color: Color) -> Self {
        Self { width, color }
    }
}

impl Border {
    pub fn draw(&self, canvas: &mut Canvas, bounds: Bounds) {
        let width = self.width as i32;
        canvas.draw_rectangle(
            Bounds::new(
                bounds.min.x,
                bounds.min.y,
                bounds.min.x + width,
                bounds.max.y - width,
            ),
            self.color,
        );
        canvas.draw_rectangle(
            Bounds::new(
                bounds.min.x,
                bounds.max.y - width,
                bounds.max.x - width,
                bounds.max.y,
            ),
            self.color,
        );
        canvas.draw_rectangle(
            Bounds::new(
                bounds.max.x - width,
                bounds.min.y + width,
                bounds.max.x,
                bounds.max.y,
            ),
            self.color,
        );
        canvas.draw_rectangle(
            Bounds::new(
                bounds.min.x + width,
                bounds.min.y,
                bounds.max.x,
                bounds.min.y + width,
            ),
            self.color,
        );
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
        canvas.draw_rectangle(
            Indents::equal(self.border.width).shrink(bounds),
            self.backdround,
        );
        self.border.draw(canvas, bounds);
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

use crate::basics::{Interval, Point, Rectangle};
use crate::canvas::color::Color;
use crate::canvas::{Canvas, Fill, Shape};

pub struct View<M> {
    pub width: Length,
    pub height: Length,
    pub shapes: Vec<Shape>,
    pub children: Vec<View<M>>,
    pub orientation: Orientation,
    pub margin: Indents,
    pub padding: Indents,
    pub on_click: Option<M>,
}

#[derive(Copy, Clone)]
pub enum Length {
    Pixels(u32),
    Fill,
    FillPortion(u32),
}

pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone)]
pub struct Indents {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl<M> View<M> {
    pub fn draw(&self, canvas: &mut Canvas, bounds: Rectangle) {
        let bounds = bounds.shrink(self.padding.into());
        for shape in &self.shapes {
            shape.draw(canvas, bounds)
        }
        self.draw_children(canvas, bounds.min, bounds.size())
    }

    fn draw_children(&self, canvas: &mut Canvas, top_left: Point, size: Point) {
        match self.orientation {
            Orientation::Vertical => {
                let child_portions_y = self.count_child_portions_y();
                self.children.iter().fold(top_left.y, |child_top, child| {
                    child
                        .draw_as_child(
                            canvas,
                            Point::new(top_left.x, child_top),
                            size,
                            1,
                            child_portions_y,
                        )
                        .y
                });
            }
            Orientation::Horizontal => {
                let child_portions_x = self.count_child_portions_x();
                self.children.iter().fold(top_left.x, |child_left, child| {
                    child
                        .draw_as_child(
                            canvas,
                            Point::new(child_left, top_left.y),
                            size,
                            child_portions_x,
                            1,
                        )
                        .x
                });
            }
        }
    }

    fn draw_as_child(
        &self,
        canvas: &mut Canvas,
        origin: Point,
        max_size: Point,
        portions_x: u32,
        portions_y: u32,
    ) -> Point {
        let size = Point::new(
            self.width.pixels(max_size.x as u32, portions_x) as i32,
            self.height.pixels(max_size.y as u32, portions_y) as i32,
        );
        let bounds = origin.blow_rectangle(size);
        self.draw(canvas, bounds.shrink(self.margin.into()));
        bounds.max
    }

    fn count_child_portions_x(&self) -> u32 {
        Length::count_portions(self.children.iter().map(|child| child.width))
    }
    fn count_child_portions_y(&self) -> u32 {
        Length::count_portions(self.children.iter().map(|child| child.height))
    }
}

impl<M> Default for View<M> {
    fn default() -> Self {
        Self {
            width: Length::Pixels(0),
            height: Length::Pixels(0),
            shapes: vec![],
            children: vec![],
            orientation: Orientation::Vertical,
            margin: Default::default(),
            padding: Default::default(),
            on_click: None,
        }
    }
}

impl Length {
    fn pixels(self, max_length: u32, total_portions: u32) -> u32 {
        match self {
            Length::Pixels(pixels) => pixels,
            Length::Fill => Length::FillPortion(1).pixels(max_length, total_portions),
            Length::FillPortion(portion) => max_length * portion / total_portions,
        }
    }

    fn count_portions(lengths: impl Iterator<Item = Length>) -> u32 {
        lengths
            .map(|length| match length {
                Length::Fill => 1,
                Length::FillPortion(portion) => portion,
                _ => 0,
            })
            .sum()
    }
}

impl Default for Indents {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl Indents {
    fn equal(value: u32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    fn axis(vetrical: u32, horizontal: u32) -> Self {
        Self {
            left: horizontal,
            top: vetrical,
            right: horizontal,
            bottom: vetrical,
        }
    }

    fn all(left: u32, top: u32, right: u32, bottom: u32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Into<(i32, i32, i32, i32)> for Indents {
    fn into(self) -> (i32, i32, i32, i32) {
        (
            self.left as i32,
            self.top as i32,
            self.right as i32,
            self.bottom as i32,
        )
    }
}

pub fn my_view<M>(message: M) -> View<M> {
    View {
        width: Length::Fill,
        height: Length::Fill,
        shapes: vec![
            Shape::Fill {
                color: Color::hex("#4488cc"),
            },
            Shape::Stroke {
                color: Color::hex("#ffffff"),
                line_width: 1.0,
            },
            Shape::Text {
                string: "Hello".to_string(),
                color: Color::white(),
            },
        ],
        margin: Indents::equal(8),
        ..View::default()
    }
}

pub fn row<M>() -> View<M> {
    View {
        width: Length::Fill,
        height: Length::Fill,
        shapes: vec![],
        children: vec![],
        orientation: Orientation::Horizontal,
        margin: Default::default(),
        padding: Default::default(),
        on_click: None,
    }
}

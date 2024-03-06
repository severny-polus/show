pub mod style;
pub mod util;
pub mod views;

use crate::{Bounds, Canvas, Event, Point};

pub use self::util::{Indents, Length, Orientation};
pub use style::Style;

type MouseButton = glfw::MouseButton;

pub trait View<M = ()> {
    fn width(&self) -> Length;
    fn height(&self) -> Length;
    fn set_bounds(&mut self, bounds: Bounds);
    fn process(&mut self, event: Event) -> Option<M>;
    fn draw(&self, canvas: &mut Canvas);
}

impl<M> dyn View<M> {
    fn adjust_bounds(
        &mut self,
        min: Point,
        total_size: Point,
        portions_x: f64,
        portions_y: f64,
    ) -> Point {
        let size = Point::new(
            self.width().pixels(total_size.x as u32, portions_x) as i32,
            self.height().pixels(total_size.y as u32, portions_y) as i32,
        );
        let bounds = min.pull(size);
        self.set_bounds(bounds);
        bounds.max
    }
}

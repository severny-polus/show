use std::rc::Rc;

use crate::{graphics::Context, Bounds, Event, Length, Orientation, Point, Style};

pub trait Drawer<M = ()> {
    fn width(&self) -> Length;
    fn height(&self) -> Length;
    fn set_bounds(&mut self, context: &Context, bounds: Bounds);
    fn process(&mut self, event: Event) -> Option<M>;
    fn draw(&mut self, context: &Context);

    fn adjust_bounds(
        &mut self,
        context: &Context,
        min: Point,
        parent_size: Point,
        portions_x: f64,
        portions_y: f64,
    ) -> Point {
        let size = Point::new(
            self.width().pixels(parent_size.x as u32, portions_x) as i32,
            self.height().pixels(parent_size.y as u32, portions_y) as i32,
        );
        let bounds = Bounds::pull(min, size);
        self.set_bounds(context, bounds);
        bounds.max
    }
}

pub struct CommonDrawer<M> {
    pub bounds: Bounds,
    pub style: Style,
    pub orientation: Orientation,
    pub children: Vec<Box<dyn Drawer<M>>>,
}

impl<M> CommonDrawer<M> {
    fn count_child_portions_x(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.width()))
    }

    fn count_child_portions_y(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.height()))
    }
}

impl<M> Drawer<M> for CommonDrawer<M> {
    fn width(&self) -> Length {
        self.style.width
    }

    fn height(&self) -> Length {
        self.style.height
    }

    fn set_bounds(&mut self, context: &Context, bounds: Bounds) {
        self.bounds = self.style.margin.shrink(bounds);
        let bounds = self.style.padding.shrink(self.bounds);
        let parent_min = bounds.min;
        let parent_size = bounds.size();
        match self.orientation {
            Orientation::Vertical => {
                let portions_y = self.count_child_portions_y();
                self.children.iter_mut().fold(parent_min.y, |min_y, child| {
                    child
                        .adjust_bounds(
                            context,
                            Point::new(parent_min.x, min_y),
                            parent_size,
                            1.,
                            portions_y,
                        )
                        .y
                });
            }
            Orientation::Horizontal => {
                let portions_x = self.count_child_portions_x();
                self.children.iter_mut().fold(parent_min.x, |min_x, child| {
                    child
                        .adjust_bounds(
                            context,
                            Point::new(min_x, parent_min.y),
                            parent_size,
                            portions_x,
                            1.,
                        )
                        .x
                });
            }
        }
    }

    fn process(&mut self, _event: Event) -> Option<M> {
        None
    }

    fn draw(&mut self, context: &Context) {
        self.style.draw_rectangle(context, self.bounds);
        for child in &mut self.children {
            child.draw(context);
        }
    }
}

use crate::{
    container::Container, Bounds, Context, Event, Length, Orientation, Point, Style, View,
};

pub trait Drawer<M = ()> {
    fn width(&self) -> Length;
    fn height(&self) -> Length;
    fn set_bounds(&mut self, bounds: Bounds);
    fn process(&mut self, event: Event) -> Option<M>;
    fn draw(&mut self, context: &mut Context);

    fn adjust_bounds(
        &mut self,
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
        self.set_bounds(bounds);
        bounds.max
    }
}

pub struct ContainerDrawer<M> {
    pub bounds: Bounds,
    pub style: Style,
    pub orientation: Orientation,
    pub children: Vec<Box<dyn Drawer<M>>>,
}

impl<M> ContainerDrawer<M> {
    fn count_child_portions_x(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.width()))
    }

    fn count_child_portions_y(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.height()))
    }
}

impl<M> Drawer<M> for ContainerDrawer<M> {
    fn width(&self) -> Length {
        self.style.width
    }

    fn height(&self) -> Length {
        self.style.height
    }

    fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = self.style.margin.shrink(bounds);
        let bounds = self.style.padding.shrink(self.bounds);
        let total_min = bounds.min;
        let total_size = bounds.size();
        match self.orientation {
            Orientation::Vertical => {
                let portions_y = self.count_child_portions_y();
                self.children.iter_mut().fold(total_min.y, |min_y, child| {
                    child
                        .adjust_bounds(Point::new(total_min.x, min_y), total_size, 1., portions_y)
                        .y
                });
            }
            Orientation::Horizontal => {
                let portions_x = self.count_child_portions_x();
                self.children.iter_mut().fold(total_min.x, |min_x, child| {
                    child
                        .adjust_bounds(Point::new(min_x, total_min.y), total_size, portions_x, 1.)
                        .x
                });
            }
        }
    }

    fn process(&mut self, _event: Event) -> Option<M> {
        None
    }

    fn draw(&mut self, context: &mut Context) {
        self.style.draw_rectangle(context, self.bounds);
        for child in &mut self.children {
            child.draw(context);
        }
    }

    fn adjust_bounds(
        &mut self,
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
        self.set_bounds(bounds);
        bounds.max
    }
}

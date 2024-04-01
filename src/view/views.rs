use std::marker::PhantomData;

use crate::{style::Style, Bounds, Canvas, Event, Length, Orientation, Point, View};

pub struct Container<M> {
    style: Style,
    bounds: Bounds,
    orientation: Orientation,
    children: Vec<Box<dyn View<M>>>,
}

impl<M> Container<M> {
    pub fn new(
        orientation: Orientation,
        style: Style,
        children: Vec<Box<dyn View<M>>>,
    ) -> Box<Self> {
        Box::new(Self {
            style,
            children,
            orientation,
            bounds: Bounds::zero(),
        })
    }

    fn count_child_portions_x(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.width()))
    }

    fn count_child_portions_y(&self) -> f64 {
        Length::count_portions(self.children.iter().map(|child| child.height()))
    }
}

impl<M> View<M> for Container<M> {
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

    fn draw(&mut self, canvas: &mut Canvas) {
        self.style.draw(canvas, self.bounds);
        for child in &mut self.children {
            child.draw(canvas);
        }
    }
}

pub struct Row<M> {
    _phantom: PhantomData<M>,
}

impl<M> Row<M> {
    pub fn new(style: Style, children: Vec<Box<dyn View<M>>>) -> Box<Container<M>> {
        Container::new(Orientation::Horizontal, style, children)
    }
}

pub struct Column<M> {
    _phantom: PhantomData<M>,
}

impl<M> Column<M> {
    pub fn new(style: Style, children: Vec<Box<dyn View<M>>>) -> Box<Container<M>> {
        Container::new(Orientation::Vertical, style, children)
    }
}

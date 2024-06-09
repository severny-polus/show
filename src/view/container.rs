use std::marker::PhantomData;

use crate::{
    Context, ContainerDrawer, Drawer, Orientation, Style, View,
};

pub struct Container<M> {
    style: Style,
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
        })
    }
}

impl<M: 'static> View<M> for Container<M> {
    fn new_drawer(&self, context: &mut Context) -> Box<dyn Drawer<M>> {
        Box::new(ContainerDrawer {
            bounds: Default::default(),
            style: self.style,
            orientation: self.orientation,
            children: self
                .children
                .iter()
                .map(|child| child.new_drawer(context))
                .collect(),
        })
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

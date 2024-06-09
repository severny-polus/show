pub mod container;
pub mod style;
pub mod util;

use crate::{Context, Drawer};

pub use self::util::{Indents, Length, Orientation};
pub use style::Style;

type MouseButton = glfw::MouseButton;

pub trait View<M = ()> {
    fn new_drawer(&self, context: &mut Context) -> Box<dyn Drawer<M>>;
}

pub trait ViewFn<M>: Fn(&mut Context) -> Box<dyn Drawer<M>> {}

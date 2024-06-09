
pub mod graphics;
pub mod command;
pub mod event;
pub mod math;
pub mod model;
pub mod program;
pub mod view;
pub mod drawer;

pub use graphics::{Context, Color};
pub use command::*;
pub use event::*;
pub use math::*;
pub use model::*;
pub use program::*;
pub use view::*;
pub use drawer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}

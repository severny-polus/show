#![feature(const_fn_floating_point_arithmetic)]

pub mod canvas;
pub mod command;
pub mod event;
pub mod math;
pub mod model;
pub mod program;
pub mod view;

pub use canvas::*;
pub use command::*;
pub use event::*;
pub use math::*;
pub use model::*;
pub use program::*;
pub use view::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}

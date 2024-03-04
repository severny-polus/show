pub mod canvas;
pub mod command;
pub mod event;
pub mod math;
pub mod model;
pub mod view;
pub mod program;

pub use canvas::*;
pub use command::*;
pub use event::*;
pub use math::*;
pub use model::*;
pub use view::*;
pub use program::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}

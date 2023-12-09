use crate::command::Command;
use crate::view::View;

pub trait Model: Sized {
    type Message: Copy;
    fn init() -> (Self, Command<Self::Message>);
    fn update(&mut self, message: Self::Message);
    fn view(&self) -> Box<dyn View<Self::Message>>;
}

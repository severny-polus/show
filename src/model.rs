use crate::{Command, Subscriptions, View};

pub trait Model: Sized {
    type Flags;
    type Message: Copy;
    fn init(flags: Self::Flags) -> (Self, Command<Self::Message>);
    fn subscriptions() -> Subscriptions<Self::Message>;
    fn update(&mut self, message: Self::Message);
    fn view(&self) -> Box<dyn View<Self::Message>>;
}

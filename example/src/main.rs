use crate::Message::ButtonClick;
use show::basics::Bounds;
use show::canvas::color::Color;
use show::canvas::shape::Shape;
use show::command::Command;
use show::model::Model;
use show::view::{my_view, Orientation, View};
use show::window::Program;

fn main() {
    let mut program = Program::new().unwrap();
    program.run::<App>("hello").unwrap();
}

struct App {}

#[derive(Copy, Clone)]
enum Message {
    ButtonClick,
}

impl Model for App {
    type Message = Message;

    fn init() -> (Self, Command<Self::Message>) {
        (Self {}, Command::None)
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> View<Self::Message> {
        View {
            shapes: vec![Shape::Rectangle {
                color: "#4488cc".into(),
            }],
            children: vec![
                my_view(ButtonClick),
                my_view(ButtonClick),
                my_view(ButtonClick),
            ],
            orientation: Orientation::Horizontal,
            ..Default::default()
        }
    }
}

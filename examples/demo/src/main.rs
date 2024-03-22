use show::{
    style::Border, views::Row, Color, Command, Indents, Model, Program, Size, Style, Subscriptions, View
};

fn main() {
    let mut program = Program::new().unwrap();
    program.run::<App>(Size::default(), "hello", ()).unwrap();
}

struct App {}

#[derive(Copy, Clone)]
enum Message {
    ButtonClick,
}

impl Model for App {
    type Message = Message;
    type Flags = ();

    fn init(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::None)
    }

    fn subscriptions() -> Subscriptions<Self::Message> {
        Subscriptions::default()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::None
    }

    fn view(&self) -> Box<dyn View<Self::Message>> {
        Row::new(
            Style {
                margin: Indents::equal(8),
                backdround: Color::black(),
                border: Border::new(2, "#f4f4f4".into()),
                ..Default::default()
            },
            vec![],
        )
    }
}

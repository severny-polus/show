use show::{views::Row, Command, Model, Program, Style, Subscriptions, View};

fn main() {
    let mut program = Program::new().unwrap();
    program.run::<App>("hello", ()).unwrap();
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

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> Box<dyn View<Self::Message>> {
        Row::new(
            Style {
                backdround: "#f4f4f4".into(),
                ..Default::default()
            },
            vec![],
        )
    }
}

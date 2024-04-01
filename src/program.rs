use crate::{Bounds, Canvas, Command, Event, Model, Point, Subscriptions, View};
use glfw::{Context, InitError, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

pub struct Program {
    glfw: glfw::Glfw,
}

#[derive(Debug)]
pub enum Error {
    InitError(InitError),
    WindowCreationError,
    CanvasError(String),
}

impl From<InitError> for Error {
    fn from(value: InitError) -> Self {
        Error::InitError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::CanvasError(value)
    }
}

pub enum Size {
    Pixels(u32, u32),
    Max,
}

impl Default for Size {
    fn default() -> Self {
        Self::Pixels(600, 400)
    }
}

impl Program {
    pub fn new() -> Result<Self, Error> {
        let mut glfw = glfw::init_no_callbacks()?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Samples(Some(4))); // enables antialiasing
        Ok(Self { glfw })
    }

    pub fn run<T: Model>(&mut self, size: Size, title: &str, flags: T::Flags) -> Result<(), Error> {
        let (width, height) = match size {
            Size::Pixels(width, height) => (width, height),
            Size::Max => (600, 400),
        };
        let (mut window, events) = self
            .glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .ok_or(Error::WindowCreationError)?;

        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_framebuffer_size_polling(true);

        window.maximize();

        let mut size: Point = window.get_framebuffer_size().into();
        let dpi = window.get_size().0 as f32 / size.x as f32;

        let mut canvas = Canvas::new(size, dpi, |str| window.get_proc_address(str))?;

        let (mut model, cmd) = T::init(flags);
        let mut view = model.view();
        view.set_bounds(Bounds::from_size(size));
        view.draw(&mut canvas);

        while !window.should_close() {
            canvas.clear();
            view.draw(&mut canvas);
            // canvas.draw_image();

            window.swap_buffers();
            view.process(Event::Frame);
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::FramebufferSize(width, height) => {
                        size = Point::new(width, height);
                        canvas.set_size(size);
                        view.set_bounds(Bounds::from_size(size))
                    }
                    _ => (),
                }
                match view.process(Event::Window(event)) {
                    Some(message) => {
                        match model.update(message) {
                            Command::Update => {
                                view = model.view();
                            }
                            _ => {}
                        };
                    }
                    None => {}
                }
            }
        }
        Ok(())
    }
}

struct EmptyModel<V: View> {
    view: fn() -> V,
}

impl<V: View + 'static> Model for EmptyModel<V> {
    type Flags = fn() -> V;
    type Message = ();

    fn init(view: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { view }, Command::None)
    }

    fn subscriptions() -> Subscriptions<Self::Message> {
        Subscriptions::default()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::None
    }

    fn view(&self) -> Box<dyn View<Self::Message>> {
        Box::new((self.view)())
    }
}

impl Program {
    pub fn show<V: View + 'static>(
        &mut self,
        size: Size,
        title: &str,
        view: fn() -> V,
    ) -> Result<(), Error> {
        self.run::<EmptyModel<V>>(size, title, view)
    }
}

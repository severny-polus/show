use std::{borrow::Borrow, rc::Rc};

use crate::{Bounds, Canvas, Command, Length, Model, Point, Subscriptions, View};
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

impl Program {
    pub fn new() -> Result<Self, Error> {
        let mut glfw = glfw::init_no_callbacks()?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        Ok(Self { glfw: glfw })
    }

    pub fn run<T: Model>(&mut self, title: &str, flags: T::Flags) -> Result<(), Error> {
        let (mut window, events) = self
            .glfw
            .create_window(600, 400, title, WindowMode::Windowed)
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
            view.draw(&mut canvas);
            // canvas.clear(); // doesn't role
            canvas.flush();

            window.swap_buffers();
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
                view.process(event);
            }
        }
        Ok(())
    }
}

struct EmptyModel {
    view: fn() -> Box<dyn View<()>>,
}

impl Model for EmptyModel {
    type Flags = fn() -> Box<dyn View<()>>;
    type Message = ();

    fn init(view: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { view }, Command::None)
    }

    fn subscriptions() -> Subscriptions<Self::Message> {
        Subscriptions::default()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Box<dyn View<Self::Message>> {
        (self.view)()
    }
}

impl Program {
    pub fn show(&mut self, title: &str, view: fn() -> Box<dyn View<()>>) -> Result<(), Error> {
        self.run::<EmptyModel>(title, view)
    }
}

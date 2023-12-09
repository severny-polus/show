use crate::basics::{Point, Rectangle};
use crate::canvas::color::Color;
use crate::canvas::Canvas;
use crate::model::Model;
use crate::window::Error::CanvasError;
use femtovg::renderer::OpenGl;
use femtovg::{ErrorKind, Paint, Path, Renderer};
use glfw::{
    Context, Glfw, InitError, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode,
};
use std::marker::PhantomData;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;

pub struct Program {
    glfw: glfw::Glfw,
}

impl Program {
    pub fn new() -> Result<Self, Error> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        Ok(Self { glfw: glfw })
    }

    pub fn run<M: Model>(&mut self, title: &str) -> Result<(), Error> {
        let (mut window, events) = self
            .glfw
            .create_window(600, 400, title, WindowMode::Windowed)
            .ok_or(Error::WindowCreationError)?;

        let mut canvas = Canvas::new(|str| window.get_proc_address(str))?;

        window.set_mouse_button_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_framebuffer_size_polling(true);

        window.maximize();

        let mut size: Point = window.get_framebuffer_size().into();
        let dpi = window.get_size().0 as f32 / size.x as f32;

        canvas.set_size(size, dpi);

        let (mut model, cmd) = M::init();
        let view = &model.view();

        while !window.should_close() {
            view.draw(&mut canvas, Rectangle::from_size(size));
            canvas.flush();

            window.swap_buffers();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::FramebufferSize(width, height) => {
                        size = Point::new(width as i32, height as i32);
                        canvas.set_size(size, dpi);
                    }
                    _ => (),
                }
                println!("{:?}", event);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    InitError(InitError),
    WindowCreationError,
    CanvasError(ErrorKind),
}

impl From<InitError> for Error {
    fn from(value: InitError) -> Self {
        Error::InitError(value)
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error::CanvasError(value)
    }
}

// #[derive(Copy)]
// enum Setting {
//     Size(Vector),
//     Position(Vector),
//     Title(String),
//     Maximized(bool),
// }

// struct InitialSettings {
//     glfw: Glfw,
//     width: u32,
//     height: u32,
//     title: &str,
//     maximized: bool,
// }
//
// impl InitialSettings {
//     fn from_glfw(glfw: Glfw) -> Self {
//         Self {
//             glfw: glfw,
//             width: 600,
//             height: 400,
//             title: "",
//             maximized: true,
//         }
//     }
//
//     fn new(glfw: &Glfw, settings: &[Setting]) -> &Self {
//         settings.iter().fold(
//             &mut InitialSettings::from_glfw(glfw),
//             |settings, setting| settings.apply(*setting),
//         )
//     }
//
//     fn apply(&mut self, setting: Setting) {
//         match setting {
//             Setting::Size(size) => (self.width, self.height) = (u32(size.x), u32(size.y)),
//             Setting::Position(_) => {}
//             Setting::Title(title) => self.title = title.as_str(),
//             Setting::Maximized(maximized) => self.maximized = maximized,
//         }
//     }
// }

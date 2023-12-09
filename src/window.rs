use crate::basics::Point;
use crate::model::Model;
use crate::program::Error::CanvasError;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, ErrorKind, Paint, Path, Renderer};
use glfw::{
    Context, Glfw, InitError, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode,
};
use std::marker::PhantomData;
use std::ops::Deref;

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

    pub fn run<M: 'static + Model>(&mut self, title: &str) -> Result<(), Error> {
        let (mut window, events) = self
            .glfw
            .create_window(600, 400, title, WindowMode::Windowed)
            .ok_or(Error::WindowCreationError)?;

        window.maximize();
        window.set_mouse_button_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_framebuffer_size_polling(true);

        let opengl = unsafe { OpenGl::new_from_function(|str| window.get_proc_address(str)) }?;
        let mut canvas = Canvas::new(opengl)?;

        let mut size = window.get_framebuffer_size();
        let dpi = window.get_size().0 as f32 / size.0 as f32;

        canvas.set_size(size.0 as u32, size.1 as u32, dpi);

        while !window.should_close() {
            let mut rect = Path::new();
            rect.rect(100.0, 100.0, 200.0, 200.0);
            canvas.clear_rect(0, 0, width as u32, height as u32, Color::black());
            canvas.fill_path(&rect, &Paint::color(Color::rgb(0, 0, 255)));
            canvas.flush();

            window.swap_buffers();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::FramebufferSize(width, height) => {
                        size = (width, height);
                        canvas.set_size(width as u32, height as u32, dpi);
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

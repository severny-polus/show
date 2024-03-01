use crate::basics::{Bounds, Point};
use crate::canvas::Canvas;
use crate::model::Model;
use glfw::{Context, InitError, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

pub struct Program {
    glfw: glfw::Glfw,
}

impl Program {
    pub fn new() -> Result<Self, Error> {
        let mut glfw = glfw::init_no_callbacks()?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        Ok(Self { glfw: glfw })
    }

    pub fn run<M: Model>(&mut self, title: &str) -> Result<(), Error> {
        let (mut window, events) = self
            .glfw
            .create_window(600, 400, title, WindowMode::Windowed)
            .ok_or(Error::WindowCreationError)?;

        window.set_mouse_button_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_framebuffer_size_polling(true);

        window.maximize();

        let mut size: Point = window.get_framebuffer_size().into();
        let dpi = window.get_size().0 as f32 / size.x as f32;

        let mut canvas = Canvas::new(size, dpi, |str| window.get_proc_address(str))?;

        let (mut model, cmd) = M::init();
        let view = &model.view();

        while !window.should_close() {
            view.draw(&mut canvas, Bounds::from_size(size));
            // canvas.clear();
            canvas.flush();

            window.swap_buffers();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::FramebufferSize(width, height) => {
                        size = Point::new(width as i32, height as i32);
                        canvas.set_size(size);
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

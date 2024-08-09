mod definition;

pub use crate::window::definition::Definition;
use glfw::{Action, Context, Key};
use tracing::info;

#[derive(Debug)]
pub struct Window {
    pub definition: Definition
}

impl Window {
    pub(crate) fn run(&self) {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(
            self.definition.width,
            self.definition.height,
            self.definition.title.as_str(),
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        while !window.should_close() {
            window.swap_buffers();

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                info!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
}

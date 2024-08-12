mod definition;
mod project;
mod slice;

use std::ops::Deref;
pub use crate::application::definition::Definition;
use crate::renderer::renderer::Renderer;
use crate::{renderer, window};
use crate::window::event_handler::WindowEventHandler;
use crate::window::platform::winit::EventHandler;
pub use crate::window::Window;

#[derive(Debug)]
pub struct Application {
    application_definition: Definition
}

impl Application {
    pub fn new(application_definition: Definition) -> Self {
        Application { application_definition }
    }

    pub fn run(&self) {
        let window_definition = window::definition::Definition::new(
            self.application_definition.title.clone(),
            self.application_definition.width,
            self.application_definition.height
        );

        let renderer_definition = renderer::definition::Definition::new();

        let mut renderer = Renderer::new(renderer_definition);
        let mut window = Window::new(window_definition);
        let mut handler = WindowEventHandler { renderer };

        handler.renderer.attach_window_handle_provider(window.get_window_handle_provider());
        window.attach_handler(handler);
        window.run()
    }
}

mod definition;
mod project;
mod slice;

pub use crate::application::definition::Definition;
use crate::window;
pub use crate::window::Window;

#[derive(Debug)]
pub struct Application {
    application_definition: Definition
}

impl Application {
    pub fn new(application_definition: Definition) -> Self {
        Application {
            application_definition
        }
    }

    pub fn run(&self) {
        let window_definition = window::definition::Definition::new(self.application_definition.title.clone(), self.application_definition.width, self.application_definition.height);
        let window = Window::new(window_definition);
        window.run()
    }
}

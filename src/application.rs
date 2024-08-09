mod definition;

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
        let window = Window {
            definition: window::Definition::build(self.application_definition.title.clone(), self.application_definition.width, self.application_definition.height)
        };

        window.run()
    }
}

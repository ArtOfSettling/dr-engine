mod definition;
mod project;
mod slice;

pub use crate::application::definition::Definition;
use crate::renderer::renderer::Renderer;
use crate::{renderer, window};
use crate::module::default_module::DefaultModule;
use crate::module::Module;
use crate::window::platform::winit::EventHandler;
use crate::window::Window;

pub struct Application {
    application_definition: Definition,
    module: Box<dyn Module>
}

pub(crate) struct WindowEventHandler {
    pub(crate) renderer: Renderer
}

impl EventHandler for WindowEventHandler {
    fn resume(&self) {
        self.renderer.resume();
    }

    fn render(&self) {
        self.renderer.render();
    }

    fn destroy(&self) {
        self.renderer.destroy();
    }
}

impl Application {
    pub fn default() -> Self {
        Application {
            application_definition: Definition::default(),
            module: DefaultModule::default()
        }
    }

    pub fn new(application_definition: Definition, module: Box<dyn Module>) -> Self {
        Application { application_definition, module }
    }

    pub fn with_application_definition(application_definition: Definition) -> Self {
        Application {
            application_definition,
            module: DefaultModule::default()
        }
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

        let platform_renderer = self.module.get_platform_renderer(
            window.get_window_handle_provider()
        );
        handler.renderer.set_platform_renderer(platform_renderer);
        window.attach_handler(handler);
        window.run()
    }
}

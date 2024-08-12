use crate::renderer::renderer::Renderer;
use crate::window::platform::winit::EventHandler;

pub struct WindowEventHandler {
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

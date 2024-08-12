use winit::raw_window_handle::HasWindowHandle;
use crate::window::definition::Definition;
use crate::window::platform::winit::{EventHandler, WinitWindow};

pub(crate) mod definition;
pub(crate) mod platform;
pub(crate) mod event_handler;

pub struct Window {
    pub window_definition: Definition,
    winit_window: WinitWindow
}

impl Window {
    pub(crate) fn new(window_definition: Definition) -> Self {
        let winit_window = WinitWindow::new(
            window_definition.title.clone(),
            window_definition.width,
            window_definition.height
        );
        Self { window_definition, winit_window }
    }

    pub(crate) fn run(self) {
        self.winit_window.run();
    }
}

impl Window {
    pub(crate) fn attach_handler(&mut self, callback_handler: impl EventHandler + 'static) {
        self.winit_window.attach_handler(callback_handler);
    }
}

impl Window {
    pub(crate) fn get_window_handle_provider(&self) -> &dyn HasWindowHandle {
        self.winit_window.get_window_handle_provider()
    }
}

use winit::dpi::{LogicalSize, Size};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;
use crate::window::platform::state::{MyUserEvent, State};

pub(crate) struct WinitWindow {
    event_loop: EventLoop<MyUserEvent>,
    window: Window,
    callback_handler: Option<Box<dyn EventHandler + 'static>>
}

pub trait EventHandler {
    fn resume(&self);
    fn render(&self);
    fn destroy(&self);
}

impl WinitWindow {
    pub(crate) fn get_window_handle_provider(&self) -> &dyn HasWindowHandle {
        &self.window
    }
}

impl WinitWindow {
    pub(crate) fn new(
        title: String,
        width: u32,
        height: u32
    ) -> Self {
        let event_loop = EventLoop::<MyUserEvent>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window_attributes = Window::default_attributes()
            .with_title(title.clone())
            .with_inner_size(Size::Logical(LogicalSize::new(width as f64, height as f64)));

        #[allow(deprecated)]
        let window = event_loop.create_window(window_attributes).unwrap();

        Self { event_loop, window, callback_handler: None }
    }

    pub(crate) fn attach_handler(&mut self, callback_handler: impl EventHandler + 'static) {
        self.callback_handler = Some(Box::new(callback_handler))
    }

    pub(crate) fn run(self) {
        let callback_handler = self.callback_handler.unwrap();
        let mut state = State::new(self.window, callback_handler);
        self.event_loop.run_app(&mut state).unwrap();
    }
}

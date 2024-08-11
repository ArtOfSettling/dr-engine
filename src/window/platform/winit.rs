use winit::dpi::{LogicalSize, Size};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use crate::window::platform::state::State;

#[derive(Debug)]
pub(crate) struct MyUserEvent;

#[derive(Debug)]
pub(crate) struct WinitWindow {
    window: Window
}

impl WinitWindow {
    pub(crate) fn new(title: String, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::<MyUserEvent>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window_attributes = Window::default_attributes()
            .with_title(title.clone())
            .with_inner_size(Size::Logical(LogicalSize::new(width as f64, height as f64)));

        #[allow(deprecated)]
        let window = event_loop.create_window(window_attributes).unwrap();

        let mut state = State::new();
        event_loop.run_app(&mut state).unwrap();

        Self {
            window
        }
    }

    pub(crate) fn run(&self, window: &crate::window::Window) {
        /*
        let mut state = State::new(render_window);
        let _ = self.event_loop.run_app(&mut state).unwrap();
         */
    }
}

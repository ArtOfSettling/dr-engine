use tracing::trace;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use crate::window::platform::winit::MyUserEvent;
use crate::window::Window;

pub (crate) struct State {

}

impl State {
    pub fn new() -> Self {
        Self { }
    }
}

impl ApplicationHandler<MyUserEvent> for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        trace!("Resumed");
        //self.window.resume();
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, user_event: MyUserEvent) {
        trace!("{:?}", user_event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => (),
            _ => trace!("{:?}", event)
        }

        match event {
            WindowEvent::CloseRequested => {
                trace!("Close Requested");
                //self.window.destroy();
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                //self.window.render();
            },
            _ => ()
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        trace!("{:?}", event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {

    }
}

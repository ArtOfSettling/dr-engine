use std::fmt::Debug;
use tracing::trace;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::window::platform::winit::EventHandler;

#[derive(Debug)]
pub(in crate::window::platform) struct MyUserEvent;

pub(in crate::window::platform) struct State {
    callback_handler: Box<dyn EventHandler>,
    window: Window
}

impl State {
    pub(in crate::window::platform) fn new(
        window: Window,
        callback_handler: Box<dyn EventHandler>
    ) -> Self {
        Self { callback_handler, window }
    }
}

impl ApplicationHandler<MyUserEvent> for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        trace!("Resumed");
        self.callback_handler.resume();
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
                self.callback_handler.destroy();
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.callback_handler.render();
            },
            _ => ()
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        trace!("{:?}", event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.window.request_redraw()
    }
}

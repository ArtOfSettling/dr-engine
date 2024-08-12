pub mod default_module;

use winit::raw_window_handle::HasWindowHandle;
use crate::platform::PlatformRenderer;

pub trait Module {
    fn get_platform_renderer(&self, window_handle_provider: &dyn HasWindowHandle) -> Box<dyn PlatformRenderer>;
}

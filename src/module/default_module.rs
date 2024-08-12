use winit::raw_window_handle::HasWindowHandle;
use crate::module::Module;
use crate::platform::PlatformRenderer;
use crate::renderer::platform::vulkan_renderer::VulkanRenderer;

#[derive(Debug)]
pub struct DefaultModule {

}

impl DefaultModule {
    pub fn default() -> Box<dyn Module> {
        Box::new(Self { })
    }
}

impl Module for DefaultModule {
    fn get_platform_renderer(&self, window_handle_provider: &dyn HasWindowHandle) -> Box<dyn PlatformRenderer> {
        Box::new(VulkanRenderer::new(window_handle_provider))
    }
}

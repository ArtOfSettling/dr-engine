use tracing::{info, trace};
use winit::raw_window_handle::HasWindowHandle;
use crate::renderer::platform::vulkan_renderer::VulkanRenderer;
use crate::renderer::definition::Definition;

pub(crate) struct Renderer {
    pub renderer_definition: Definition,

    // for now, no hidden abstraction for this, it simply holds a vulkan renderer and does not expose
    // it outside of this crate. Potential extension point in the future for other renderers.
    platform_renderer: Option<VulkanRenderer>
}

impl Renderer {
    pub(crate) fn new(renderer_definition: Definition) -> Self {
        Self {
            renderer_definition,
            platform_renderer: None
        }
    }

    pub(crate) fn attach_window_handle_provider(
        &mut self,
        window_handle_provider: &dyn HasWindowHandle
    ) {
        self.platform_renderer = Some(VulkanRenderer::new(window_handle_provider))
    }

    pub(crate) fn resume(&self) {
        trace!("Renderer Resume");
    }

    pub(crate) fn render(&self) {
        trace!("Renderer Render");
    }

    pub(crate) fn destroy(&self) {
        trace!("Renderer Destroy");
    }
}

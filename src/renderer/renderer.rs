use crate::platform::PlatformRenderer;
use crate::renderer::definition::Definition;

pub(crate) struct Renderer {
    pub renderer_definition: Definition,

    // for now, no hidden abstraction for this, it simply holds a vulkan renderer and does not expose
    // it outside of this crate. Potential extension point in the future for other renderers.
    platform_renderer: Option<Box<dyn PlatformRenderer>>
}

impl Renderer {
    pub(crate) fn new(renderer_definition: Definition) -> Self {
        Self {
            renderer_definition,
            platform_renderer: None
        }
    }

    pub(crate) fn set_platform_renderer(
        &mut self,
        platform_renderer: Box<dyn PlatformRenderer>
    ) {
        self.platform_renderer = Some(platform_renderer)
    }

    pub(crate) fn resume(&self) {
        if let Some(platform_renderer) = &self.platform_renderer {
            platform_renderer.resume()
        }
    }

    pub(crate) fn render(&self) {
        if let Some(platform_renderer) = &self.platform_renderer {
            platform_renderer.render()
        }
    }

    pub(crate) fn destroy(&self) {
        if let Some(platform_renderer) = &self.platform_renderer {
            platform_renderer.destroy()
        }
    }
}

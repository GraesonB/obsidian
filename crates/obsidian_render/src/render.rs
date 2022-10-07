use anyhow::Result;
use raw_window_handle::HasRawWindowHandle;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanRenderBackend;

pub enum Backend {
    Vulkan
}

pub trait Render {
    fn render(&mut self, dims: &[u32; 2], ) -> Result<()>;
}

impl dyn Render {
    pub fn create_backend(
        backend: &Backend,
        window_handle: &impl HasRawWindowHandle,
        dims: &[u32; 2]
    ) -> Result<impl Render> {
        match backend {
            Backend::Vulkan => VulkanRenderBackend::new(window_handle, dims)
        }
    }
}
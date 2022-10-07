use crate::Render;
use anyhow::Result;
use raw_window_handle::HasRawWindowHandle;
use log::info;

pub(crate) struct VulkanRenderBackend;

impl Render for VulkanRenderBackend {
    fn render(&mut self, dims: &[u32; 2]) -> Result<()> {
        Ok(())
    }
}

impl VulkanRenderBackend {
    pub fn new(
        _window_handle: &impl HasRawWindowHandle,
        _dims: &[u32; 2],
    ) -> Result<Self> {
        info!("Created Vulkan render backend");
        Ok(Self{})
    }
}
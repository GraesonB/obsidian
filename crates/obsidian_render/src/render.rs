use anyhow::Result;

pub enum Backend {
    Vulkan
}

pub trait Render {
    fn render(&mut self, dims: &[u32; 2], ) -> Result<()>;
}
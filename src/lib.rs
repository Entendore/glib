pub mod backend;
pub mod binding;
pub mod command;
pub mod component;
pub mod device;
pub mod disposal;
pub mod error;
pub mod extensions;
pub mod frame;
pub mod gpu_driven;
pub mod memory;
pub mod pipeline;
pub mod queue;
pub mod render_graph;
pub mod resource;
pub mod shader;
pub mod swapchain;
pub mod sync;
pub mod types;

pub use error::{GlibError, Result};

pub struct Context {
    pub device: device::Device,
}

impl Context {
    pub fn builder() -> ContextBuilder { ContextBuilder::default() }
}

#[derive(Default)]
pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build(self) -> Result<Context> {
        Ok(Context { device: device::Device::new() })
    }
}

pub mod prelude {
    pub use crate::types::*;
    pub use crate::sync::*;
    pub use crate::memory::*;
    pub use crate::gpu_driven::*;
    pub use crate::disposal::*;
    pub use crate::{Context, ContextBuilder, GlibError, Result};
}

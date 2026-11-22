pub mod null;
pub use null::NullBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind { Vulkan, Null }

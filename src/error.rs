use thiserror::Error;

pub type Result<T, E = GlibError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum GlibError {
    #[error("out of memory: requested {requested} bytes")]
    OutOfMemory { requested: u64 },
    #[error("invalid handle")]
    InvalidHandle,
    #[error("render graph cycle detected at pass `{0}`")]
    GraphCycle(String),
    #[error("backend error: {0}")]
    Backend(String),
}

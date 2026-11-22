use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
#[repr(C)]
pub struct Extent3d { pub width: u32, pub height: u32, pub depth: u32 }

impl Extent3d {
    pub const fn wh(w: u32, h: u32) -> Self { Self { width: w, height: h, depth: 1 } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Extent2d { pub width: u32, pub height: u32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format { Bgra8Srgb, Rgba16Float, Depth32Float }
impl Default for Format { fn default() -> Self { Self::Bgra8Srgb } }

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VisBufPayload {
    pub packed: u64,
}

impl VisBufPayload {
    pub fn new(triangle_id: u32, instance_id: u32) -> Self {
        Self { packed: ((instance_id as u64) << 32) | (triangle_id as u64) }
    }
    pub fn triangle_id(&self) -> u32 { (self.packed & 0xFFFFFFFF) as u32 }
    pub fn instance_id(&self) -> u32 { (self.packed >> 32) as u32 }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MeshCluster {
    pub base_vertex: u32,
    pub triangle_count: u32,
}

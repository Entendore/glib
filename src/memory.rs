use crate::error::*;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct MemoryBlock {
    pub pool_id: u32,
    pub offset: u64,
    pub size: u64,
}

pub struct SparseMemoryPool {
    pub inner: Arc<SparseMemoryInner>,
}

pub struct SparseMemoryInner {
    pub free_blocks: Mutex<Vec<MemoryBlock>>,
    pub block_size: u64,
}

impl SparseMemoryPool {
    pub fn new(block_size: u64, blocks: u64) -> Self {
        let mut vec = Vec::new();
        for i in 0..blocks {
            vec.push(MemoryBlock { pool_id: 0, offset: i * block_size, size: block_size });
        }
        Self { inner: Arc::new(SparseMemoryInner { free_blocks: Mutex::new(vec), block_size }) }
    }

    pub fn allocate_page(&self) -> Result<MemoryBlock> {
        let mut blocks = self.inner.free_blocks.lock();
        blocks.pop().ok_or(GlibError::OutOfMemory { requested: self.inner.block_size })
    }

    pub fn free_page(&self, block: MemoryBlock) {
        let mut blocks = self.inner.free_blocks.lock();
        blocks.push(block);
    }
}

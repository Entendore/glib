use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct PipelineStage: u32 {
        const TOP_OF_PIPE = 1 << 0;
        const COMPUTE_SHADER = 1 << 1;
        const TRANSFER = 1 << 2;
        const BOTTOM_OF_PIPE = 1 << 3;
    }
    
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct Access: u32 {
        const MEMORY_READ = 1 << 0;
        const MEMORY_WRITE = 1 << 1;
    }
}

pub struct TimelineSemaphore {
    pub current_value: u64,
}

impl TimelineSemaphore {
    pub fn new() -> Self { Self { current_value: 0 } }
    pub fn advance(&mut self) -> u64 {
        self.current_value += 1;
        self.current_value
    }
}

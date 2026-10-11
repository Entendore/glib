use glib::prelude::*;
use glib::gpu_driven::VisBufPayload;
use glib::sync::TimelineSemaphore;
use glib::memory::SparseMemoryPool;
use glib::disposal::DisposalQueue;

#[test]
fn test_visbuf_payload_packing() {
    let payload = VisBufPayload::new(0xDEADBEEF, 0x12345678);
    assert_eq!(payload.triangle_id(), 0xDEADBEEF);
    assert_eq!(payload.instance_id(), 0x12345678);
}

#[test]
fn test_timeline_semaphore_monotonicity() {
    let mut sem = TimelineSemaphore::new();
    assert_eq!(sem.advance(), 1);
    assert_eq!(sem.advance(), 2);
    assert_eq!(sem.current_value, 2);
}

#[test]
fn test_sparse_memory_pool_alloc_free() {
    let pool = SparseMemoryPool::new(1024, 2);
    let block1 = pool.allocate_page().expect("Allocation should succeed");
    let _block2 = pool.allocate_page().expect("Allocation should succeed");
    
    assert!(pool.allocate_page().is_err(), "Pool should be exhausted");
    
    pool.free_page(block1);
    let block4 = pool.allocate_page().expect("Reallocation should succeed");
    assert_eq!(block4.offset, 0, "Should reuse the freed offset");
}

#[test]
fn test_disposal_queue_purge_logic() {
    let queue = DisposalQueue::new();
    
    // Resource 1 is queued to die after frame 5
    queue.dispose(101, 5);
    // Resource 2 is queued to die after frame 10
    queue.dispose(102, 10);
    
    // GPU completes frame 5
    queue.purge(5);
    
    let pending = queue.pending.lock();
    assert_eq!(pending.len(), 1, "Only resource 2 should remain");
    assert_eq!(pending[0].0, 102);
}

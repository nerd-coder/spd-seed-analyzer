//! Counting global allocator: live/peak heap bytes and allocation counts.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub struct Counting;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

fn grow(size: usize) {
    COUNT.fetch_add(1, Relaxed);
    BYTES.fetch_add(size, Relaxed);
    let live = LIVE.fetch_add(size, Relaxed) + size;
    PEAK.fetch_max(live, Relaxed);
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        grow(layout.size());
        System.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        grow(layout.size());
        System.alloc_zeroed(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Relaxed);
        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        LIVE.fetch_sub(layout.size(), Relaxed);
        grow(new_size);
        System.realloc(ptr, layout, new_size)
    }
}

/// Allocation counters accumulated since the last [`reset`].
#[derive(Clone, Copy)]
pub struct Snapshot {
    /// Peak live heap above the live heap at reset time.
    pub peak_bytes: usize,
    pub allocations: usize,
    pub allocated_bytes: usize,
}

pub fn reset() -> usize {
    let live = LIVE.load(Relaxed);
    PEAK.store(live, Relaxed);
    COUNT.store(0, Relaxed);
    BYTES.store(0, Relaxed);
    live
}

pub fn snapshot(baseline: usize) -> Snapshot {
    Snapshot {
        peak_bytes: PEAK.load(Relaxed).saturating_sub(baseline),
        allocations: COUNT.load(Relaxed),
        allocated_bytes: BYTES.load(Relaxed),
    }
}

//! Port of tikv-jemallocator/tests/smoke.rs plus additional allocator invariants.
use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

use tcmalloc_better::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

#[test]
fn smoke() {
    let mut values = Vec::new();
    values.reserve(1);
    values.push(3);
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], 3);
}

#[test]
fn overaligned() {
    let size = 8;
    let align = 16;
    let iterations = 100;
    unsafe {
        let blocks: Vec<_> = (0..iterations)
            .map(|_| {
                let ptr = GLOBAL.alloc(Layout::from_size_align(size, align).unwrap());
                assert!(!ptr.is_null());
                ptr
            })
            .collect();
        for &ptr in &blocks {
            assert_eq!((ptr as usize) % align, 0);
        }
        for &ptr in &blocks {
            GLOBAL.dealloc(ptr, Layout::from_size_align(size, align).unwrap());
        }
    }
}

#[test]
fn box_drop_runs() {
    struct Recorder<'a>(&'a AtomicUsize);

    impl Drop for Recorder<'_> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let counter = AtomicUsize::new(0);
    {
        let _owned = Box::new(Recorder(&counter));
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn vec_reallocation_preserves_contents() {
    let mut values: Vec<u64> = (0..64).collect();
    let snapshot = values.clone();
    values.reserve(128);
    assert_eq!(values, snapshot);
    values.extend(snapshot.iter().copied());
    assert_eq!(values.len(), snapshot.len() * 2);
    values.retain(|entry| entry % 2 == 0);
    assert!(values.iter().all(|entry| entry % 2 == 0));
}

#[test]
fn string_reserve_preserves_contents() {
    let mut text = String::from("tc");
    text.push_str("malloc");
    let baseline = text.clone();
    text.reserve(128);
    assert_eq!(text, baseline);
    text.shrink_to_fit();
    assert!(text.capacity() >= text.len());
    assert!(text.ends_with('c'));
}

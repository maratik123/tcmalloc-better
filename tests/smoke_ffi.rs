//! Port of tikv-jemallocator/tests/smoke_ffi.rs.

use tcmalloc_better::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

#[test]
fn smoke() {
    let alignment = core::mem::align_of::<u32>();
    let size = core::mem::size_of::<u32>();
    let ptr = unsafe { libtcmalloc_sys::BridgeTCMallocInternalNewAlignedNothrow(size, alignment) }
        as *mut u32;
    assert!(!ptr.is_null());
    unsafe {
        ptr.write(0xDECADE);
        assert_eq!(ptr.read(), 0xDECADE);
        libtcmalloc_sys::TCMallocInternalDeleteSizedAligned(ptr.cast(), size, alignment);
    }
}

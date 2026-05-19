use std::sync::atomic::{AtomicI16, AtomicU16};

#[repr(transparent)]
pub struct HkPropertyBag(usize);

#[repr(C)]
pub struct HkBaseObject {
    pub vftable: usize,
}

#[repr(C)]
pub struct HkReferencedObject {
    pub base_object: HkBaseObject,
    pub property_bag: HkPropertyBag,
    pub mem_size_and_flags: AtomicU16,
    pub reference_count: AtomicI16,
}

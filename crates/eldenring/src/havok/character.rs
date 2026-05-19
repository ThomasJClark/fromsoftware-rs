use core::slice;

use fromsoftware_shared::OwnedPtr;

use crate::havok::{HkQsTransform, HkReferencedObject, HkaSkeleton};

#[repr(C)]
pub struct HkbCharacterSetup {
    pub referenced_object: HkReferencedObject,
    _unk18: [u8; 0x10],
    pub skeleton: OwnedPtr<HkaSkeleton>,
}

#[repr(C)]
struct SkeletonState {
    _unk0: [u8; 0x54],
    poses_offset: i32,
}

#[repr(transparent)]
pub struct SkeletonStatePointer(OwnedPtr<SkeletonState>);

impl SkeletonStatePointer {
    pub fn poses(&self, len: usize) -> &'_ [HkQsTransform] {
        let base_addr = self.0.as_ptr();
        unsafe {
            let data = base_addr.byte_offset(self.0.poses_offset as isize).cast();
            slice::from_raw_parts(data, len)
        }
    }

    pub fn poses_mut(&mut self, len: usize) -> &'_ mut [HkQsTransform] {
        let base_addr = self.0.as_ptr();
        unsafe {
            let data = base_addr.byte_offset(self.0.poses_offset as isize).cast();
            slice::from_raw_parts_mut(data, len)
        }
    }
}

#[repr(C)]
pub struct HkbCharacter {
    pub referenced_object: HkReferencedObject,
    _unk18: [u8; 0x20],
    pub state: OwnedPtr<SkeletonStatePointer>,
    _unk40: [u8; 0x50],
    pub setup: OwnedPtr<HkbCharacterSetup>,
}

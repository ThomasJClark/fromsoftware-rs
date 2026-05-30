use core::slice;

use fromsoftware_shared::OwnedPtr;

use crate::havok::{HkQsTransform, HkQuaternion, HkReferencedObject, HkVector4, HkaSkeleton};

#[repr(C)]
pub struct HkbCharacterSetup {
    pub referenced_object: HkReferencedObject,
    _unk18: [u8; 0x10],
    pub skeleton: OwnedPtr<HkaSkeleton>,
}

#[repr(C)]
struct HavokCharacterState {
    _unk0: [u8; 0x14],
    motion_offset: i32,
    _unk18: [u8; 0x3c],
    poses_offset: i32,
}

#[repr(transparent)]
pub struct HavokCharacterStatePointer(OwnedPtr<HavokCharacterState>);

impl HavokCharacterStatePointer {
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

    pub fn motion(&self) -> &'_ HkQsTransform {
        let base_addr = self.0.as_ptr();
        unsafe { &*base_addr.byte_offset(self.0.motion_offset as isize).cast() }
    }

    pub fn motion_mut(&mut self) -> &'_ mut HkQsTransform {
        let base_addr = self.0.as_ptr();
        unsafe { &mut *base_addr.byte_offset(self.0.motion_offset as isize).cast() }
    }

    /// The current linear velocity of the character, in local space, premultiplied by the frame
    /// time
    pub fn linear_velocity(&self) -> &'_ HkVector4 {
        &self.motion().translation
    }

    /// The current linear velocity of the character, in local space, premultiplied by the frame
    /// time
    pub fn linear_velocity_mut(&mut self) -> &'_ mut HkVector4 {
        &mut self.motion_mut().translation
    }

    pub fn angular_velocity(&self) -> &'_ HkQuaternion {
        &self.motion().rotation
    }

    pub fn angular_velocity_mut(&mut self) -> &'_ mut HkQuaternion {
        &mut self.motion_mut().rotation
    }
}

#[repr(C)]
pub struct KkbCharacterControllerDriver {
    pub referenced_object: HkReferencedObject,
    pub unk18: usize,
    pub unk20: HkVector4,
    pub unk30: HkVector4,
    pub unk40: i32,
    pub unk44: bool,
    pub unk50: HkVector4,
    pub unk60: HkVector4,
    pub unk70: HkVector4,
    pub unk80: HkVector4,
    pub unk90: bool,
    pub unk98: usize,
    pub unka0: i32,
}

#[repr(C)]
pub struct HkbCharacter {
    pub referenced_object: HkReferencedObject,
    _unk18: [u8; 0x20],
    pub state: OwnedPtr<HavokCharacterStatePointer>,
    _unk40: [u8; 0x50],
    pub setup: OwnedPtr<HkbCharacterSetup>,
}

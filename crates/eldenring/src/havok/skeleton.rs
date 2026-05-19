use crate::havok::{HkArray, HkQsTransform, HkReal, HkReferencedObject, HkStringPtr};

#[repr(C)]
pub struct HkaBone {
    pub name: HkStringPtr,
    pub lock_translation: bool,
}

#[repr(C)]
pub struct HkaSkeletonLocalFrameOnBone {
    pub local_frame: usize,
    pub bone_index: u16,
}

#[repr(C)]
pub struct HkaSkeletonPartition {
    pub name: HkStringPtr,
    pub start_bone_index: i16,
    pub num_bones: i16,
}

#[repr(C)]
pub struct HkaSkeleton {
    pub referenced_object: HkReferencedObject,
    pub name: HkStringPtr,
    pub parent_indices: HkArray<u16>,
    pub bones: HkArray<HkaBone>,
    pub reference_pose: HkArray<HkQsTransform>,
    pub reference_floats: HkArray<HkReal>,
    pub float_slots: HkArray<HkStringPtr>,
    pub local_frames: HkArray<HkaSkeletonLocalFrameOnBone>,
    pub partitions: HkArray<HkaSkeletonPartition>,
}

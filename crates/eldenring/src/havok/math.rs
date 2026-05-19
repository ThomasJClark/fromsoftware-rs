pub type HkReal = f32;

pub type HkVector4 = glam::f32::Vec4;

pub type HkQuaternion = glam::f32::Quat;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HkQsTransform {
    pub translation: HkVector4,
    pub rotation: HkQuaternion,
    pub scale: HkVector4,
}

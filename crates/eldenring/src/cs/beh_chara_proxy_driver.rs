use std::ptr::NonNull;

use crate::{cs::ChrIns, havok::KkbCharacterControllerDriver};

#[repr(C)]
pub struct CSBehCharaProxyDriver {
    pub base: KkbCharacterControllerDriver,
    pub chr: NonNull<ChrIns>,
}

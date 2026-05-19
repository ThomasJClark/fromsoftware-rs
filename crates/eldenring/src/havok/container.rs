use std::{
    ffi::{CStr, c_char},
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    ptr::NonNull,
    slice,
};

#[repr(C)]
pub struct HkArray<T> {
    data: *const T,
    len: i32,
    capacity_and_flags: u32,
}

impl<T> HkArray<T> {
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { self.data.as_ref() }.map_or(&[], |ptr| unsafe {
            slice::from_raw_parts(ptr as _, self.len as usize)
        })
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { self.data.as_ref() }.map_or(&mut [], |ptr| unsafe {
            slice::from_raw_parts_mut(ptr as *const _ as *mut _, self.len as usize)
        })
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    pub fn capacity(&self) -> usize {
        (self.capacity_and_flags & 0x3FFFFFFF) as usize
    }

    pub fn dont_deallocate(&self) -> bool {
        (self.capacity_and_flags & 0x80000000) != 0
    }
}

impl<T> Index<usize> for HkArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.as_slice().index(index)
    }
}

impl<T> IndexMut<usize> for HkArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.as_mut_slice().index_mut(index)
    }
}

impl Debug for HkArray<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HkArray(")?;
        Debug::fmt(&self.as_slice(), f)?;
        f.write_str(")")
    }
}

#[repr(transparent)]
pub struct HkStringPtr(Option<NonNull<c_char>>);

impl HkStringPtr {
    pub fn to_str(&self) -> &str {
        let Some(ptr) = self.0 else {
            return "";
        };

        unsafe {
            let c_str = CStr::from_ptr(ptr.as_ptr());
            str::from_utf8_unchecked(c_str.to_bytes())
        }
    }
}

impl Display for HkStringPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_str(), f)
    }
}

impl Debug for HkStringPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HkStringPtr(")?;
        Debug::fmt(&self.to_str(), f)?;
        f.write_str(")")
    }
}

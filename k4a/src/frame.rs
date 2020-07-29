use std::marker::PhantomData;
use std::ops::Deref;

use super::body::Skeleton;
use super::error::{k4a_result, Error};

#[derive(Debug)]
pub struct Frame<'d> {
    frame_handle: libk4a_sys::k4abt_frame_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Frame<'d> {
    /// # Safety
    ///
    /// Ensure `frame_handle` is unique, and not a null or not dangling
    pub unsafe fn from_handle(frame_handle: libk4a_sys::k4abt_frame_t) -> Self {
        Self {
            frame_handle,
            _phantom: PhantomData,
        }
    }

    pub fn get_num_bodies(&self) -> u32 {
        unsafe { libk4a_sys::k4abt_frame_get_num_bodies(self.frame_handle) }
    }

    pub fn get_body_id(&self, index: u32) -> u32 {
        unsafe { libk4a_sys::k4abt_frame_get_body_id(self.frame_handle, index) }
    }

    pub fn get_body_skeleton(&self, index: u32) -> Result<Skeleton, Error> {
        let mut skelton = std::mem::MaybeUninit::<Skeleton>::uninit();
        let result = unsafe {
            libk4a_sys::k4abt_frame_get_body_skeleton(
                self.frame_handle,
                index,
                skelton.as_mut_ptr(),
            )
        };
        k4a_result(result)?;
        Ok(unsafe { skelton.assume_init() })
    }
}

impl<'d> Clone for Frame<'d> {
    fn clone(&self) -> Self {
        unsafe {
            libk4a_sys::k4abt_frame_reference(self.frame_handle);
            Self::from_handle(self.frame_handle)
        }
    }
}

impl<'d> Drop for Frame<'d> {
    fn drop(&mut self) {
        let frame_handle = self.frame_handle;
        if frame_handle.is_null() {
            return;
        }
        unsafe {
            libk4a_sys::k4abt_frame_release(frame_handle);
        }
        self.frame_handle = std::ptr::null_mut();
    }
}

impl<'d> Deref for Frame<'d> {
    type Target = libk4a_sys::k4abt_frame_t;

    fn deref(&self) -> &Self::Target {
        &self.frame_handle
    }
}

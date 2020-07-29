use std::{marker::PhantomData, ops::Deref};

#[derive(Debug)]
pub struct Capture<'d> {
    capture_handle: libk4a_sys::k4a_capture_t,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Capture<'d> {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_handle(capture_handle: libk4a_sys::k4a_capture_t) -> Self {
        Self {
            capture_handle,
            _phantom: PhantomData,
        }
    }
}

impl<'d> Clone for Capture<'d> {
    fn clone(&self) -> Self {
        unsafe {
            libk4a_sys::k4a_capture_reference(self.capture_handle);
            Self::from_handle(self.capture_handle)
        }
    }
}

impl<'d> Drop for Capture<'d> {
    fn drop(&mut self) {
        let capture_handle = self.capture_handle;
        if capture_handle.is_null() {
            return;
        }
        unsafe {
            libk4a_sys::k4a_capture_release(capture_handle);
        }
        self.capture_handle = std::ptr::null_mut();
    }
}

impl<'d> Deref for Capture<'d> {
    type Target = libk4a_sys::k4a_capture_t;

    fn deref(&self) -> &Self::Target {
        &self.capture_handle
    }
}

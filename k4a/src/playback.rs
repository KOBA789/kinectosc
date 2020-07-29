use std::ffi::CString;

use super::calibration::Calibration;
use super::capture::Capture;
use super::error::{k4a_result, k4a_stream_result, Error, StreamError};

pub struct Playback {
    playback_handle: libk4a_sys::k4a_playback_t,
}

impl Playback {
    pub fn open(path: &str) -> Result<Self, Error> {
        let cstring = CString::new(path).unwrap();
        let mut playback_handle = std::ptr::null_mut();
        let result = unsafe { libk4a_sys::k4a_playback_open(cstring.as_ptr(), &mut playback_handle) };
        k4a_result(result)?;
        Ok(Playback { playback_handle })
    }

    pub fn get_calibration(&self) -> Result<Calibration, Error> {
        let mut calibration = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            libk4a_sys::k4a_playback_get_calibration(
                self.playback_handle,
                calibration.as_mut_ptr(),
            )
        };
        k4a_result(result)?;
        Ok(unsafe { calibration.assume_init() })
    }

    pub fn get_capture(&self) -> Result<Capture, StreamError> {
        let mut capture_handle = std::ptr::null_mut();
        let wait_result = unsafe {
            libk4a_sys::k4a_playback_get_next_capture(self.playback_handle, &mut capture_handle)
        };
        k4a_stream_result(wait_result)?;
        Ok(unsafe { Capture::from_handle(capture_handle) })
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        let playback_handle = std::mem::replace(&mut self.playback_handle, std::ptr::null_mut());
        if playback_handle.is_null() {
            return;
        }
        unsafe { libk4a_sys::k4a_playback_close(playback_handle) };
    }
}

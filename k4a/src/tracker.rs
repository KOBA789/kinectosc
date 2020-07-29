use super::calibration::Calibration;
use super::capture::Capture;
use super::error::{k4a_result, k4a_wait_result, Error, WaitError};
use super::frame::Frame;
use super::tracker_configuration::TrackerConfiguration;

pub struct Tracker {
    tracker_handle: libk4a_sys::k4abt_tracker_t,
}

impl Tracker {
    pub fn create(
        calibration: &Calibration,
        tracker_configuration: TrackerConfiguration,
    ) -> Result<Self, Error> {
        let mut tracker_handle = std::ptr::null_mut();
        let result = unsafe {
            libk4a_sys::k4abt_tracker_create(
                calibration,
                tracker_configuration,
                &mut tracker_handle,
            )
        };
        k4a_result(result)?;
        Ok(Tracker { tracker_handle })
    }

    pub fn enqueue_capture(&self, capture: Capture, timeout: i32) -> Result<(), WaitError> {
        let wait_result = unsafe {
            libk4a_sys::k4abt_tracker_enqueue_capture(self.tracker_handle, *capture, timeout)
        };
        k4a_wait_result(wait_result)?;
        Ok(())
    }

    pub fn k4abt_tracker_pop_result(&self, timeout: i32) -> Result<Frame, WaitError> {
        let mut frame_handle = std::ptr::null_mut();
        let wait_result = unsafe {
            libk4a_sys::k4abt_tracker_pop_result(self.tracker_handle, &mut frame_handle, timeout)
        };
        k4a_wait_result(wait_result)?;
        Ok(unsafe { Frame::from_handle(frame_handle) })
    }
}

impl Drop for Tracker {
    fn drop(&mut self) {
        let tracker_handle = self.tracker_handle;
        if tracker_handle.is_null() {
            return;
        }
        unsafe {
            libk4a_sys::k4abt_tracker_shutdown(tracker_handle);
            libk4a_sys::k4abt_tracker_destroy(tracker_handle);
        }
        self.tracker_handle = std::ptr::null_mut();
    }
}

use std::ops::Deref;

use super::calibration::Calibration;
use super::capture::Capture;
use super::device_configuration::{ColorResolution, DepthMode, DeviceConfiguration};
use super::error::{k4a_result, k4a_wait_result, Error, WaitError};

pub struct Device {
    device_handle: libk4a_sys::k4a_device_t,
}

impl Device {
    pub fn open(index: u32) -> Result<Self, Error> {
        let mut device_handle = std::ptr::null_mut();
        let result = unsafe { libk4a_sys::k4a_device_open(index, &mut device_handle) };
        k4a_result(result)?;
        Ok(Device { device_handle })
    }

    pub fn get_calibration(
        &self,
        depth_mode: DepthMode,
        color_resolution: ColorResolution,
    ) -> Result<Calibration, Error> {
        let mut calibration = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            libk4a_sys::k4a_device_get_calibration(
                self.device_handle,
                depth_mode,
                color_resolution,
                calibration.as_mut_ptr(),
            )
        };
        k4a_result(result)?;
        Ok(unsafe { calibration.assume_init() })
    }

    pub fn start_cameras(
        self,
        device_configuration: DeviceConfiguration,
    ) -> Result<RunningDevice, Error> {
        RunningDevice::start(self, device_configuration)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let device_handle = std::mem::replace(&mut self.device_handle, std::ptr::null_mut());
        if device_handle.is_null() {
            return;
        }
        unsafe { libk4a_sys::k4a_device_close(device_handle) };
    }
}

pub struct RunningDevice {
    device: Device,
    depth_mode: DepthMode,
    color_resolution: ColorResolution,
}

impl RunningDevice {
    pub fn start(device: Device, device_configuration: DeviceConfiguration) -> Result<Self, Error> {
        let result = unsafe {
            libk4a_sys::k4a_device_start_cameras(device.device_handle, &device_configuration)
        };
        k4a_result(result)?;
        Ok(RunningDevice {
            device,
            depth_mode: device_configuration.depth_mode,
            color_resolution: device_configuration.color_resolution,
        })
    }

    pub fn get_calibration(&self) -> Result<Calibration, Error> {
        self.device
            .get_calibration(self.depth_mode, self.color_resolution)
    }

    pub fn get_capture(&self, timeout: i32) -> Result<Capture, WaitError> {
        let mut capture_handle = std::ptr::null_mut();
        let wait_result = unsafe {
            libk4a_sys::k4a_device_get_capture(self.device_handle, &mut capture_handle, timeout)
        };
        k4a_wait_result(wait_result)?;
        Ok(unsafe { Capture::from_handle(capture_handle) })
    }

    pub fn stop(mut self) -> Device {
        let unsafe_clone = Device {
            device_handle: self.device.device_handle,
        };
        let original = std::mem::replace(&mut self.device, unsafe_clone);
        drop(self);
        original
    }
}

impl Drop for RunningDevice {
    fn drop(&mut self) {
        let device_handle = self.device.device_handle;
        if device_handle.is_null() {
            return;
        }
        unsafe { libk4a_sys::k4a_device_stop_cameras(device_handle) };
    }
}

impl Deref for RunningDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

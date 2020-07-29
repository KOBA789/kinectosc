#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for k4a_device_configuration_t {
    fn default() -> Self {
        k4a_device_configuration_t {
            color_format: k4a_image_format_t::K4A_IMAGE_FORMAT_COLOR_MJPG,
            color_resolution: k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF,
            depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_OFF,
            camera_fps: k4a_fps_t::K4A_FRAMES_PER_SECOND_30,
            synchronized_images_only: false,
            depth_delay_off_color_usec: 0,
            wired_sync_mode: k4a_wired_sync_mode_t::K4A_WIRED_SYNC_MODE_STANDALONE,
            subordinate_delay_off_master_usec: 0,
            disable_streaming_indicator: false,
        }
    }
}

impl Default for k4abt_tracker_configuration_t {
    fn default() -> Self {
        k4abt_tracker_configuration_t {
            sensor_orientation: k4abt_sensor_orientation_t::K4ABT_SENSOR_ORIENTATION_DEFAULT,
            processing_mode: k4abt_tracker_processing_mode_t::K4ABT_TRACKER_PROCESSING_MODE_GPU,
            gpu_device_id: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct k4a_float3_t {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct k4a_float2_t {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct k4a_quaternion_t {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

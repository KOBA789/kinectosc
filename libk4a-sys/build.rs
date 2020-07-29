extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\windows-desktop\\amd64\\release\\lib");
        println!("cargo:rustc-link-search=C:\\Program Files\\Azure Kinect Body Tracking SDK\\sdk\\windows-desktop\\amd64\\release\\lib");
    }
    
    println!("cargo:rustc-link-lib=k4a");
    println!("cargo:rustc-link-lib=k4arecord");
    println!("cargo:rustc-link-lib=k4abt");
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .prepend_enum_name(false)
        .derive_copy(false)
        .no_copy("k4a_body_t|k4a_skelton_t|k4a_joint_t")
        .newtype_enum("k4a_result_t|k4a_buffer_result_t|k4a_calibration_model_type_t|k4a_calibration_type_t|k4a_color_control_command_t|k4a_color_control_mode_t|k4a_firmware_build_t|k4a_firmware_signature_t|k4a_log_level_t|k4a_playback_seek_origin_t|k4a_stream_result_t|k4a_transformation_interpolation_type_t|k4a_wait_result_t|k4a_image_format_t|k4a_color_resolution_t|k4a_depth_mode_t|k4a_fps_t|k4a_wired_sync_mode_t|k4abt_joint_id_t|k4abt_sensor_orientation_t|k4abt_tracker_processing_mode_t|k4abt_joint_confidence_level_t")
        .blacklist_type("k4a_float2_t|k4a_float2_t__xy|k4a_float3_t|k4a_float3_t__xyz|k4a_quaternion_t|k4a_quaternion_t__wxyz")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    if cfg!(target_os = "windows") {
        builder = builder
            .clang_arg("-IC:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\include")
            .clang_arg("-IC:\\Program Files\\Azure Kinect Body Tracking SDK\\sdk\\include");
    }

    let bindings =
        builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

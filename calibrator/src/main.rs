use std::ffi::CString;
use openvr::system::{Event, event::Controller};
use nalgebra::{Translation3, Point3, UnitQuaternion, Vector3};

mod calibrator;
mod calibration;

fn main() {
    let profile_path = "calibration_profile.json";
    calibration::save(profile_path,&calibration::Profile::default()).unwrap();
    let ctx = unsafe {
        openvr::init(openvr::ApplicationType::Other)
    }.unwrap();
    let sys = ctx.system().unwrap();
    let osct_hand_left_sn = CString::new("OSCTL").unwrap();
    let osct_hand_right_sn = CString::new("OSCTR").unwrap();
    let mut osct_hand_left_id = None;
    let mut osct_hand_right_id = None;
    for idx in 0..openvr::MAX_TRACKED_DEVICE_COUNT {
        let device = idx as u32;
        if let Ok(serial_number) = sys.string_tracked_device_property(device, openvr::property::SerialNumber_String) {
            if serial_number == osct_hand_left_sn {
                osct_hand_left_id = Some(idx);
            }
            if serial_number == osct_hand_right_sn {
                osct_hand_right_id = Some(idx);
            }
        }
    }
    let osct_hand_left_id = osct_hand_left_id.unwrap();
    let osct_hand_right_id = osct_hand_right_id.unwrap();
    let mut ref_tri = calibrator::AvgTriangle::new();
    let mut tgt_tri = calibrator::AvgTriangle::new();
    'outer: loop {
        while let Some((ev, pose)) = sys.poll_next_event_with_pose(openvr::TrackingUniverseOrigin::RawAndUncalibrated) {
            if let Event::ButtonPress(Controller { button: 33 }) = ev.event {
                let id = ev.tracked_device_index;
                if let Some(role) = sys.get_controller_role_for_tracked_device_index(id) {
                    println!("{:?}", role);
                    let target_id = match role {
                        openvr::TrackedControllerRole::LeftHand => osct_hand_left_id,
                        openvr::TrackedControllerRole::RightHand => osct_hand_right_id,
                    };
                    let poses = sys.device_to_absolute_tracking_pose(
                        openvr::TrackingUniverseOrigin::RawAndUncalibrated, 
                        0.0
                    );
                    let ref_mat = poses[ev.tracked_device_index as usize].device_to_absolute_tracking();
                    let ref_pos = tracking_matrix_to_point3(ref_mat);
                    let tgt_mat = poses[target_id].device_to_absolute_tracking();
                    let tgt_pos = tracking_matrix_to_point3(tgt_mat);
                    println!("({}), ({})", ref_pos, tgt_pos);
                    ref_tri.push_point(ref_pos);
                    tgt_tri.push_point(tgt_pos);
                    if let (Some(ref_tri), Some(tgt_tri)) = (ref_tri.triangle(), tgt_tri.triangle()) {
                        let (wfd_translation, wfd_rotation) = calibrator::calibrate(&ref_tri, &tgt_tri);
                        println!("translation: {}", wfd_translation);
                        println!("rotation: {}", wfd_rotation);
                        let profile = calibration::Profile::new(wfd_translation, wfd_rotation);
                        calibration::save(profile_path, &profile).unwrap();
                        break 'outer;
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn tracking_matrix_to_point3(mat: &[[f32; 4]; 3]) -> Point3<f64> {
    Point3::new(mat[0][3] as f64, mat[1][3] as f64, mat[2][3] as f64)
}

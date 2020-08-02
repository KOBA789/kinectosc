use std::net::SocketAddr;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use k4a::joint_id;

mod active_sensor;
mod osc;
mod calibration;
mod kinect;
mod profile_provider;
mod filter;

fn main() {
    let is_running = Arc::new(AtomicBool::new(true));
    let r = is_running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();

    let mut profile = profile_provider::ProfileProvider::new("calibration_profile.json".into());

    let target_addr: SocketAddr = "127.0.0.1:8124"
        .parse()
        .expect("failed to parse target addr");
    let mut osc_client = osc::Client::new("0.0.0.0:9010", target_addr).unwrap();

    let sensor_config = k4a::DeviceConfiguration {
        depth_mode: k4a::DepthMode::K4A_DEPTH_MODE_NFOV_UNBINNED,
        ..Default::default()
    };
    let tracker_config = k4a::TrackerConfiguration {
        gpu_device_id: 1,
        ..Default::default()
    };
    let kinect = kinect::Kinect::open_sensor(0, sensor_config, tracker_config).unwrap();
    let mut filter = filter::KinectJointFilter::new(filter::SmoothParams::default());
    loop {
        if !is_running.load(Ordering::SeqCst) {
            break;
        }
        profile.reload_if_updated();

        let frame = kinect.next_frame().unwrap();
        let num_bodies = frame.get_num_bodies();
        if num_bodies == 0 {
            continue;
        }
        let skeleton: k4a::Skeleton = frame
            .get_body_skeleton(0)
            .unwrap();
        filter.update(&skeleton);
        let pelvis = &filter.joints[joint_id::K4ABT_JOINT_PELVIS as usize];
        let message = build_message_from_filtered_joint(&profile, pelvis, 0);
        osc_client.send(message).unwrap();
        let ankle_left = &filter.joints[joint_id::K4ABT_JOINT_FOOT_LEFT as usize];
        let message = build_message_from_filtered_joint(&profile, ankle_left, 1);
        osc_client.send(message).unwrap();
        let ankle_right = &filter.joints[joint_id::K4ABT_JOINT_FOOT_RIGHT as usize];
        let message = build_message_from_filtered_joint(&profile, ankle_right, 2);
        osc_client.send(message).unwrap();

        let hand_left = &filter.joints[joint_id::K4ABT_JOINT_HAND_LEFT as usize];
        let message = build_message_from_filtered_joint(&profile, hand_left, 3);
        osc_client.send(message).unwrap();
        let hand_right = &filter.joints[joint_id::K4ABT_JOINT_HAND_RIGHT as usize];
        let message = build_message_from_filtered_joint(&profile, hand_right, 4);
        osc_client.send(message).unwrap();
    }
}

fn build_message_from_filtered_joint(profile: &profile_provider::ProfileProvider, joint: &filter::FilteredJoint, id: u32) -> osc::PoseMessage {
    let wfd_rotation = profile.wfd_rotation;
    let wfd_translation = profile.wfd_translation;
    osc::PoseMessage {
        id,
        is_valid: joint.frame_count > 1,
        wfd_rotation,
        wfd_translation,
        position: joint.predicted_position / 1000.0,
        orientation: joint.raw_orientation,
        velocity: joint.trend / 1000.0 * 30.0,
    }
}

use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc;
use std::time;

use nalgebra::{UnitQuaternion, Point3, Translation3, Quaternion, Rotation, Vector3};
use notify::{Watcher, RecursiveMode, watcher};

use active_sensor::{ActiveSensor, RealtimePlayback};
use k4a::joint_id;

mod active_sensor;
mod osc;
mod calibration;
mod conv;
mod kinect;
mod profile_provider;

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
        let pelvis: &k4a::Joint = &skeleton.joints[joint_id::K4ABT_JOINT_PELVIS as usize];
        let message = build_message_from_joint(&profile, pelvis, 0);
        osc_client.send(message).unwrap();
        let ankle_left: &k4a::Joint = &skeleton.joints[joint_id::K4ABT_JOINT_FOOT_LEFT as usize];
        let message = build_message_from_joint(&profile, ankle_left, 1);
        osc_client.send(message).unwrap();
        let ankle_right: &k4a::Joint = &skeleton.joints[joint_id::K4ABT_JOINT_FOOT_RIGHT as usize];
        let message = build_message_from_joint(&profile, ankle_right, 2);
        osc_client.send(message).unwrap();

        let hand_left: &k4a::Joint = &skeleton.joints[joint_id::K4ABT_JOINT_HAND_LEFT as usize];
        let message = build_message_from_joint(&profile, hand_left, 3);
        osc_client.send(message).unwrap();
        let hand_right: &k4a::Joint = &skeleton.joints[joint_id::K4ABT_JOINT_HAND_RIGHT as usize];
        let message = build_message_from_joint(&profile, hand_right, 4);
        osc_client.send(message).unwrap();
    }
}

fn build_message_from_joint(profile: &profile_provider::ProfileProvider, joint: &k4a::Joint, id: u32) -> osc::PoseMessage {
    let wfd_rotation = profile.wfd_rotation;
    let wfd_translation = profile.wfd_translation;
    osc::PoseMessage {
        id,
        result: 200,
        wfd_rotation,
        wfd_translation,
        position: Point3::new(
            joint.position.x as f64 / 1000.0,
            joint.position.y as f64 / 1000.0,
            joint.position.z as f64 / 1000.0,
        ),
        orientation: UnitQuaternion::from_quaternion(Quaternion::new(
            joint.orientation.w as f64,
            joint.orientation.x as f64,
            joint.orientation.y as f64,
            joint.orientation.z as f64,
        )),
    }
}

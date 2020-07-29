use active_sensor::{ActiveSensor, RealtimePlayback};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

mod active_sensor;

fn main() {
    let is_running = Arc::new(AtomicBool::new(true));
    let r = is_running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:9010").expect("failed to bind port");
    let target_addr: SocketAddr = "127.0.0.1:9012"
        .parse()
        .expect("failed to parse target addr");

    let args: Vec<_> = std::env::args().collect();
    let sensor: Box<dyn ActiveSensor> = if args.len() == 1 {
        let device = k4a::Device::open(0).expect("failed to open device");
        let config = k4a::DeviceConfiguration {
            depth_mode: k4a::DepthMode::K4A_DEPTH_MODE_WFOV_2X2BINNED,
            ..Default::default()
        };
        let running = device
            .start_cameras(config)
            .expect("failed to start cameras");
        Box::new(running)
    } else if args.len() == 2 {
        let filename = &args[1];
        let playback = k4a::Playback::open(&filename).expect("failed to open recording");
        let realtime_playback = RealtimePlayback::new(playback, 30);
        Box::new(realtime_playback)
    } else {
        panic!("invalid argument");
    };

    let calibration = sensor.get_calibration().expect("failed to get calibration");
    let tracker_configuration = k4a::TrackerConfiguration::default();
    let tracker = k4a::Tracker::create(&calibration, tracker_configuration)
        .expect("failed to create tracker");
    let mut packet = vec![0u8; 8 + 260 + 1024];
    packet[..9].copy_from_slice(b"/joints\0,");
    for ofs in (0..32).map(|i| 9 + i * 8) {
        packet[ofs..ofs + 8].copy_from_slice(b"ifffffff");
    }
    packet[265..268].copy_from_slice(b"\0\0\0");
    loop {
        if !is_running.load(Ordering::SeqCst) {
            break;
        }
        let payload = &mut packet[268..];

        let capture = match sensor.get_capture(0) {
            Ok(capture) => capture,
            Err(active_sensor::Error::Timeout) => continue,
            Err(active_sensor::Error::End) => break,
            Err(active_sensor::Error::Fatal) => panic!("failed to capture"),
        };
        tracker.enqueue_capture(capture, 0).ok();
        let frame = match tracker.k4abt_tracker_pop_result(0) {
            Ok(frame) => frame,
            Err(k4a::WaitError::Timeout) => continue,
            Err(e) => panic!("failed to pop frame: {:?}", e),
        };
        let num_bodies = frame.get_num_bodies();
        for index in 0..num_bodies {
            if index != 0 {
                continue;
            }
            //let id = frame.get_body_id(index);
            let skeleton: k4a::Skeleton = frame
                .get_body_skeleton(index)
                .expect("failed to get skelton from frame");
            for (joint_index, joint) in (&skeleton.joints).iter().enumerate() {
                let ofs = joint_index * 32;
                let slice = &mut payload[ofs..ofs + 32];
                slice[0..4].copy_from_slice(&joint.confidence_level.0.to_be_bytes());
                slice[4..8].copy_from_slice(&joint.position.x.to_be_bytes());
                slice[8..12].copy_from_slice(&joint.position.y.to_be_bytes());
                slice[12..16].copy_from_slice(&joint.position.z.to_be_bytes());
                slice[16..20].copy_from_slice(&joint.orientation.w.to_be_bytes());
                slice[20..24].copy_from_slice(&joint.orientation.x.to_be_bytes());
                slice[24..28].copy_from_slice(&joint.orientation.y.to_be_bytes());
                slice[28..32].copy_from_slice(&joint.orientation.z.to_be_bytes());
                //println!("{:02}.{:02} P{{{:?}}}", id, joint_index, joint.position);
                //println!("      O{{{:?}}}", joint.orientation);
            }
        }
        socket
            .send_to(&packet, target_addr)
            .expect("failed to send");
    }
}

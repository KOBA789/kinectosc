use super::active_sensor::{self, ActiveSensor};

pub struct Kinect {
    sensor: Box<dyn ActiveSensor>,
    tracker: k4a::Tracker,
}

impl Kinect {
    pub fn open_sensor(
        device_index: u32,
        sensor_config: k4a::DeviceConfiguration,
        tracker_config: k4a::TrackerConfiguration
    ) -> Result<Self, k4a::Error> {
        let device = k4a::Device::open(device_index)?;
        let running = device
            .start_cameras(sensor_config)?;
        let calibration = running.get_calibration()?;
        let tracker = k4a::Tracker::create(&calibration, tracker_config)?;
        Ok(Self {
            sensor: Box::new(running),
            tracker,
        })
    }

    pub fn next_frame(&self) -> Result<k4a::Frame, active_sensor::Error> {
        let capture = self.sensor.get_capture(-1)?;
        self.tracker.enqueue_capture(capture, -1)?;
        Ok(self.tracker.k4abt_tracker_pop_result(-1)?)
    }
}

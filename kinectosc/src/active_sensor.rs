use k4a::{Playback, Capture, StreamError, RunningDevice, WaitError, Calibration};
use std::time;
use std::cell::Cell;

pub enum Error {
    End,
    Timeout,
    Fatal,
}

pub trait ActiveSensor {
    fn get_capture(&self, timeout: i32) -> Result<Capture, Error>;
    fn get_calibration(&self) -> Result<Calibration, k4a::Error>;
}

pub struct RealtimePlayback {
    playback: Playback,
    interval: time::Duration,
    last_time: Cell<time::Instant>,
}

impl RealtimePlayback {
    pub fn new(playback: Playback, fps: u32) -> Self {
        let last_time = Cell::new(time::Instant::now());
        let interval = time::Duration::from_secs(1) / fps;
        RealtimePlayback {
            playback,
            interval,
            last_time,
        }
    }
}

impl ActiveSensor for RealtimePlayback {
    fn get_capture(&self, _timeout: i32) -> Result<Capture, Error> {
        let start_time = time::Instant::now();
        let last_time = self.last_time.get();
        let elapsed = start_time - last_time;
        if self.interval > elapsed {
            std::thread::sleep(self.interval - elapsed);
        }
        let r = ActiveSensor::get_capture(&self.playback, 0);
        self.last_time.set(time::Instant::now());
        r
    }
    
    fn get_calibration(&self) -> Result<Calibration, k4a::Error> {
        ActiveSensor::get_calibration(&self.playback)
    }
}

impl ActiveSensor for Playback {
    fn get_capture(&self, _timeout: i32) -> Result<Capture, Error> {
        match self.get_capture() {
            Ok(capture) => Ok(capture),
            Err(StreamError::Eof) => Err(Error::End),
            Err(StreamError::Failed) => Err(Error::Fatal),
        }
    }
    
    fn get_calibration(&self) -> Result<Calibration, k4a::Error> {
        self.get_calibration()
    }    
}

impl ActiveSensor for RunningDevice {
    fn get_capture(&self, timeout: i32) -> Result<Capture, Error> {
        match self.get_capture(timeout) {
            Ok(capture) => Ok(capture),
            Err(WaitError::Timeout) => Err(Error::Timeout),
            Err(WaitError::Failed) => Err(Error::Fatal),
        }
    }

    fn get_calibration(&self) -> Result<Calibration, k4a::Error> {
        self.get_calibration()
    }
}

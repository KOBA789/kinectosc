use k4a::{Playback, Capture, StreamError, RunningDevice, WaitError, Calibration};
use std::time;
use std::cell::Cell;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    End,
    Timeout,
    Fatal,
}

impl From<k4a::Error> for Error {
    fn from(k4a_err: k4a::Error) -> Self {
        match k4a_err {
            k4a::Error::Failed => Error::Fatal,
        }
    }
}

impl From<WaitError> for Error {
    fn from(wait_err: WaitError) -> Self {
        match wait_err {
            WaitError::Timeout => Error::Timeout,
            WaitError::Failed => Error::Fatal,
        }
    }
}

impl From<StreamError> for Error {
    fn from(stream_err: StreamError) -> Self {
        match stream_err {
            StreamError::Failed => Error::Fatal,
            StreamError::Eof => Error::End,
        }    
    }
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
        Ok(self.get_capture()?)
    }
    
    fn get_calibration(&self) -> Result<Calibration, k4a::Error> {
        self.get_calibration()
    }    
}

impl ActiveSensor for RunningDevice {
    fn get_capture(&self, timeout: i32) -> Result<Capture, Error> {
        Ok(self.get_capture(timeout)?)
    }

    fn get_calibration(&self) -> Result<Calibration, k4a::Error> {
        self.get_calibration()
    }
}

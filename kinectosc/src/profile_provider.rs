use std::path::PathBuf;
use std::sync::mpsc;
use std::time;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent, ReadDirectoryChangesWatcher};

use super::calibration;
use nalgebra::{Translation3, UnitQuaternion};

pub struct ProfileProvider {
    path: PathBuf,
    watcher: ReadDirectoryChangesWatcher,
    rx: mpsc::Receiver<DebouncedEvent>,
    pub wfd_rotation: UnitQuaternion<f64>,
    pub wfd_translation: Translation3<f64>,
}

impl ProfileProvider {
    pub fn new(path: PathBuf) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut watcher = watcher(tx, time::Duration::from_secs(5)).unwrap();
        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
        let profile = calibration::load(&path).unwrap();
        let wfd_rotation = profile.wfd_rotation();
        let wfd_translation = profile.wfd_translation();
        Self { path, watcher, rx, wfd_rotation, wfd_translation }
    }

    pub fn reload_if_updated(&mut self) {
        let events = self.rx.try_iter().count();
        if events > 0 {
            match calibration::load(&self.path) {
                Ok(profile) => {
                    self.wfd_rotation = profile.wfd_rotation();
                    self.wfd_translation = profile.wfd_translation();
                    eprintln!("Profile was reloaded");
                },
                Err(e) => {
                    eprintln!("Could not reload profile: {}", e);
                },
            }
        }
    }
}

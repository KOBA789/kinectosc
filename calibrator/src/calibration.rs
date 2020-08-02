use std::{fs, path, io};

use serde::{Serialize, Deserialize};
use nalgebra::{Translation3, UnitQuaternion, Quaternion};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    translation: Translation,
    rotation: Rotation,
}

impl Profile {
    pub fn new(wfd_translation: Translation3<f64>, wfd_rotation: UnitQuaternion<f64>) -> Self {
        Self {
            translation: wfd_translation.into(),
            rotation: wfd_rotation.into(),
        }
    }

    pub fn wfd_translation(&self) -> Translation3<f64> {
        self.translation.clone().into()
    }

    pub fn wfd_rotation(&self) -> UnitQuaternion<f64> {
        UnitQuaternion::from_quaternion(self.rotation.clone().into())
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new(Translation3::identity(), UnitQuaternion::identity())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rotation {
    w: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl From<Translation> for Translation3<f64> {
    fn from(transl: Translation) -> Self {
        Translation3::new(transl.x, transl.y, transl.z)
    }
}

impl From<Rotation> for Quaternion<f64> {
    fn from(rot: Rotation) -> Self {
        Quaternion::new(rot.w, rot.i, rot.j, rot.k)
    }
}

impl From<Translation3<f64>> for Translation {
    fn from(transl3: Translation3<f64>) -> Self {
        Translation {
            x: transl3.x,
            y: transl3.y,
            z: transl3.z,
        }
    }
}

impl From<UnitQuaternion<f64>> for Rotation {
    fn from(unitq: UnitQuaternion<f64>) -> Self {
        Rotation {
            w: unitq.w,
            i: unitq.i,
            j: unitq.j,
            k: unitq.k,
        }    
    }
}

pub fn load<P: AsRef<path::Path>>(path: P) -> io::Result<Profile> {
    let file = fs::File::open(path)?;
    let profile = serde_json::from_reader(file)?;
    Ok(profile)
}

pub fn save<P: AsRef<path::Path>>(path: P, profile: &Profile) -> io::Result<()> {
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, profile)?;
    Ok(())
}

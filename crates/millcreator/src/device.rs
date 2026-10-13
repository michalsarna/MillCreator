//! A device: the milling machine a project is made for. It holds the size of the work area, the spindle, the speeds,
//! the safe height and the cameras. The devices are kept in one file; a device can be exported to a file and imported.
use base_core::camimg::{CameraKind, CameraSource};
use base_core::units::Units;
use serde::{Deserialize, Serialize};

/// The name of the stand-in device for working without a machine. It is the same in every language, so the one device is
/// found again after the language is changed.
pub const DUMMY_NAME: &str = "Dummy";
pub const MAX_DEVICES: usize = 200;
pub const MAX_FILE: u64 = 2_000_000;
pub const FILE_FORMAT: u32 = 1;

/// The travel of the machine: X and Y are the work area, Z is how far the spindle goes down from the top (mm).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spindle {
    pub min_rpm: f64,
    pub max_rpm: f64,
    /// Seconds to wait after the spindle is switched on, before the cut starts.
    pub spin_up: f64,
}

/// The corner of the work area that is the machine zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

impl Origin {
    pub const ALL: [Origin; 4] = [Origin::FrontLeft, Origin::FrontRight, Origin::BackLeft, Origin::BackRight];
    pub fn label(self) -> &'static str {
        match self {
            Origin::FrontLeft => "Front left",
            Origin::FrontRight => "Front right",
            Origin::BackLeft => "Back left",
            Origin::BackRight => "Back right",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub units: Units,
    pub area: Area,
    pub origin: Origin,
    /// The height above the top of the material where the spindle moves between cuts (mm).
    pub safe_z: f64,
    /// The speed of the moves above the material (mm/min).
    pub rapid: f64,
    pub spindle: Spindle,
    #[serde(default)]
    pub cameras: Vec<CameraSource>,
    #[serde(default)]
    pub active_camera: usize,
    #[serde(default)]
    pub notes: String,
}

impl Default for Device {
    /// The plain defaults: a small machine, a safe height of 5 mm.
    fn default() -> Device {
        Device {
            name: String::new(),
            units: Units::Mm,
            area: Area { x: 300.0, y: 200.0, z: 60.0 },
            origin: Origin::FrontLeft,
            safe_z: 5.0,
            rapid: 3000.0,
            spindle: Spindle { min_rpm: 1000.0, max_rpm: 24000.0, spin_up: 2.0 },
            cameras: vec![],
            active_camera: 0,
            notes: String::new(),
        }
    }
}

impl Device {
    /// The stand-in device: the defaults under the name Dummy.
    pub fn dummy() -> Device {
        Device { name: DUMMY_NAME.to_string(), ..Device::default() }
    }

    pub fn is_dummy(&self) -> bool {
        self.name == DUMMY_NAME
    }

    pub fn camera(&self) -> Option<&CameraSource> {
        self.cameras.get(self.active_camera).filter(|c| c.enabled && c.is_valid())
    }

    /// Are the numbers usable? Used for what is read from a file and before a device is saved.
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && v >= lo && v <= hi;
        if self.name.trim().is_empty() || self.name.len() > 120 {
            return Err("the name is empty or too long".into());
        }
        if !ok(self.area.x, 10.0, 10_000.0) || !ok(self.area.y, 10.0, 10_000.0) || !ok(self.area.z, 1.0, 2_000.0) {
            return Err("the work area is out of range".into());
        }
        if !ok(self.safe_z, 0.1, self.area.z) {
            return Err("the safe height is out of range".into());
        }
        if !ok(self.rapid, 1.0, 100_000.0) {
            return Err("the speed of the moves is out of range".into());
        }
        let s = &self.spindle;
        if !ok(s.min_rpm, 0.0, 1_000_000.0) || !ok(s.max_rpm, 1.0, 1_000_000.0) || s.min_rpm > s.max_rpm || !ok(s.spin_up, 0.0, 600.0) {
            return Err("the spindle speeds are out of range".into());
        }
        if self.cameras.len() > 8 || self.active_camera > 8 || self.notes.len() > 20_000 {
            return Err("too many cameras or too long notes".into());
        }
        for c in &self.cameras {
            if c.name.len() > 120 || c.rotation > 3 {
                return Err("a camera is not valid".into());
            }
            if let CameraKind::Url(u) = &c.kind {
                if !u.is_empty() && !base_core::camimg::valid_url(u) {
                    return Err("a camera address is not an http or https address".into());
                }
            }
        }
        Ok(())
    }

    /// A free name: the wanted one, or with a number after it.
    pub fn free_name(wanted: &str, taken: &[String]) -> String {
        if !taken.iter().any(|t| t == wanted) {
            return wanted.to_string();
        }
        (2..).map(|n| format!("{wanted} ({n})")).find(|n| !taken.contains(n)).unwrap_or_else(|| wanted.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct OneFile {
    format: u32,
    device: Device,
}

#[derive(Serialize, Deserialize)]
struct ManyFile {
    format: u32,
    devices: Vec<Device>,
}

/// One device as a file.
pub fn export(d: &Device) -> String {
    serde_json::to_string_pretty(&OneFile { format: FILE_FORMAT, device: d.clone() }).unwrap_or_default()
}

/// A device from a file. It is checked strictly; a newer format is refused.
pub fn import(text: &str) -> Result<Device, String> {
    if text.len() as u64 > MAX_FILE {
        return Err("the file is too large".into());
    }
    let f: OneFile = serde_json::from_str(text).map_err(|e| format!("this is not a device file: {e}"))?;
    if f.format > FILE_FORMAT {
        return Err("the file is from a newer version of the program".into());
    }
    f.device.check()?;
    Ok(f.device)
}

/// All devices as they are kept between runs.
pub fn save_all(devices: &[Device]) -> String {
    serde_json::to_string_pretty(&ManyFile { format: FILE_FORMAT, devices: devices.to_vec() }).unwrap_or_default()
}

/// The devices that were kept. A device that does not pass the check is left out.
pub fn load_all(text: &str) -> Vec<Device> {
    match serde_json::from_str::<ManyFile>(text) {
        Ok(f) if f.format <= FILE_FORMAT => f.devices.into_iter().take(MAX_DEVICES).filter(|d| d.check().is_ok()).collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_and_the_dummy_are_valid() {
        assert!(Device::dummy().check().is_ok());
        assert!(Device::dummy().is_dummy());
        assert!(Device::default().check().is_err(), "a device needs a name");
    }

    #[test]
    fn wrong_numbers_are_refused() {
        let mut d = Device::dummy();
        d.area.x = f64::NAN;
        assert!(d.check().is_err());
        let mut d = Device::dummy();
        d.safe_z = 500.0;
        assert!(d.check().is_err(), "safe height above the travel");
        let mut d = Device::dummy();
        d.spindle.min_rpm = 30000.0;
        assert!(d.check().is_err());
        let mut d = Device::dummy();
        d.cameras.push(CameraSource::url("c", "ftp://x", 0));
        assert!(d.check().is_err());
        d.cameras[0] = CameraSource::url("c", "http://pi.local:8080/s.mjpg", 1);
        assert!(d.check().is_ok());
    }

    #[test]
    fn a_device_goes_to_a_file_and_back_and_a_bad_file_is_refused() {
        let mut d = Device::dummy();
        d.name = "My router".into();
        d.area.z = 80.0;
        d.cameras.push(CameraSource::url("top", "http://pi.local/s.mjpg", 2));
        let back = import(&export(&d)).unwrap();
        assert_eq!(back, d);
        assert!(import("nonsense").is_err());
        assert!(import(r#"{"format": 9, "device": {}}"#).is_err());
        let mut bad = d.clone();
        bad.area.y = -1.0;
        assert!(import(&export(&bad)).is_err());
    }

    #[test]
    fn the_kept_devices_come_back_and_a_damaged_file_gives_none() {
        let mut a = Device::dummy();
        a.name = "A".into();
        let all = vec![a.clone(), Device::dummy()];
        assert_eq!(load_all(&save_all(&all)), all);
        assert!(load_all("{ broken").is_empty());
        // A device with wrong numbers is left out, the rest stay.
        let mut bad = a.clone();
        bad.area.x = 0.0;
        assert_eq!(load_all(&save_all(&[bad, a.clone()])), vec![a]);
    }

    #[test]
    fn a_free_name_gets_a_number() {
        let taken = vec!["A".to_string(), "A (2)".to_string()];
        assert_eq!(Device::free_name("B", &taken), "B");
        assert_eq!(Device::free_name("A", &taken), "A (3)");
    }
}

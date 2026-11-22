use crate::error::*;

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub limits: Limits,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Limits {
    pub max_image_dimension_2d: u32,
}

pub struct Device {
    pub info: DeviceInfo,
}

impl Device {
    pub fn new() -> Self {
        Self {
            info: DeviceInfo {
                name: "Null Device".into(),
                limits: Limits { max_image_dimension_2d: 16384 },
            }
        }
    }
    pub fn wait_idle(&self) -> Result<()> { Ok(()) }
}

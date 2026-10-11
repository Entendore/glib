use crate::device::DeviceInfo;
use crate::error::Result;

pub struct NullBackend;

impl NullBackend {
    pub fn enumerate() -> Result<Vec<DeviceInfo>> {
        Ok(vec![DeviceInfo {
            name: "Null CI Device".into(),
            limits: crate::device::Limits::default(),
        }])
    }
}

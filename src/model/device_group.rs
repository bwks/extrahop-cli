use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct DeviceGroups {
    pub device_groups: Vec<DeviceGroup>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct DeviceGroup {
    #[serde(deserialize_with = "null_to_default")]
    pub built_in: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub dynamic: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub field: String,
    // #[serde(deserialize_with = "null_to_default")]
    // pub filter: {},
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub include_custom_devices: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub value: String,
}

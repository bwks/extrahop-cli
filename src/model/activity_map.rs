use std::collections::HashMap;

use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Deserialize)]
pub struct ActivityMaps {
    pub activity_maps: Vec<ActivityMap>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ActivityMap {
    #[serde(deserialize_with = "null_to_default")]
    pub description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub mode: String,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub owner: String,
    pub rights: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub short_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub show_alert_status: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub weighting: String,
    // TODO: Find variants for walks
    // #[serde(deserialize_with = "null_to_default")]
    // #[serde(default)]
    // pub walks: Vec<Walks>,
    pub walks: Vec<HashMap<String, serde_json::Value>>,
}

impl Tabled for ActivityMap {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let mut walks = vec![];
        for w in self.walks.iter() {
            for (k, v) in w {
                walks.push(format!("{}: {}", k, v));
            }
        }
        vec![
            self.description.clone(),
            self.id.to_string(),
            self.mod_time.to_string(),
            self.mode.clone(),
            self.name.clone(),
            self.owner.clone(),
            self.rights.join("\n"),
            self.short_code.clone(),
            self.show_alert_status.to_string(),
            self.weighting.clone(),
            walks.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "description".to_string(),
            "id".to_string(),
            "mod_time".to_string(),
            "mode".to_string(),
            "name".to_string(),
            "owner".to_string(),
            "rights".to_string(),
            "short_code".to_string(),
            "show_alert_status".to_string(),
            "weighting".to_string(),
            "walks".to_string(),
        ]
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Walks {
    pub origins: Vec<ObjectIdType>,
    pub steps: Vec<Steps>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ObjectIdType {
    #[serde(deserialize_with = "null_to_default")]
    pub object_id: u64,
    #[serde(deserialize_with = "null_to_default")]
    pub object_type: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ProtocolRole {
    #[serde(deserialize_with = "null_to_default")]
    pub protocol: String,
    #[serde(deserialize_with = "null_to_default")]
    pub role: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Steps {
    pub peer_in: Vec<ObjectIdType>,
    pub peer_not_in: Vec<ObjectIdType>,
    pub relationships: Vec<ProtocolRole>,
}

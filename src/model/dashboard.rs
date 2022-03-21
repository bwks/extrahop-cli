use serde::Deserialize;
use tabled::Tabled;

use crate::model::null_to_default;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Dashboard {
    #[serde(deserialize_with = "null_to_default")]
    pub author: String,
    #[serde(deserialize_with = "null_to_default")]
    pub comment: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub mod_time: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub owner: String,
    #[serde(deserialize_with = "null_to_default")]
    pub rights: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub short_code: String,
    #[serde(deserialize_with = "null_to_default")]
    #[serde(rename = "type")]
    pub _type: String,
}

impl Tabled for Dashboard {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        vec![
            format!("{}", self.author),
            format!("{}", self.comment),
            format!("{}", self.id),
            format!("{}", self.mod_time),
            format!("{}", self.name),
            format!("{}", self.owner),
            self.rights.join(", "),
            format!("{}", self.short_code),
            format!("{}", self._type),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            String::from("author"),
            String::from("comment"),
            String::from("id"),
            String::from("mod_time"),
            String::from("name"),
            String::from("owner"),
            String::from("rights"),
            String::from("short_code"),
            String::from("type"),
        ]
    }
}

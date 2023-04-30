use restson::RestPath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PingMeta {
    id: String,
    #[serde(alias = "statusEmoji")]
    status_emoji: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    meta: PingMeta,
}

impl Ping {
    pub fn status(&self) -> &str {
        &self.meta.status_emoji
    }
}

impl RestPath<()> for Ping {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("util/ping"))
    }
}

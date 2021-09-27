use serde::{Deserialize, Serialize};
use restson::{RestPath};

use crate::general::{Pagination, TransactionsLinks};

#[derive(Serialize, Debug)]
pub struct TagId {
    id:         String
}

impl TagId {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<'de> Deserialize<'de> for TagId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(TagId {
            id:     s
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagRelationships {
    pub transactions:   TransactionsLinks
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagData {
    pub id:                 TagId,
    pub relationships:      TagRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagListResponse {
    pub data:   Vec<TagData>,
    pub links:  Pagination<TagListResponse>
}

impl RestPath<()> for TagListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("tags"))
    }
}


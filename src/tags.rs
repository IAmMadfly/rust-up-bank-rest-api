use serde::{Deserialize, Serialize};
use restson::{RestPath};

use crate::general::{Pagination, TransactionsLinks};

#[derive(Serialize, Debug)]
pub struct TagId {
    id:         String
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
    transactions:   TransactionsLinks
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagData {
    id:                 TagId,
    relationships:      TagRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagListResponse {
    data:   Vec<TagData>,
    links:  Pagination<TagListResponse>
}

impl RestPath<()> for TagListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("tags"))
    }
}


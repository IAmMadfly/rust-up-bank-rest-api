use serde::{Deserialize, Serialize};
use restson::{RestPath};

use crate::{accounts::AccountId, categories::CategoryRelationship, general::{CashbackObject, MoneyObject, PageLink, Pagination, RelationLink, RelationshipsLink, RoundUp, TimeObject}, tags::TagId};

#[derive(Serialize, Debug)]
pub struct TransactionId {
    id:             String
}

impl TransactionId {
    pub fn new(id: &str) -> TransactionId {
        TransactionId {
            id:     id.to_string()
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<'de> Deserialize<'de> for TransactionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(TransactionId {
            id:     s
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionHoldInfo {
    pub amount:         MoneyObject,
    #[serde(alias = "foreignAmount")]
    pub foreign_amount: MoneyObject
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionAttributes {
    pub status:             String,
    #[serde(alias = "rawText")]
    pub raw_text:           Option<String>,
    pub description:        String,
    pub message:            Option<String>,
    #[serde(alias = "holdAmount")]
    pub hold_info:          Option<TransactionHoldInfo>,
    #[serde(alias = "roundUp")]
    pub round_up:           Option<RoundUp>,
    pub cashback:           Option<CashbackObject>,
    pub amount:             MoneyObject,
    #[serde(alias = "foreignAmount")]
    pub foreign_amount:     Option<MoneyObject>,
    #[serde(alias = "settledAt")]
    pub settled_at:         Option<TimeObject>,
    #[serde(alias = "createdAt")]
    pub created_at:         TimeObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRelationship {
    pub id:                 AccountId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountObject {
    pub data:               Option<AccountRelationship>,
    pub links:              Option<RelationshipsLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryObject {
    pub data:               Option<CategoryRelationship>,
    pub links:              Option<RelationshipsLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTagsData {
    pub id:                 TagId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTagLink {
    #[serde(alias = "self")]
    pub self_link:          RelationLink
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  TransactionTags {
    pub data:               Vec<TransactionTagsData>,
    pub links:              Option<TransactionTagLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRelationships {
    pub account:            AccountObject,
    #[serde(alias = "transferAccount")]
    pub transfer_account:   Option<AccountObject>,
    pub category:           CategoryObject,
    #[serde(alias = "parentCategory")]
    pub parent_category:    CategoryObject,
    pub tags:               TransactionTags
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub id:                 TransactionId,
    pub attributes:         TransactionAttributes,
    pub relationships:      TransactionRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionListResponse {
    pub data:       Vec<TransactionData>,
    pub links:  Pagination<TransactionListResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub data:       TransactionData
}

impl RestPath<()> for TransactionListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("transactions"))
    }
}

impl RestPath<&PageLink<TransactionListResponse>> for TransactionListResponse {
    fn get_path(link: &PageLink<TransactionListResponse>) -> Result<String, restson::Error> {
        let url = String::from("transactions?") + &link.params;
        println!("Link: {}", url);
        Ok(url)
    }
}

impl RestPath<TransactionId> for TransactionResponse {
    fn get_path(id: TransactionId) -> Result<String, restson::Error> {
        Ok(String::from("transactions/") + &id.id)
    }
}


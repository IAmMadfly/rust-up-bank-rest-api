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
    amount:         MoneyObject,
    #[serde(alias = "foreignAmount")]
    foreign_amount: MoneyObject
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionAttributes {
    status:             String,
    #[serde(alias = "rawText")]
    raw_text:           Option<String>,
    description:        String,
    message:            Option<String>,
    #[serde(alias = "holdAmount")]
    hold_info:          Option<TransactionHoldInfo>,
    #[serde(alias = "roundUp")]
    round_up:           Option<RoundUp>,
    cashback:           Option<CashbackObject>,
    amount:             MoneyObject,
    #[serde(alias = "foreignAmount")]
    foreign_amount:     Option<MoneyObject>,
    #[serde(alias = "settledAt")]
    settled_at:         Option<TimeObject>,
    #[serde(alias = "createdAt")]
    created_at:         TimeObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRelationship {
    id:                 AccountId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountObject {
    data:               Option<AccountRelationship>,
    links:              Option<RelationshipsLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryObject {
    data:               Option<CategoryRelationship>,
    links:              Option<RelationshipsLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTagsData {
    id:                 TagId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTagLink {
    #[serde(alias = "self")]
    self_link:          RelationLink
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  TransactionTags {
    data:               Vec<TransactionTagsData>,
    links:              Option<TransactionTagLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRelationships {
    account:            AccountObject,
    #[serde(alias = "transferAccount")]
    transfer_account:   Option<AccountObject>,
    category:           CategoryObject,
    #[serde(alias = "parentCategory")]
    parent_category:    CategoryObject,
    tags:               TransactionTags
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    id:                 TransactionId,
    attributes:         TransactionAttributes,
    relationships:      TransactionRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionListResponse {
    data:       Vec<TransactionData>,
    pub links:  Pagination<TransactionListResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    data:       TransactionData
}

impl RestPath<()> for TransactionListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("transactions"))
    }
}

impl RestPath<PageLink<TransactionListResponse>> for TransactionListResponse {
    fn get_path(link: PageLink<TransactionListResponse>) -> Result<String, restson::Error> {
        let url = String::from("transactions") + &link.link;
        println!("Link: {}", url);
        Ok(url)
    }
}

impl RestPath<TransactionId> for TransactionResponse {
    fn get_path(id: TransactionId) -> Result<String, restson::Error> {
        Ok(String::from("transactions/") + &id.id)
    }
}


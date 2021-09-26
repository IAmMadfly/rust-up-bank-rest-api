use serde::{Deserialize, Serialize};
use restson::{RestPath};

use crate::general::{Pagination, MoneyObject, TransactionsLinks};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAttributes {
    #[serde(alias = "displayName")]
    display_name:   String,
    #[serde(alias = "accountType")]
    account_type:   String,
    balance:        MoneyObject,
    #[serde(alias = "createdAt")]
    created_at:     String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRelationships {
    transactions:   TransactionsLinks
}

#[derive(Serialize, Debug)]
pub struct AccountId {
    id:             String
}

impl AccountId {
    pub fn new(id: &str) -> AccountId {
        AccountId {
            id:     id.to_string()
        }
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(AccountId {
            id:     s
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    id:             AccountId,
    attributes:     AccountAttributes,
    relationships:  AccountRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsListResponse {
    data:   Vec<AccountData>,
    links:  Pagination<AccountsListResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    data:   AccountData
}

impl RestPath<()> for AccountsListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("accounts"))
    }
}

impl RestPath<AccountId> for AccountResponse {
    fn get_path(id: AccountId) -> Result<String, restson::Error> {
        Ok(String::from("accounts/") + &id.id)
    }
}

// impl RestPath<AccountId> for AccountsListResponse {
//     fn get_path(query: AccountId) -> Result<String, restson::Error> {
//         Ok(String::from("accounts")+"?"+)
//     }
// }


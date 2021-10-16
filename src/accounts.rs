use serde::{Deserialize, Serialize};
use restson::{RestPath};

use crate::general::{Pagination, MoneyObject, TransactionsLinks};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAttributes {
    #[serde(alias = "displayName")]
    pub display_name:   String,
    #[serde(alias = "accountType")]
    pub account_type:   String,
    pub balance:        MoneyObject,
    #[serde(alias = "createdAt")]
    pub created_at:     String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRelationships {
    pub transactions:   TransactionsLinks
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

    pub fn id(&self) -> &str {
        &self.id
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
    pub id:             AccountId,
    pub attributes:     AccountAttributes,
    pub relationships:  AccountRelationships
}

/// Can be used to get the Accounts list, with the use of a RestClient
///
/// Example:
/// ```
/// let mut client = get_new_blocking_client(String::from("Bearer up:demo:rtHR7D3eBEqKPiIT"))
///     .unwrap();
/// let accounts: AccountsListResponse = client.get(()).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsListResponse {
    pub data:   Vec<AccountData>,
    pub links:  Pagination<AccountsListResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub data:   AccountData
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


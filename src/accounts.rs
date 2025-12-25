use restson::RestPath;
use serde::{Deserialize, Serialize};

use crate::general::{MoneyObject, Pagination, TransactionsLinks};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AccountType {
    #[serde(alias = "saver")]
    SAVER,
    #[serde(alias = "transactional")]
    TRANSACTIONAL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountAttributes {
    #[serde(alias = "displayName")]
    pub display_name: String,
    #[serde(alias = "accountType")]
    pub account_type: AccountType,
    pub balance: MoneyObject,
    #[serde(alias = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRelationships {
    pub transactions: TransactionsLinks,
}

#[derive(Serialize, Debug, Clone)]
pub struct AccountId {
    pub id: String,
}

impl AccountId {
    pub fn new(id: &str) -> AccountId {
        AccountId { id: id.to_string() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(AccountId { id: s })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountData {
    pub id: AccountId,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
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
    pub data: Vec<AccountData>,
    pub links: Pagination<AccountsListResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub data: AccountData,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type_parses_from_uppercase() {
        let saver_json = r#""SAVER""#;
        let transactional_json = r#""TRANSACTIONAL""#;

        let saver: AccountType = serde_json::from_str(saver_json).unwrap();
        let transactional: AccountType = serde_json::from_str(transactional_json).unwrap();

        match saver {
            AccountType::SAVER => (),
            _ => panic!("Expected SAVER variant"),
        }

        match transactional {
            AccountType::TRANSACTIONAL => (),
            _ => panic!("Expected TRANSACTIONAL variant"),
        }
    }

    #[test]
    fn test_account_type_parses_from_lowercase() {
        let saver_json = r#""saver""#;
        let transactional_json = r#""transactional""#;

        let saver: Result<AccountType, _> = serde_json::from_str(saver_json);
        let transactional: Result<AccountType, _> = serde_json::from_str(transactional_json);

        assert!(saver.is_ok(), "Should parse 'saver' successfully");
        assert!(
            transactional.is_ok(),
            "Should parse 'transactional' successfully"
        );

        match saver.unwrap() {
            AccountType::SAVER => (),
            _ => panic!("Expected SAVER variant"),
        }

        match transactional.unwrap() {
            AccountType::TRANSACTIONAL => (),
            _ => panic!("Expected TRANSACTIONAL variant"),
        }
    }
}

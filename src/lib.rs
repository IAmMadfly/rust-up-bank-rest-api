

//! This library is to be used to simplify the access of Up Bank data through
//! Rust applications.

use std::fs;
use std::path::Path;
use std::env;

use toml;
pub use restson::{
    RestClient,
    blocking,
    Error
};
use serde::{
    Deserialize,
    // Serialize
};

/// Module for all Transaction related data
pub mod transactions;
/// Module for all Account related data
pub mod accounts;
/// Module for all Category related data
pub mod categories;
/// Module for all Tag related data
pub mod tags;
/// Module for elements which are not to any specific data type, such as `Pagination<T>`
pub mod general;

mod test;

#[derive(Deserialize)]
pub struct Config {
    token: String
}

/// Pass in your Up Bank token, returns an *async* RestClient where
/// the API can be easily called, with the required results returned.
///
/// Example:
///
/// ```
/// if let Ok(client) = get_new_client(String::from("Bearer up:demo:rtHR7D3eBEqKPiIT")) {
///     let accounts: AccountListsResponse = client.get(()).await.unwrap();
/// }
/// ```
pub fn get_new_client(token: String) -> Result<RestClient, Error> {
    let key= format!("Bearer  {}", token);
    let mut client = RestClient::new("https://api.up.com.au/api/v1/")?;
    client.set_header("Authorization", &key)?;
    Ok(client)
}

/// Pass in your Up Bank token, returns a *blocking* RestClient where
/// the API can be easily called, with the required results returned.
///
/// Example:
///
/// ```
/// if let Ok(client) = get_new_blocking_client(String::from("Bearer up:demo:rtHR7D3eBEqKPiIT")) {
///     let accounts: AccountListsResponse = client.get(()).unwrap();
/// }
/// ```
pub fn get_new_blocking_client(token: String) -> Result<blocking::RestClient, Error> {
    let key= format!("Bearer  {}", token);
    let mut client = RestClient::new_blocking("https://api.up.com.au/api/v1/")?;
    client.set_header("Authorization", &key)?;
    Ok(client)
}


// use crate::accounts::{AccountId, AccountResponse, AccountsListResponse};
// use crate::categories::{CategoriesListResponse, CategoryResponse, CategoryId};
// use crate::tags::{TagListResponse};
// use crate::transactions::{TransactionId, TransactionListResponse, TransactionResponse};

// fn main() {
//     let mut client = RestClient::new_blocking("https://api.up.com.au/api/v1/").unwrap();

//     let key= format!("Bearer  {}", "up:yeah:key-goes-here");
//     client.set_header("Authorization", &key).unwrap();

//     let _accounts: AccountsListResponse = client.get(()).unwrap();
//     let _account: AccountResponse = client.get(
//         AccountId::new("49908555-f6e9-42fa-9a02-8de302aecb51")
//     ).unwrap();

//     let _categories: CategoriesListResponse = client.get(()).unwrap();
//     let _category: CategoryResponse = client.get(
//         CategoryId::new("home")
//     ).unwrap();

//     let _tags: TagListResponse = client.get(()).unwrap();

//     let _transactions: TransactionListResponse = client.get(()).unwrap();
//     let _transaction: TransactionResponse = client
//         .get(TransactionId::new("f45ca050-3d44-40bd-9c4c-a24e05fdfe8d")).unwrap();

//     // if let Some(link) = &_transactions.links.next {
//     //     println!("Getting next page");
//     //     let page = link.get(&mut client).unwrap();
//     //     println!("Paged data: {:?}", page);
//     // };

//     if let Some(new_link) = _transactions.links.next.clone() {
//         let mut curr_link = Some(new_link);
//         let mut page_number = 1u32;

//         loop {
//             println!("Page number: {}", page_number);
//             page_number += 1;
//             if let Some(link) = &curr_link {
//                 println!("Asking for data");
//                 let data = link.get_blocking(&mut client).unwrap();
//                 println!("Got data");
//                 curr_link = data.links.next;
//             } else {
//                 break;
//             }
//         }
//     }

// }

pub fn get_token() -> String {
    let test_token_path = "./test.toml";
    let config: Config;

    // Read Up Authentication token from environment variable
    if let Ok(token) = env::var("UP_TOKEN") {
        config = Config { token };
    }
    // Else read from test file
    else if Path::new(test_token_path).exists() {
        config = toml::from_str(
            &fs::read_to_string(
                test_token_path
            )
            .expect("Failed to read config file")
        ).expect("Failed to parse into config");
    }
    // Else error as no authentication token can be found
    else {
        panic!("No authentication token has been provided, please set UP_TOKEN env variable or consult documentation.")
    }

    config.token
}


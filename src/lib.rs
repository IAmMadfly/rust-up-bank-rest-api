//! This library is to be used to simplify the access of Up Bank data through
//! Rust applications.

// use serde::{Deserialize, Serialize};
pub use restson::{self, blocking, Error, RestClient};

/// Module for all Account related data
pub mod accounts;
/// Module for all Category related data
pub mod categories;
/// Module for all Tag related data
pub mod tags;
/// Module for all Transaction related data
pub mod transactions;

/// Module for elements which are not to any specific data type, such as `Pagination<T>`
pub mod general;
pub mod utility;

mod client;
mod test;

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
    let key = format!("Bearer  {}", token);
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
    let key = format!("Bearer  {}", token);
    let mut client = RestClient::new_blocking("https://api.up.com.au/api/v1/")?;
    client.set_header("Authorization", &key)?;
    Ok(client)
}

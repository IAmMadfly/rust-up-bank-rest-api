
// use serde::{Deserialize, Serialize};
use restson::{RestClient};

mod transactions;
mod accounts;
mod categories;
mod tags;
mod general;

use crate::accounts::{AccountId, AccountResponse, AccountsListResponse};
use crate::categories::{CategoriesListResponse, CategoryResponse, CategoryId};
use crate::tags::{TagListResponse};
use crate::transactions::{TransactionId, TransactionListResponse, TransactionResponse};

fn main() {
    println!("Hello, world!");

    let mut client = RestClient::new_blocking("https://api.up.com.au/api/v1/").unwrap();

    client.set_header("Authorization", "Bearer up:yeah:PwkaUOT6b6jhpAZvU8YQaRChesJThb2ucG5QwGp32nafosoxvjztlsLRkSFuaMkkApQN1L8sURlcFbtqIjPhf9lzl3mlO0GXhEVwDlbsss8jY1jYFJu8YRzbpIV8RxKr").unwrap();

    let _accounts: AccountsListResponse = client.get(()).unwrap();
    let _account: AccountResponse = client.get(
        AccountId::new("49908555-f6e9-42fa-9a02-8de302aecb51")
    ).unwrap();

    let _categories: CategoriesListResponse = client.get(()).unwrap();
    let _category: CategoryResponse = client.get(
        CategoryId::new("home")
    ).unwrap();

    let _tags: TagListResponse = client.get(()).unwrap();

    let _transactions: TransactionListResponse = client.get(()).unwrap();
    let _transaction: TransactionResponse = client
        .get(TransactionId::new("f45ca050-3d44-40bd-9c4c-a24e05fdfe8d")).unwrap();

    if let Some(link) = _transactions.links.next {
        println!("Getting next page");
        link.get(&mut client).unwrap();
    }
    

    println!("Data: {:?}", _transaction);
}


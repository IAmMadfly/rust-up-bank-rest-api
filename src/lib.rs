
// use serde::{Deserialize, Serialize};
pub use restson::{RestClient, blocking};

pub mod transactions;
pub mod accounts;
pub mod categories;
pub mod tags;
pub mod general;

mod test;

// use crate::accounts::{AccountId, AccountResponse, AccountsListResponse};
// use crate::categories::{CategoriesListResponse, CategoryResponse, CategoryId};
// use crate::tags::{TagListResponse};
// use crate::transactions::{TransactionId, TransactionListResponse, TransactionResponse};

// fn main() {
//     println!("Hello, world!");

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


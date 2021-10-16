use restson::blocking;
use restson::Error;

use up_bank_api;
use up_bank_api::accounts::AccountsListResponse;
use up_bank_api::accounts::AccountData;

fn main() {
    let accounts = get_accounts();
    println!("Accounts:\n{:#?}", accounts);
}

fn get_blocking_client() -> Result<blocking::RestClient, Error> {
    let client = up_bank_api::get_new_blocking_client(up_bank_api::get_token());

    client
}

fn get_accounts() -> Vec<AccountData> {
    let mut client = get_blocking_client().expect("Failed to get client");
    let accounts: Result<AccountsListResponse, Error> = client.get(());
    accounts.expect("Error getting accounts data").data
}
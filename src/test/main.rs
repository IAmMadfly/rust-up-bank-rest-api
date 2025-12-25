#[cfg(test)]
mod tests {
    use std::env;
    use std::path;

    use restson::blocking;
    use restson::Error;
    use restson::Response;
    use restson::RestClient;

    use crate::accounts::AccountsListResponse;
    use crate::get_new_blocking_client;
    use crate::get_new_client;
    use crate::transactions::TransactionListResponse;

    fn get_token() -> String {
        let path = path::Path::new(&env!("CARGO_MANIFEST_DIR")).join(".test_token");
        std::fs::read_to_string(path).unwrap()
    }

    fn get_blocking_client() -> Result<blocking::RestClient, Error> {
        let client = get_new_blocking_client(get_token());

        client
    }

    fn get_client() -> Result<RestClient, Error> {
        let client = get_new_client(get_token());

        client
    }

    #[test]
    fn test_blocking_client() {
        assert!(get_blocking_client().is_ok());
    }

    #[test]
    fn test_client() {
        assert!(get_client().is_ok());
    }

    #[test]
    fn get_accounts() {
        let client = get_blocking_client().expect("Failed to get client");

        let accounts: Result<Response<AccountsListResponse>, Error> = client.get(());
        assert!(accounts.is_ok());
    }

    #[test]
    fn get_transactions() {
        let mut client = get_blocking_client().expect("Failed to get client");

        let transactions = client.get::<(), TransactionListResponse>(());

        if let Err(e) = transactions {
            println!("Error: {:?}", e);
            panic!("Error on getting transactions");
        }

        if let Some(link) = &transactions.expect("Failed to get transactions").links.next {
            let more_trans = link.get_blocking(&mut client);
            assert!(more_trans.is_ok());
        } else {
            assert!(false);
        }
    }
}

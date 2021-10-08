


#[cfg(test)]
mod tests {
    use std::fs;

    use restson::Error;
    use restson::RestClient;
    use restson::blocking;
    use serde_derive::Deserialize;
    use toml;

    use crate::accounts::AccountsListResponse;
    use crate::get_new_client;
    use crate::get_new_blocking_client;
    use crate::transactions::TransactionListResponse;

    #[derive(Deserialize)]
    struct Config {
        token: String
    }

    fn get_token() -> String {
        let config: Config = toml::from_str(
            &fs::read_to_string("./test.toml")
                .expect("Failed to read config file")
            ).expect("Failed to parse into config");
        config.token
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
        let mut client = get_blocking_client().expect("Failed to get client");

        let accounts: Result<AccountsListResponse, Error> = client.get(());
        assert!(accounts.is_ok());
    }

    #[test]
    fn get_transactions() {
        let mut client = get_blocking_client().expect("Failed to get client");

        let transactions: Result<TransactionListResponse, Error> = client.get(());
        assert!(transactions.is_ok());

        if let Some(link) = transactions.expect("Failed to get transactions").links.next {
            let more_trans = link.get_blocking(&mut client);
            assert!(more_trans.is_ok());
        } else {
            assert!(false);
        }
    }
}

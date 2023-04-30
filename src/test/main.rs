#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    use restson::blocking;
    use restson::Error;
    use restson::Response;
    use restson::RestClient;
    use serde_derive::Deserialize;
    use toml;

    use crate::accounts::AccountsListResponse;
    use crate::get_new_blocking_client;
    use crate::get_new_client;
    use crate::transactions::TransactionListResponse;

    #[derive(Deserialize)]
    struct Config {
        token: String,
    }

    fn get_token() -> String {
        let test_token_path = "./test.toml";
        let config: Config;

        // Read Up Authentication token from environment variable
        if let Ok(token) = env::var("UP_TOKEN") {
            config = Config { token };
        }
        // Else read from test file
        else if Path::new(test_token_path).exists() {
            config = toml::from_str(
                &fs::read_to_string(test_token_path).expect("Failed to read config file"),
            )
            .expect("Failed to parse into config");
        }
        // Else error as no authentication token can be found
        else {
            panic!("No authentication token has been provided, please set UP_TOKEN env variable or consult documentation.")
        }

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
        let client = get_blocking_client().expect("Failed to get client");

        let accounts: Result<Response<AccountsListResponse>, Error> = client.get(());
        assert!(accounts.is_ok());
    }

    #[test]
    fn get_transactions() {
        let mut client = get_blocking_client().expect("Failed to get client");

        let transactions = client.get::<(), TransactionListResponse>(());
        assert!(transactions.is_ok());

        if let Some(link) = &transactions.expect("Failed to get transactions").links.next {
            let more_trans = link.get_blocking(&mut client);
            assert!(more_trans.is_ok());
        } else {
            assert!(false);
        }
    }
}

# Up Bank Rest API

This project is dedicated to being a simple to use API the gather the data given by Up Bank (https://up.com.au/)
It also provides the required types to gather the data

## Personal Access Token

A personal access token is required to query your personal data from the API and can be generated [here](https://api.up.com.au/getting_started).
This token should be stored in an environment variable called \"UP_TOKEN\" such as:

```sh
export UP_TOKEN="up:yeah:YourTestUpTokenHere"
```

Otherwise a file can be stored on the root of this repo called "test.toml" containing the following contents:

```toml
token = "up:yeah:YourTestUpTokenHere"
```

## Testing your access token

Once you have provided your access token, you can check that it works by running `cargo test`
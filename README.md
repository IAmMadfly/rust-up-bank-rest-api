# Up Bank Rest API

This project is dedicated to being a simple to use API the gather the data given by Up Bank (https://up.com.au/)
It also provides the required types to gather the data

## Personal Access Token

A personal access token is required to query your personal data from the API and can be generated [here](https://api.up.com.au/getting_started).

### Testing the library
To test this library, create a file called `.test_token` in the root of this repository containing your personal access token:

```
up:yeah:YourTestUpTokenHere
```

## Testing your access token

Once you have provided your access token, you can check that it works by running `cargo test`

//! Online checks that the live Up API still returns the fields this library
//! depends on, and that those responses still deserialize into our types.
//!
//! These tests need a `.test_token` file in the repository root (see README).

use std::path;

use serde_json::Value;

use crate::accounts::AccountsListResponse;
use crate::categories::CategoriesListResponse;
use crate::tags::TagListResponse;
use crate::transactions::TransactionListResponse;
use crate::utility::Ping;

const BASE_URL: &str = "https://api.up.com.au/api/v1/";

fn get_token() -> String {
    let path = path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".test_token");
    std::fs::read_to_string(path).unwrap().trim().to_string()
}

/// Fetches an endpoint and returns the raw JSON body, untouched by our types.
fn fetch(endpoint: &str) -> Value {
    let response = reqwest::blocking::Client::new()
        .get(format!("{}{}", BASE_URL, endpoint))
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .expect("request to Up API failed");

    let status = response.status();
    let body = response.text().expect("failed to read response body");
    assert!(
        status.is_success(),
        "{} returned {}: {}",
        endpoint,
        status,
        body
    );

    serde_json::from_str(&body).expect("response was not valid JSON")
}

/// Returns true if the dotted path exists in the JSON, including when its value is null.
/// Arrays are checked through their first element.
fn has_path(value: &Value, path: &str) -> bool {
    let mut current = value;
    for segment in path.split('.') {
        if let Value::Array(items) = current {
            match items.first() {
                Some(first) => current = first,
                None => return false,
            }
        }
        match current.get(segment) {
            Some(next) => current = next,
            None => return false,
        }
    }
    true
}

fn assert_paths(name: &str, json: &Value, paths: &[&str]) {
    let missing: Vec<&str> = paths
        .iter()
        .copied()
        .filter(|path| !has_path(json, path))
        .collect();
    assert!(
        missing.is_empty(),
        "{} response is missing fields the library uses: {:?}",
        name,
        missing
    );
}

fn has_items(json: &Value) -> bool {
    json["data"].as_array().is_some_and(|items| !items.is_empty())
}

#[test]
fn accounts_response_matches_schema() {
    let raw = fetch("accounts");
    assert_paths(
        "accounts",
        &raw,
        &[
            "data.id",
            "data.attributes.displayName",
            "data.attributes.accountType",
            "data.attributes.balance.currencyCode",
            "data.attributes.balance.value",
            "data.attributes.balance.valueInBaseUnits",
            "data.attributes.createdAt",
            "data.relationships.transactions.links.related",
            "links.prev",
            "links.next",
        ],
    );

    let typed: AccountsListResponse =
        serde_json::from_value(raw).expect("accounts response no longer deserializes");
    assert!(!typed.data.is_empty(), "expected at least one account");
}

#[test]
fn transactions_response_matches_schema() {
    let raw = fetch("transactions");
    if !has_items(&raw) {
        eprintln!("skipping transactions schema check: no transactions on this account");
        return;
    }

    assert_paths(
        "transactions",
        &raw,
        &[
            "data.id",
            "data.attributes.status",
            "data.attributes.rawText",
            "data.attributes.description",
            "data.attributes.message",
            "data.attributes.isCategorizable",
            "data.attributes.holdInfo",
            "data.attributes.roundUp",
            "data.attributes.cashback",
            "data.attributes.amount.valueInBaseUnits",
            "data.attributes.foreignAmount",
            "data.attributes.settledAt",
            "data.attributes.createdAt",
            "data.relationships.account.data",
            "data.relationships.transferAccount",
            "data.relationships.category.data",
            "data.relationships.parentCategory",
            "data.relationships.tags.data",
            "links.next",
        ],
    );

    let _: TransactionListResponse =
        serde_json::from_value(raw).expect("transactions response no longer deserializes");
}

#[test]
fn categories_response_matches_schema() {
    let raw = fetch("categories");
    assert_paths(
        "categories",
        &raw,
        &[
            "data.id",
            "data.attributes.name",
            "data.relationships.parent.data",
            "data.relationships.children.data",
        ],
    );

    let _: CategoriesListResponse =
        serde_json::from_value(raw).expect("categories response no longer deserializes");
}

#[test]
fn tags_response_matches_schema() {
    let raw = fetch("tags");
    if !has_items(&raw) {
        eprintln!("skipping tags schema check: no tags on this account");
        return;
    }

    assert_paths(
        "tags",
        &raw,
        &["data.id", "data.relationships.transactions.links"],
    );

    let _: TagListResponse =
        serde_json::from_value(raw).expect("tags response no longer deserializes");
}

#[test]
fn ping_response_matches_schema() {
    let raw = fetch("util/ping");
    assert_paths("ping", &raw, &["meta.id", "meta.statusEmoji"]);

    let ping: Ping = serde_json::from_value(raw).expect("ping response no longer deserializes");
    assert!(!ping.status().is_empty());
}

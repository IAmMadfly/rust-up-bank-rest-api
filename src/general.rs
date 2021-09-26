use std::{marker::PhantomData};

use restson::{Error, RestPath, blocking::RestClient};
use serde::{Deserialize, Serialize, de};
// use restson::{RestPath};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination<T> {
    pub prev: Option<Box<PageLink<T>>>,
    pub next: Option<Box<PageLink<T>>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PageLink<T> {
    pub link:           String,
    phantom:        PhantomData<T>
}

impl<T: RestPath<PageLink<T>> + for<'de> serde::Deserialize<'de>> PageLink<T> {
    pub fn get(self, client: &mut RestClient) -> Result<T, Error> {
        client.get(self)
    }
}

impl<'de, T> Deserialize<'de> for PageLink<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let link: String = Deserialize::deserialize(deserializer)?;

        let index = link.find("?");

        if let None = index {
            // panic!("Shits broekn");
            return Err(de::Error::custom("Error"));
            // return Err("Failed to get index of start of parameters in URL")
        }

        Ok(PageLink {
            link,
            phantom: PhantomData
        })
    }
}

#[derive(Serialize, Debug)]
pub struct RelationLink {
    link:   String
}

impl<'de> Deserialize<'de> for RelationLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let link = Deserialize::deserialize(deserializer)?;

        Ok(RelationLink {
            link
        })
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipsLink {
    related:        String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsLinks {
    links:          Option<RelationshipsLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoneyObject {
    #[serde(alias = "currencyCode")]
    currency_code:          String,
    value:                  String,
    #[serde(alias = "valueInBaseUnits")]
    value_in_base_units:    i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CashbackObject {
    description:        String,
    amount:             MoneyObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundUp {
    amount:             MoneyObject,
    boost_portion:      Option<MoneyObject>
}

#[derive(Serialize, Debug)]
pub struct TimeObject {
    time:               String
}

impl<'de> Deserialize<'de> for TimeObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let string = Deserialize::deserialize(deserializer)?;

        Ok(TimeObject {
            time:   string
        })
    }
}

use std::{marker::PhantomData};

use restson::{Error, RestPath, blocking::RestClient};
use serde::{Deserialize, Serialize, de};
// use restson::{RestPath};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination<T> {
    pub prev: Option<Box<PageLink<T>>>,
    pub next: Option<Box<PageLink<T>>>,
}

#[derive(Serialize, Debug)]
pub struct PageLink<T> {
    pub base_url:   String,
    pub params:     String,
    phantom:        PhantomData<T>
}

impl<T> Clone for PageLink<T> {
    fn clone(&self) -> Self {
        Self { 
            base_url: self.base_url.clone(), 
            params: self.params.clone(), 
            phantom: PhantomData
        }
    }
}

impl<'a ,T: 'a +  RestPath<&'a PageLink<T>> + for<'de> serde::Deserialize<'de>> PageLink<T> {
    pub fn get_blocking(&'a self, client: &mut RestClient) -> Result<T, Error> {
        client.get(self)
    }

    pub async fn get(&'a self, client: &mut restson::RestClient) -> Result<T, Error> {
        client.get(self).await
    }
}

impl<'de, T> Deserialize<'de> for PageLink<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let mut params: String = Deserialize::deserialize(deserializer)?;

        let index_maybe = params.find("?");

        let base_url: String = if let Some(index) = index_maybe {
            params.drain(..(index+1)).collect()
        } else {
            return Err(
                de::Error::custom(
                    "Failed to get index of parameters in URL for PageLink"
                )
            );
        };

        Ok(PageLink {
            base_url,
            params,
            phantom: PhantomData
        })
    }
}

#[derive(Serialize, Debug)]
pub struct RelationLink {
    pub link:   String
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
    pub related:        String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsLinks {
    pub links:          Option<RelationshipsLink>
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
    pub description:        String,
    pub amount:             MoneyObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundUp {
    pub amount:             MoneyObject,
    pub boost_portion:      Option<MoneyObject>
}

#[derive(Serialize, Debug)]
pub struct TimeObject {
    pub time:               String
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

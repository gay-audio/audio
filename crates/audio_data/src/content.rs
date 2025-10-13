// Needed because diesels proc-macros force there to be multiple bound locations.
#![allow(clippy::multiple_bound_locations)]

use crate::sealed::Sealed;
use derive_more::Display;
use diesel::{Selectable, pg::Pg, prelude::Queryable, sql_types::Text};
use serde::{Deserialize, Serialize};
use std::{error::Error, str::FromStr};
use uuid::Uuid;

macro_rules! direct_from_str {
    ($type:ident, $err:ident) => {
        impl ::std::str::FromStr for $type {
            type Err = $err;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                if s == <Self as ToString>::to_string(&$type) {
                    Ok($type)
                } else {
                    Err($err)
                }
            }
        }
        impl ::std::convert::TryFrom<::std::string::String> for $type {
            type Error = $err;
            fn try_from(value: ::std::string::String) -> std::result::Result<Self, Self::Error> {
                <Self as FromStr>::from_str(&value)
            }
        }
        impl ::std::convert::From<$type> for ::std::string::String {
            fn from(value: $type) -> ::std::string::String {
                <$type as ToString>::to_string(&value)
            }
        }
    };
}

pub trait ContentType: ToString + Sealed {}

#[derive(Display, Debug)]
pub struct ContentParseError;
impl Error for ContentParseError {}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Script;
direct_from_str!(Script, ContentParseError);
impl Queryable<Text, Pg> for Script {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Self::from_str(&row)?;

        Ok(Self)
    }
}
impl ContentType for Script {}

#[derive(Deserialize, Serialize, Display, Clone, Debug)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Audio;
direct_from_str!(Audio, ContentParseError);
impl Queryable<Text, Pg> for Audio {
    type Row = String;
    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Self::from_str(&row)?;

        Ok(Self)
    }
}
impl ContentType for Audio {}

#[derive(Deserialize, Serialize, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Content<C: ContentType> {
    id: Uuid,
    name: String,
    content_type: C,
}

pub struct CreateContent {
    pub name: String,
    pub content: String,
}

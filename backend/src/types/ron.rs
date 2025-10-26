use std::str::FromStr;

use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{
    ContentType, DeserializeHandler, SerializeResponder,
    macros::{impl_handler_for_deserialized_handler, impl_responder_for_serialized_responder},
};
use ron::{Error as RonError, de::SpannedError};

pub struct Ron<T>(pub T);

impl<T> Ron<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ContentType for Ron<T> {
    const NAME: &str = "ron";
    fn content_type() -> Mime {
        Mime::from_str("text/ron").unwrap()
    }
}
impl<T: Serialize> SerializeResponder for Ron<T> {
    type SerError = ron::Error;

    fn serialize(&self) -> Result<String, Self::SerError> {
        ron::to_string(&self.0)
    }
}
impl<T: for<'a> Deserialize<'a>> DeserializeHandler<T> for Ron<T> {
    type DeError = ron::de::SpannedError;

    fn deserialize(value: &str) -> Result<T, Self::DeError> {
        ron::from_str(value)
    }
}

impl_responder_for_serialized_responder!(Ron, SpannedError);
impl_handler_for_deserialized_handler!(Ron, RonError, SpannedError);

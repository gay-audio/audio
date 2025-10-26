use mime::{APPLICATION_JSON, Mime};
use serde::{Deserialize, Serialize};

use crate::types::{
    ContentType, DeserializeHandler, SerializeResponder,
    macros::{impl_handler_for_deserialized_handler, impl_responder_for_serialized_responder},
};
use serde_json::Error as JsonError;

pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ContentType for Json<T> {
    const NAME: &str = "json";
    fn content_type() -> Mime {
        APPLICATION_JSON
    }
}
impl<T: Serialize> SerializeResponder for Json<T> {
    type SerError = JsonError;

    fn serialize(&self) -> Result<String, Self::SerError> {
        serde_json::to_string(&self.0)
    }
}
impl<T: for<'a> Deserialize<'a>> DeserializeHandler<T> for Json<T> {
    type DeError = JsonError;

    fn deserialize(value: &str) -> Result<T, Self::DeError> {
        serde_json::from_str(value)
    }
}

impl_responder_for_serialized_responder!(Json, JsonError);
impl_handler_for_deserialized_handler!(Json, JsonError, JsonError);

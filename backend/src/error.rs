use std::{
    fmt::{Debug, Display},
    string::FromUtf8Error,
};

use actix_web::{ResponseError, error::PayloadError, http::StatusCode};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum SerdePayloadError<Ser, De> {
    #[error("Payload ({length} bytes) is larger than allowed (limit: {limit} bytes).")]
    OverflowKnownLength { length: usize, limit: usize },

    #[error("Payload has exceeded limit ({limit} bytes)")]
    Overflow { limit: usize },

    #[error("Content type error")]
    ContentType,

    #[error(transparent)]
    Deserialize(De),

    #[error(transparent)]
    Serialize(Ser),

    #[error("Error occured while reading payload: {0}")]
    Payload(PayloadError),

    #[error(transparent)]
    Utf8Error(FromUtf8Error),
}
impl<Ser, De> From<actix_web::error::PayloadError> for SerdePayloadError<Ser, De> {
    fn from(value: actix_web::error::PayloadError) -> Self {
        Self::Payload(value)
    }
}
impl<Ser, De> From<FromUtf8Error> for SerdePayloadError<Ser, De> {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8Error(value)
    }
}
impl<Ser: Display + Debug, De: Display + Debug> ResponseError for SerdePayloadError<Ser, De> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::OverflowKnownLength { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Overflow { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Serialize(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Payload(err) => err.status_code(),
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(ThisError, Debug)]
#[error("Uknown mime type")]
pub struct UnknownMimeType(pub String);
impl ResponseError for UnknownMimeType {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

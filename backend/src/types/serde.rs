//! In short, a mess. But it works

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::{
    FromRequest, HttpMessage, HttpResponse, Responder, body::EitherBody, error::ContentTypeError,
    web::Data,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    config::Config,
    error::UnknownMimeType,
    types::{
        ContentType,
        json::{Json, JsonExtractFut},
        ron::{Ron, RonExtractFut},
    },
};

pub enum SerdeExtractFut<T> {
    Ron(RonExtractFut<T>),
    Json(JsonExtractFut<T>),
    Unknown(ContentTypeError),
}
impl<T: DeserializeOwned> Future for SerdeExtractFut<T> {
    type Output = Result<Serde<T>, actix_web::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut *self {
            Self::Json(json) => Pin::new(json)
                .poll(cx)
                .map(|res| res.map(|json| Serde(json.0))),
            Self::Ron(ron) => Pin::new(ron)
                .poll(cx)
                .map(|res| res.map(|ron| Serde(ron.0))),
            Self::Unknown(ContentTypeError::ParseError) => {
                Poll::Ready(Err(actix_web::Error::from(ContentTypeError::ParseError)))
            }
            Self::Unknown(_) => Poll::Ready(Err(actix_web::Error::from(
                ContentTypeError::UnknownEncoding,
            ))),
        }
    }
}

/// A generic wrapper type for any
pub struct Serde<T>(pub T);
impl<T: for<'a> Deserialize<'a>> FromRequest for Serde<T> {
    type Error = actix_web::Error;
    type Future = SerdeExtractFut<T>;
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_http::Payload,
    ) -> Self::Future {
        let config = req.app_data::<Data<Config>>().unwrap();

        let default_content = config.server.default_mime();

        match req
            .mime_type()
            .map(|mime| mime.unwrap_or(default_content.clone()))
        {
            Ok(mime) if Ron::<T>::content_type(&mime) => {
                SerdeExtractFut::Ron(Ron::from_request(req, payload))
            }
            Ok(mime) if Json::<T>::content_type(&mime) => {
                SerdeExtractFut::Json(Json::from_request(req, payload))
            }
            Ok(_) => SerdeExtractFut::Unknown(ContentTypeError::UnknownEncoding),
            Err(err) => SerdeExtractFut::Unknown(err),
        }
        // todo!()
    }
}
impl<T: Serialize> Responder for Serde<T> {
    type Body = EitherBody<String>;
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let config = req.app_data::<Data<Config>>().unwrap();

        let default_content = config.server.default_mime();

        match req
            .mime_type()
            .map(|mime| mime.unwrap_or(default_content.clone()))
        {
            Ok(mime) if Ron::<T>::content_type(&mime) => Ron(self.0).respond_to(req),
            Ok(mime) if Json::<T>::content_type(&mime) => Json(self.0).respond_to(req),
            Ok(mime) => {
                HttpResponse::from_error(UnknownMimeType(mime.to_string())).map_into_right_body()
            }
            Err(err) => HttpResponse::from_error(err).map_into_right_body(),
        }
    }
}

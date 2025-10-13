// I never want to touch `macro_rules` ever again. This was a nightmare.

macro_rules! impl_responder_for_serialized_responder {
    ($name:tt, $de_error:ident) => {
        ::paste::paste! {
            impl<T: ::serde::Serialize> ::actix_web::Responder for $name<T> {
                type Body = ::actix_web::body::EitherBody<String>;

                fn respond_to(
                    self,
                    _: &actix_web::HttpRequest,
                ) -> ::actix_web::HttpResponse<Self::Body> {
                    match self.serialize() {
                        Ok(body) => match ::actix_web::HttpResponse::Ok()
                            .content_type(mime::APPLICATION_JSON)
                            .message_body(body)
                        {
                            Ok(res) => res.map_into_left_body(),
                            Err(err) => {
                                ::actix_web::HttpResponse::from_error(err).map_into_right_body()
                            }
                        },

                        Err(err) => {
                            ::actix_web::HttpResponse::from_error($crate::error::SerdePayloadError::<
                                <$name<T> as $crate::types::SerializeResponder>::SerError,
                                $de_error
                            >::Serialize(err))
                            .map_into_right_body()
                        }
                    }
                }
            }
        }
    };
}
pub(crate) use impl_responder_for_serialized_responder;

macro_rules! impl_handler_for_deserialized_handler {
    ($name:ty, $ser_error:ident, $de_error:ident) => {
        ::paste::paste! {
            type [<$name ErrorHandler>] = Option<
                ::std::sync::Arc<
                    dyn ::std::ops::Fn(
                            $crate::error::SerdePayloadError<$ser_error, $de_error>,
                            &::actix_web::HttpRequest,
                        ) -> ::actix_web::Error
                        + ::std::marker::Send
                        + ::std::marker::Sync,
                >,
            >;


            impl<T: ::serde::de::DeserializeOwned> ::actix_web::FromRequest for $name<T> {
                type Error = ::actix_web::Error;
                type Future = [<$name ExtractFut>]<T>;
                fn from_request(
                    req: &::actix_web::HttpRequest,
                    payload: &mut ::actix_web::dev::Payload,
                ) -> Self::Future {
                    let config = [<$name Config>]::from_req(req);

                    let limit = config.limit;
                    let ctype_required = config.content_type_required;
                    let err_handler = config.err_handler.clone();

                    [<$name ExtractFut>]{
                        req: ::std::option::Option::Some(req.clone()),
                        fut: [<$name Body>]::new(req, payload, ctype_required).limit(limit),
                        err_handler,
                    }
                }
            }

            pub struct [<$name ExtractFut>]<T> {
                req: ::std::option::Option<::actix_web::HttpRequest>,
                fut: [<$name Body>]<T>,
                err_handler: [<$name ErrorHandler>],
            }
            impl<T: ::serde::de::DeserializeOwned> ::std::future::Future for [<$name ExtractFut>]<T> {
                type Output = ::std::result::Result<$name<T>, ::actix_web::Error>;

                fn poll(self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context<'_>) -> ::std::task::Poll<Self::Output> {
                    let this = self.get_mut();

                    let res = ::futures_core::ready!(::std::pin::Pin::new(&mut this.fut).poll(cx));

                    let res = match res {
                        ::std::result::Result::Err(err) => {
                            let req = this.req.take().unwrap();

                            ::log::debug!(
                                "Failed to deserialize {} from payload. \
Request path: {}",
                                <$name<T> as ContentType>::NAME,
                                req.path()
                            );

                            if let ::std::option::Option::Some(err_handler) = this.err_handler.as_ref() {
                                ::std::result::Result::Err((*err_handler)(err, &req))
                            } else {
                                ::std::result::Result::Err(err.into())
                            }
                        }
                        ::std::result::Result::Ok(data) => ::std::result::Result::Ok($name(data)),
                    };

                    ::std::task::Poll::Ready(res)
                }
            }

            #[derive(Clone)]
            pub struct [<$name Config>] {
                limit: usize,
                err_handler: [<$name ErrorHandler>],
                content_type_required: bool,
            }
            impl [<$name Config>] {
                /// Set maximum accepted payload size. By default this limit is 2MB.
                pub fn limit(mut self, limit: usize) -> Self {
                    self.limit = limit;
                    self
                }

                /// Set custom error handler.
                pub fn error_handler<F>(mut self, f: F) -> Self
                where
                    F: ::std::ops::Fn(
                            $crate::error::SerdePayloadError<$ser_error, $de_error>,
                            &::actix_web::HttpRequest,
                        ) -> ::actix_web::Error
                        + ::std::marker::Send
                        + ::std::marker::Sync
                        + 'static,
                {
                    self.err_handler = ::std::option::Option::Some(::std::sync::Arc::new(f));
                    self
                }

                /// Sets whether or not the request must have a `Content-Type` header to be parsed.
                pub fn content_type_required(mut self, content_type_required: bool) -> Self {
                    self.content_type_required = content_type_required;
                    self
                }
                fn from_req(req: &::actix_web::HttpRequest) -> &Self {
                    req.app_data::<Self>()
                        .or_else(|| req.app_data::<::actix_web::web::Data<Self>>().map(|d| d.as_ref()))
                        .unwrap_or(&DEFAULT_CONFIG)
                }
            }

            const DEFAULT_LIMIT: usize = 2_097_152; // 2 mb

            /// Allow shared refs used as default.
            const DEFAULT_CONFIG: [<$name Config>] = [<$name Config>] {
                limit: DEFAULT_LIMIT,
                err_handler: None,
                content_type_required: true,
            };

            impl Default for [<$name Config>] {
                fn default() -> Self {
                    DEFAULT_CONFIG
                }
            }

            pub enum [<$name Body>]<T> {
                Error(::std::option::Option<$crate::error::SerdePayloadError<$ser_error, $de_error>>),
                Body {
                    limit: usize,
                    length: Option<usize>,
                    payload: ::actix_web::dev::Payload,
                    buf: ::actix_web::web::BytesMut,
                    _res: ::std::marker::PhantomData<T>,
                },
            }
            impl<T> ::std::marker::Unpin for [<$name Body>]<T> {}
            impl<T: ::serde::de::DeserializeOwned> [<$name Body>]<T> {
                pub fn new(req: &::actix_web::HttpRequest, payload: &mut ::actix_web::dev::Payload, content_type_required: bool) -> Self {
                    match <::actix_web::HttpRequest as ::actix_web::HttpMessage>::mime_type(req) {
                        ::std::result::Result::Ok(::std::option::Option::Some(mime)) if $name::<T>::content_type(&mime) => {}
                        ::std::result::Result::Ok(_) if !content_type_required => {}
                        _ => {
                            return Self::Error(::std::option::Option::Some(
                                $crate::error::SerdePayloadError::<$ser_error, $de_error>::ContentType,
                            ));
                        }
                    }

                    let length = <::actix_web::http::header::ContentLength as ::actix_web::http::header::Header>::parse(req).ok().map(|x| x.0);

                    let payload = payload.take();

                    Self::Body {
                        limit: DEFAULT_LIMIT,
                        length,
                        payload,
                        buf: ::actix_web::web::BytesMut::with_capacity(8192),
                        _res: ::std::marker::PhantomData,
                    }
                }

                pub fn limit(self, limit: usize) -> Self {
                    match self {
                        Self::Body {
                            length: Some(len), ..
                        } if len > limit => Self::Error(::std::option::Option::Some($crate::error::SerdePayloadError::OverflowKnownLength {
                            length: len,
                            limit,
                        })),
                        Self::Body {
                            length,
                            payload,
                            buf,
                            ..
                        } => Self::Body {
                            limit,
                            length,
                            payload,
                            buf,
                            _res: ::std::marker::PhantomData,
                        },
                        _ => self,
                    }
                }
            }

            impl<T: ::serde::de::DeserializeOwned> ::std::future::Future for [<$name Body>]<T> {
                type Output = ::std::result::Result<T, $crate::error::SerdePayloadError<$ser_error, $de_error>>;

                fn poll(self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context<'_>) -> ::std::task::Poll<Self::Output> {
                    let this = self.get_mut();

                    match this {
                        Self::Body {
                            limit,
                            buf,
                            payload,
                            ..
                        } => loop {
                            let res = std::pin::Pin::new(&mut *payload);
                            let res = ::futures_core::ready!(
                                <::actix_web::dev::Payload as ::futures_core::Stream>::poll_next(res, cx)
                            );
                            match res {
                                ::std::option::Option::Some(chunk) => {
                                    let chunk = chunk?;
                                    let buf_len = buf.len() + chunk.len();
                                    if buf_len > *limit {
                                        return ::std::task::Poll::Ready(::std::result::Result::Err($crate::error::SerdePayloadError::Overflow { limit: *limit }));
                                    } else {
                                        buf.extend_from_slice(&chunk);
                                    }
                                }
                                ::std::option::Option::None => {
                                    let string = ::std::string::String::from_utf8(buf.to_vec())?;

                                    let ron =
                                        <$name<T> as $crate::types::DeserializeHandler<T>>::deserialize(&string).map_err($crate::error::SerdePayloadError::Deserialize)?;
                                    return ::std::task::Poll::Ready(::std::result::Result::Ok(ron));
                                }
                            }
                        },
                        Self::Error(err) => ::std::task::Poll::Ready(::std::result::Result::Err(err.take().unwrap())),
                    }
                }
            }

        }
    };
}
pub(crate) use impl_handler_for_deserialized_handler;

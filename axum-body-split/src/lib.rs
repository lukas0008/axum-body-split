use axum::{
    body::Bytes,
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use std::{fmt::Debug, marker::PhantomData};

pub struct SplitBody<T1, T2, State>(pub T1, pub T2, pub PhantomData<State>)
where
    T1: FromRequest<State> + Send + Sync + Send + Sync,
    T2: FromRequest<State> + Send + Sync + Send + Sync,
    State: Send + Sync;

impl<T1, T2, S> FromRequest<S> for SplitBody<T1, T2, S>
where
    T1: FromRequest<S> + Send + Sync,
    T2: FromRequest<S> + Send + Sync,
    T1::Rejection: Debug + Send + Sync,
    T2::Rejection: Debug + Send + Sync,
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        // async move {

        fn make_builder(req: &axum::extract::Request) -> axum::http::request::Builder {
            let mut builder = axum::extract::Request::builder();
            if let Some(headers) = builder.headers_mut() {
                *headers = req.headers().clone();
            }
            builder.uri(req.uri().clone()).version(req.version())
        }

        let builders = (make_builder(&req), make_builder(&req));

        let body = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        let builders = (
            builders
                .0
                .body(axum::body::Body::new(http_body_util::Full::new(
                    body.clone(),
                )))
                // can unwrap, because this req is a clone of the one before
                .unwrap(),
            builders
                .1
                .body(axum::body::Body::new(http_body_util::Full::new(body)))
                // can unwrap, because this req is a clone of the one before
                .unwrap(),
        );
        // axum::extract::Json;
        Ok(SplitBody(
            T1::from_request(builders.0, state)
                .await
                .map_err(|err| err.into_response())?,
            T2::from_request(builders.1, state)
                .await
                .map_err(|err| err.into_response())?,
            PhantomData,
        ))
    }
}

use crate::{Context, ResponseType};
use async_trait::async_trait;
use futures::future::Future;
use hyper::{Method, StatusCode};
use route_recognizer::{Match, Params, Router as InternalRouter};
use std::collections::HashMap;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, context: Context) -> ResponseType;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn(Context) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: IntoResponse,
{
    async fn invoke(&self, context: Context) -> ResponseType {
        (self)(context).await.into_response()
    }
}

pub struct RouterMatch<'a> {
    pub handler: &'a dyn Handler,
    pub params: Params,
}

pub struct Router {
    method_map: HashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            method_map: HashMap::default(),
        }
    }

    pub fn post(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::POST)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn get(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::GET)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }


    pub fn update(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::PATCH)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn delete(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::DELETE)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn route(&self, path: &str, method: &Method) -> RouterMatch<'_> {
        if let Some(Match { handler, params }) = self
            .method_map
            .get(method)
            .and_then(|r| r.recognize(path).ok())
        {
            RouterMatch {
                handler: &**handler,
                params,
            }
        } else {
            RouterMatch {
                handler: &not_found_handler,
                params: Params::new(),
            }
        }
    }
}

async fn not_found_handler(_cx: Context) -> ResponseType {
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("NOT FOUND".into())
        .unwrap()
}

pub trait IntoResponse: Send + Sized {
    fn into_response(self) -> ResponseType;
}

impl IntoResponse for ResponseType {
    fn into_response(self) -> ResponseType {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> ResponseType {
        ResponseType::new(self.into())
    }
}

impl IntoResponse for String {
    fn into_response(self) -> ResponseType {
        ResponseType::new(self.into())
    }
}
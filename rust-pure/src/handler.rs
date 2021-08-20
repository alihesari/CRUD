use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
    email: String,
}

pub async fn create(mut ctx: Context) -> Response {
    let body: CreateRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.email
        )
        .into(),
    )
}

pub async fn read(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

pub async fn update(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

pub async fn delete(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

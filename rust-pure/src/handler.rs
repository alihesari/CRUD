use crate::{Context, ResponseType, users};
// use crate::{Context, ResponseType};
use hyper::StatusCode;
use serde::Deserialize;
use sea_orm::Database;
use sea_orm::{entity::*, error::*, query::*, DbConn};
pub use super::usersModel::Entity as Users;

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
    email: String,
}

pub async fn create(mut ctx: Context) -> ResponseType {
    let body: CreateRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };
    

    ResponseType::new(
        format!(
            "send called with. name: {} , email: {}",
            body.name, body.email
        )
        .into(),
    )
}

pub async fn read(ctx: Context) -> String {
    let db = Database::connect("postgres://postgres:123@host/crud").await.unwrap();
    let usedata = users::ActiveModel {
        name: Set("Ali".to_owned()),
        email: Set("asd@asd.com".to_owned()),
        ..Default::default()
    };
    Users::insert(usedata).exec(&db).await;

    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

pub async fn update(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

pub async fn delete(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

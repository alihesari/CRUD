use super::*;
use crate::{Context, ResponseType, users};
use hyper::StatusCode;
use serde::Deserialize;
use sea_orm::Database;
use sea_orm::{entity::*, error::*, query::*, DbConn};

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
    email: String,
}

pub async fn adduser(db: &DbConn) -> Result<(), DbErr> {
    let usedata = users::ActiveModel {
        name: Set("Ali".to_owned()),
        email: Set("asd@asd.com".to_owned()),
        ..Default::default()
    };
    let res: InsertResult = Users::insert(usedata).exec(db).await?;

    println!();
    println!("Inserted: last_insert_id = {}\n", res.last_insert_id);

    Ok(())
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

    let db = Database::connect("postgres://postgres:123@host/crud").await.unwrap();
    adduser(&db).await;

    ResponseType::new(
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

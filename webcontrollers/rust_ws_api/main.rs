#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use rocket_contrib::json::Json;
use serde_json::{json, Value};

mod request;
use request::{IgnoreNoneFieldsUpdateRequest, NewRequest, Request};

mod log;
use log::{IgnoreNoneFieldsUpdateLog, Log, NewLog};

pub mod repository;

pub mod driver;
pub mod schema;

fn extract_payload(request: Json<NewRequest>) -> Json<NewLog> {
    if (request.api_key == "12345") {
        request.log
    }
    request.log as NewLog
}

#[post("/", data = "<request>")]
fn create(request: Json<NewRequest>, connection: driver::Connection) -> Json<Log> {
    let l = extract_payload(request);

    let insert = NewLog { ..l.into_inner() };
    Json(repository::create(insert, &connection))
}

#[get("/")]
fn read(connection: driver::Connection) -> Json<Value> {
    Json(json!(repository::read(&connection)))
}

#[put("/<id>", data = "<log>")]
fn update(
    id: i32,
    log: Json<IgnoreNoneFieldsUpdateLog>,
    connection: driver::Connection,
) -> Json<Value> {
    let update = IgnoreNoneFieldsUpdateLog { ..log.into_inner() };
    Json(json!({
        "success": repository::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: driver::Connection) -> Json<Value> {
    Json(json!({ "success": repository::delete(id, &connection) }))
}

fn main() {
    rocket::ignite()
        .mount("/log", routes![create, update, delete])
        .mount("/logs", routes![read])
        .manage(driver::connect())
        .launch();
}

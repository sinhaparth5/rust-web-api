#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{Value, json};

#[get("/rust-api")]
fn get_rust_api() -> Value {
    json!([
        { "id": 1, "name": "John Doe" },
        { "id": 2, "name": "Jane Doe" }
    ])
}

#[get("/rust-api/<id>")]
fn view_rust_api(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[post("/rust-api", format = "json")]
fn create_rust_api() -> Value {
    json!({
        "id": 3,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[put("/rust-api/<id>", format = "json")]
fn update_rust_api(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[delete("/rust-api/<_id>")]
fn delete_rust_api(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rust_api,
            view_rust_api,
            create_rust_api,
            update_rust_api,
            delete_rust_api
        ])
        .launch()
        .await;
}
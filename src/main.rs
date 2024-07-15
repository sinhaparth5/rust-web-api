#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket::serde::json::{Value, json};

struct BasicAuth {
    pub username: String,
    pub password: String,
}

 impl BasicAuth {
     fn from_authorization_header(header: &str) -> Option<BasicAuth> {
         let split = header.split_whitespace().collect::<Vec<_>>();
         if split.len() != 2 {
             return None;
         }

         if split[0] != "Basic" {
             return None;
         }
         Self::from_base64_encoded(split[1])
     }

     fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
         let decoded = base64::decode(base64_string).ok()?;
         let decoded_str = String::from_utf8(decoded).ok()?;
         let split = decoded_str.split(":").collect::<Vec<_>>();
         if split.len() != 2 {
             return None;
         }
         let (username, password) = (split[0].to_string(), split[1].to_string());
         Some(BasicAuth {
             username,
             password
         })
     }
 }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

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

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
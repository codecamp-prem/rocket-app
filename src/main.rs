#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{"id": 1, "name": "John Doe"}, {"id": 2, "name": "Jane Doe"}])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!([{"id": id, "name": "John Doe", "email": "john@doe.com"}])
}

#[post("/rustaceans", format = "json")]
fn create_rustaceans() -> Value {
    json!([{"id": 3, "name": "Man Doe", "email": "man@doe.com"}])
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id: i32) -> Value {
    json!([{"id": id, "name": "Man Doe", "email": "man@doe.com"}])
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("404: not Found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustaceans,
                create_rustaceans,
                update_rustaceans,
                delete_rustaceans
            ],
        )
        .register("/", catchers![not_found])
        .launch()
        .await;
}

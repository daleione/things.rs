#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

use diesel::query_dsl::RunQueryDsl;
use things_db::models::Tag;
use things_db::schema::tag::dsl as tag_dsl;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tags")]
fn tags() -> Json<Vec<Tag>> {
    let connection = &mut things_db::establish_connection();
    let tags: Vec<Tag> = tag_dsl::tag.load(connection).expect("Error loading tag");
    Json(tags)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, tags])
}

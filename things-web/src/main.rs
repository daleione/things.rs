#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use things_db::models::{Tag, Task};
use things_db::schema::tag;
use things_db::schema::task;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tags")]
fn tags() -> Template {
    let connection = &mut things_db::establish_connection();
    let tags: Vec<Tag> = tag::dsl::tag.load(connection).expect("Error loading tag");
    let tasks: Vec<Task> = task::dsl::task
        .order_by(task::creation_date.desc())
        .limit(10)
        .load(connection)
        .expect("Error loading task");
    Template::render(
        "tags",
        context! {
            tags: tags,
            tasks: tasks,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index, tags])
}

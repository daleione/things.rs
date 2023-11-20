use diesel::prelude::*;
use things_db::models::*;
use things_db::schema::area::dsl as area_dsl;
use things_db::schema::tag::dsl as tag_dsl;
use things_db::schema::task::dsl as task_dsl;

fn main() {
    let connection = &mut things_db::establish_connection();
    let areas: Vec<Area> = area_dsl::area.load(connection).expect("Error loading area");

    println!("Displaying all {} area:", areas.len());
    for a in areas {
        println!("* {}", a);
    }

    let tags: Vec<Tag> = tag_dsl::tag.load(connection).expect("Error loading tag");
    println!("Displaying all {} tag:", tags.len());
    for t in tags {
        println!("* {}", t);
    }

    let tasks: Vec<Task> = task_dsl::task.limit(10).load(connection).expect("Error loading task");
    println!("Displaying all {} tasks:", tasks.len());
    for t in tasks {
        println!("* {}", t);
    }
}

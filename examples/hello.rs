use diesel::prelude::*;
use things_rs::models::*;
use things_rs::schema::area::dsl::*;
use things_rs::schema::tag::dsl::*;

fn main() {
    let connection = &mut things_rs::establish_connection();
    let areas: Vec<Area> = area.load(connection).expect("Error loading area");

    println!("Displaying all {} area:", areas.len());
    for a in areas {
        println!("* {}", a);
    }

    let tags: Vec<Tag> = tag.load(connection).expect("Error loading tag");
    println!("Displaying all {} tag:", tags.len());
    for t in tags {
        println!("* {}", t);
    }
}

use diesel::prelude::*;
use things_rs::models::*;
use things_rs::schema::tm_area::dsl::*;
use things_rs::schema::tm_tag::dsl::*;

fn main() {
    let connection = &mut things_rs::establish_connection();
    let areas: Vec<TMArea> = tm_area.load(connection).expect("Error loading area");

    println!("Displaying all {} area:", areas.len());
    for area in areas {
        println!("* {}", area);
    }

    let tags: Vec<TMTag> = tm_tag.load(connection).expect("Error loading tag");
    println!("Displaying all {} tag:", tags.len());
    for tag in tags {
        println!("* {}", tag);
    }
}

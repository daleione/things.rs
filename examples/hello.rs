use diesel::prelude::*;
use things_rs::models::*;
use things_rs::schema::TMArea::dsl::*;

fn main() {
    let connection = &mut things_rs::establish_connection();
    let areas: Vec<Area> = TMArea.load(connection).expect("Error loading area");

    println!("Displaying all {} area:", areas.len());
    for area in areas {
        println!("{:?}", area);
    }
}

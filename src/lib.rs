use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

const DEFAULT_FILEPATH: &'static str = "~/Library/Group Containers/JLMPQHK86H.com.culturedcode.ThingsMac/ThingsData-5YIWW/Things Database.thingsdatabase";
const ENVIRONMENT_VARIABLE_WITH_FILEPATH: &'static str = "THINGSDB";

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

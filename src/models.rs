use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::TMArea)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Area {
    pub uuid: Option<String>,
    pub title: Option<String>,
    pub visible: Option<i32>,
    pub index: Option<i32>,
    pub cachedTags: Option<Vec<u8>>,
    pub experimental: Option<Vec<u8>>,
}

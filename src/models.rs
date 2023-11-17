use std::fmt;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::area)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Area {
    pub uuid: Option<String>,
    pub title: Option<String>,
    pub visible: Option<i32>,
    pub index: Option<i32>,
    pub cached_tags: Option<Vec<u8>>,
    pub experimental: Option<Vec<u8>>,
}

impl fmt::Display for Area{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(title) = &self.title {
            write!(f, "{}", title)
        } else {
            write!(f, "")
        }
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub uuid: Option<String>,
    pub title: Option<String>,
    pub shortcut: Option<String>,
    pub used_date: Option<f32>,
    pub parent: Option<String>,
    pub index: Option<i32>,
    pub experimental: Option<Vec<u8>>,
}

impl fmt::Display for Tag{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(title) = &self.title {
            write!(f, "{}", title)
        } else {
            write!(f, "")
        }
    }
}

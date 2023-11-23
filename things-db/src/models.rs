use rocket::serde::Serialize;
use diesel::prelude::*;
use diesel::deserialize::{self, FromSql};
use chrono;
use std::fmt;
use diesel::sqlite::Sqlite;

fn print_option<T: ToString>(opt: &Option<T>) -> String {
    if let Some(title) = opt {
        title.to_string()
    } else {
        "".to_string()
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::area)]
#[diesel(check_for_backend(Sqlite))]
pub struct Area {
    pub uuid: Option<String>,
    pub title: Option<String>,
    pub visible: Option<i32>,
    pub index: Option<i32>,
    pub cached_tags: Option<Vec<u8>>,
    pub experimental: Option<Vec<u8>>,
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_option(&self.title))
    }
}

#[derive(Serialize, Queryable, Selectable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(Sqlite))]
pub struct Tag {
    pub uuid: Option<String>,
    pub title: Option<String>,
    pub shortcut: Option<String>,
    pub used_date: Option<f32>,
    pub parent: Option<String>,
    pub index: Option<i32>,
    pub experimental: Option<Vec<u8>>,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_option(&self.title))
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Datetime(chrono::NaiveDateTime);

#[derive(Serialize, Queryable, Selectable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::task)]
#[diesel(check_for_backend(Sqlite))]
pub struct Task {
    pub uuid: Option<String>,
    pub leaves_tombstone: Option<i32>,
    pub creation_date: Option<Datetime>,
    pub user_modification_date: Option<f32>,
    pub type_: Option<i32>,
    pub status: Option<i32>,
    pub stop_date: Option<f32>,
    pub trashed: Option<i32>,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub notes_sync: Option<i32>,
    pub cached_tags: Option<Vec<u8>>,
    pub start: Option<i32>,
    pub start_date: Option<i32>,
    pub start_bucket: Option<i32>,
    pub reminder_time: Option<i32>,
    pub last_reminder_interaction_date: Option<f32>,
    pub deadline: Option<i32>,
    pub deadline_suppression_date: Option<i32>,
    pub t2_deadline_offset: Option<i32>,
    pub index: Option<i32>,
    pub today_index: Option<i32>,
    pub today_index_reference_date: Option<i32>,
    pub area: Option<String>,
    pub project: Option<String>,
    pub heading: Option<String>,
    pub contact: Option<String>,
    pub untrashed_leaf_actions_count: Option<i32>,
    pub open_untrashed_leaf_actions_count: Option<i32>,
    pub checklist_items_count: Option<i32>,
    pub open_checklist_items_count: Option<i32>,
    pub rt1_repeating_template: Option<String>,
    pub rt1_recurrence_rule: Option<Vec<u8>>,
    pub rt1_instance_creation_start_date: Option<i32>,
    pub rt1_instance_creation_paused: Option<i32>,
    pub rt1_instance_creation_count: Option<i32>,
    pub rt1_after_completion_reference_date: Option<i32>,
    pub rt1_next_instance_start_date: Option<i32>,
    pub experimental: Option<Vec<u8>>,
    pub repeater: Option<Vec<u8>>,
    pub repeater_migration_date: Option<f32>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_option(&self.title))
    }
}

impl FromSql<diesel::sql_types::Float, Sqlite> for Datetime {
    fn from_sql(bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_> ) -> deserialize::Result<Self> {
        let float_value = <f32 as FromSql<diesel::sql_types::Float, Sqlite>>::from_sql(bytes)?;
        let seconds = float_value as i64;
        let nanoseconds = (float_value.fract() * 1_000_000_000.0) as u32;
        Ok(Datetime(chrono::NaiveDateTime::from_timestamp(seconds, nanoseconds)))
    }
}

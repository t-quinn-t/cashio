use diesel::prelude::*;
use chrono::prelude::*;

/// Data abstraction for a single record
#[derive(PartialEq, Debug, Queryable)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub cents: i32,
    pub date: NaiveDate,
    pub category: String,
    pub description: String,
}

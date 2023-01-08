use std::{fmt::Display, path::PathBuf};

use anyhow::{Ok, Result};
use chrono::NaiveDate;
use log::{info, debug};
use rusqlite::{params, Connection};

use crate::config::{self, db_path, test_db_path};

#[derive(Debug)]
struct Repo(PathBuf);

/// Data abstraction for a single record
pub struct Record {
    pub id: i32,
    pub name: String,
    pub cents: i32,
    pub date: NaiveDate,
    pub category: String,
    pub description: String,
}

pub enum RepoError {
    ConnectionFailed(String),
}

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "Database Error: {}", &msg),
        }
    }
}

#[derive(Debug)]
pub struct LsQueryOptions {
    id: Option<i32>,
    fuzzy: Option<String>,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    category: Option<String>,
}

impl LsQueryOptions {
    /// Creates an empty LsQueryOption
    pub fn new() -> Self {
        LsQueryOptions {
            id: None,
            fuzzy: None,
            date_from: None,
            date_to: None,
            category: None,
        }
    }

    /// Set id for QueryOption
    pub fn id(&mut self, id: i32) -> &Self {
        self.id = Some(id);
        self
    }

    pub fn fuzzy(&mut self, fuzzy: &str) -> &Self {
        self.fuzzy = Some(String::from(fuzzy));
        self
    }

    pub fn date_from(&mut self, from: NaiveDate) -> &Self {
        self.date_from = Some(from);
        self
    }

    pub fn date_to(&mut self, to: NaiveDate) -> &Self {
        self.date_to = Some(to);
        self
    }

    pub fn category(&mut self, category: &str) -> &Self {
        self.category = Some(String::from(category));
        self
    }
}

impl Repo {
    pub fn new() -> Result<Self> {
        Ok(Repo(db_path()?))
    }

    fn test() -> Result<Self> {
        Ok(Repo(test_db_path()?))
    }

    pub fn init(&self) -> Result<()> {
        let conn = Connection::open(self.0.as_path())?;
        let sql = "CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name CHAR(50) NOT NULL,
            cents INTEGER NOT NULL,
            date TEXT,
            category CHAR(50),
            description TEXT
        )";
        let k = conn.execute(sql, ())?;
        Ok(())
    }
    
    pub fn insert(&self, record: &Record) -> Result<()> {
        let conn = Connection::open(config::db_path()?)?;
        let sql = "INSERT INTO records (name, cents, date, category, description) values (

    )";

        Ok(())
    }

    pub fn list_all(&self) -> Result<Vec<Record>> {
        let conn = Connection::open(config::db_path()?)?;
        todo!()
        // Ok(Vec::new())
    }

    pub fn modify(&self, record: &Record) -> Result<Record> {
        let conn = Connection::open(config::db_path()?)?;
        todo!()
    }
}
#[cfg(test)]
mod test_repo {
    use super::Record;
    use chrono::{Local, NaiveDate};

    #[test]
    fn test_repo() {
        let r0 = Record {
            id: 0,
            name: String::from("r0"),
            cents: -1000,
            date: Local::now().date_naive(),
            category: String::from("default"),
            description: String::from("Description for id 0"),
        };

        let r1 = Record {
            id: 1,
            name: String::from("r1"),
            cents: 32032,
            date: NaiveDate::from_ymd_opt(2000, 10, 30).unwrap(),
            category: String::from("default"),
            description: String::from("Description for id 1"),
        };

        let r2 = Record {
            id: 2,
            name: String::from("r2"),
            cents: -1190,
            date: NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(),
            category: String::from("test"),
            description: String::from("Description for id 2"),
        };

        let repo = super::Repo::test();
        assert!(repo.is_ok());
        let repo = repo.unwrap();
        let r = repo.init();
        assert!(r.is_ok());
        // Add all these records into test database
    }
}

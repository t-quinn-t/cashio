use std::{fmt::Display, path::PathBuf};

use anyhow::{Ok, Result};
use chrono::NaiveDate;
use rusqlite::{params, Connection};

#[derive(Debug)]
struct Repo(Connection);

/// Data abstraction for a single record
#[derive(PartialEq, Debug)]
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
    pub fn new(pathOpt: Option<PathBuf>) -> Result<Self> {
        let conn: Connection;
        if let Some(p) = pathOpt {
            conn = Connection::open(p)?;
        } else {
            conn = Connection::open_in_memory()?;
        }
        Ok(Repo(conn))
    }

    pub fn init(&self) -> Result<()> {
        let sql = "CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name CHAR(50) NOT NULL,
            cents INTEGER NOT NULL,
            date TEXT,
            category CHAR(50),
            description TEXT
        )";
        self.0.execute(sql, ())?;
        Ok(())
    }
    
    pub fn insert(&self, record: &Record) -> Result<()> {
        let sql = "INSERT INTO records (name, cents, date, category, description) values (?1, ?2
    , ?3, ?4, ?5)";
        self.0.execute(sql, params![record.name, record.cents, record.date.format("%Y-%m-%d").to_string(), record.category, record.description])?;
        Ok(())
    }
    
    pub fn list_all(&self) -> Result<Vec<Record>> {
        
        let sql = "SELECT * FROM records";
        let mut stmt = self.0.prepare(sql)?;
        let mapping_fn = |row:& rusqlite::Row| {
            let date_string:String = row.get(3)?;
            let date = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d")?;
            Ok(Record {
                id: row.get(0)?,
                name: row.get(1)?,
                cents: row.get(2)?,
                date,
                category: row.get(4)?,
                description: row.get(5)?,
            })
        };
        let rows = stmt.query_map([], |row| {
            // ? does not work here as here we need a rusqlite::Result ...
            // TODO: move repositories to a module where only rusqlite::Result is used
            let record = mapping_fn(row);
            if record.is_err() {
                return Err(rusqlite::Error::InvalidColumnName(String::from("")))
            }
            return rusqlite::Result::Ok(record.unwrap())
        })?;
        
        let mut rows_vec = Vec::new();
        for row in rows {
            if row.is_ok() {
                rows_vec.push(row.unwrap())
            } 
        }

        Ok(rows_vec)
            
    }

    pub fn modify(&self, record: &Record) -> Result<Record> {
        todo!()
    }

}
#[cfg(test)]
mod test_repo {
    use super::Record;
    use chrono::Local;
    use rusqlite::Connection;

    #[test]
    fn test_records_repo() {
        
        let num_records = 100;
        let mut records = Vec::with_capacity(100);

        let categories = vec!["default", "grocery", "gifts", "utility"];
        let description_suffixs = vec!["Normal Text:", "Other Text:"];
        for i in 0_usize..num_records {
            let name = i.to_string();
            let cents = i as i32 * 100 * (1 - i as i32 % 2 * 2);
            let date = Local::now().naive_local().date();
            let category = String::from(*categories.get(i % categories.len()).unwrap());
            let mut description = String::from(*description_suffixs.get(i % description_suffixs.len()).unwrap());
            description.push_str("Common Description");
            let record = Record {
                id: (i+1) as i32,
                name,
                cents: cents as i32,
                date,
                category,
                description
            };
            records.push(record);
        }

        let repo = super::Repo::new(None);
        assert!(repo.is_ok());
        let repo = repo.unwrap();
        let r = repo.init();
        assert!(r.is_ok());
        // Add all these records into test databas
    
        println!("Adding all records");
        for r in records.iter() {
            repo.insert(r).unwrap();
        }
        
        let ls_result = repo.list_all().unwrap();
        for (i, rr) in ls_result.iter().enumerate() {
            assert_eq!(rr, records.get(i).unwrap())
        }
    }
}

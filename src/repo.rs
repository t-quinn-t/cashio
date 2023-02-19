use std::{fmt::Display, str::FromStr};
use anyhow::{Result, Ok};
use chrono::NaiveDate;
use futures::TryStreamExt;
use log::debug;
use sqlx::{SqliteConnection, sqlite::SqliteConnectOptions, ConnectOptions, Row};

use crate::model::Record;

pub struct Repo(SqliteConnection);
pub struct URL(String);

macro_rules! get_conn {
	($r: expr) => {
	    &mut $r.0
	};
}

const DB_DATE_FMT: &str = "%Y-%m-%d";

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

impl Repo {
    
    /// Creates the Repo object and starts the connection.
    pub async fn new(url: Option<URL>) -> Result<Self> {
        if let Some(u) = url {
            return Ok(Repo(SqliteConnectOptions::from_str(&u.0)?.create_if_missing(true).connect().await?));
        } else {
            return Ok(Repo(SqliteConnectOptions::from_str(":memory:")?.connect().await?));
        }
    }
    
    /// Initialize the database by creating tables etc. 
    pub async fn init(&mut self) -> Result<()> {
        let stmt = "CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name CHAR(50) NOT NULL,
            cents INTEGER NOT NULL,
            date TEXT,
            category CHAR(50),
            description TEXT
        )";

        let cnt = sqlx::query(stmt).execute(&mut self.0).await?;

        debug!("{} rows affected", cnt.rows_affected());
        Ok(())
    }
    
    pub async fn insert(&mut self, record: &Record) -> Result<()> {
        let stmt = "INSERT INTO records (name, cents, date, category, description) 
            VALUES (?,?,?,?,?)";
        
        let cnt = sqlx::query(stmt)
            .bind(&record.name)
            .bind(record.cents)
            .bind(record.date.to_string())
            .bind(&record.category)
            .bind(&record.description)
            .execute(&mut self.0).await?;
        
        debug!("{} rows affected", cnt.rows_affected());
        Ok(())
    }
    
    pub async fn list_all(&mut self) -> Result<Vec<Record>> {
        let stmt = "SELECT * FROM records";
        let mut rows = sqlx::query(stmt).fetch(&mut self.0);

        let mut results = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let id: i32 = row.try_get("id")?; 
            let name: &str = row.try_get("name")?; 
            let cents: i32 = row.try_get("cents")?; 
            let date: &str = row.try_get("date")?; 
            let category: &str = row.try_get("category")?; 
            let description: &str = row.try_get("description")?; 
            results.push(Record { id, name: String::from(name),cents, date: NaiveDate::parse_from_str(date, DB_DATE_FMT)?, category: String::from(category), description: String::from(description) });
        }
        
        Ok(results)
    }

    pub fn modify(&self, record: &Record) -> Result<Record> {
        todo!()  
    }

    pub fn rm(&self, record: &Record) -> Result<()> {
        todo!()
    }

    pub fn find() {
        todo!()
    }
}

#[cfg(test)]
mod test_repo {
    use super::Record;
    use chrono::Local;
    
    fn mk_record(num_records: i32) -> Vec<Record> {
 
        let num_records = 100;
        let mut records = Vec::with_capacity(100);
        
        // TODO: maybe random generated record fields 
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
        records
    }

    #[tokio::test]
    async fn test_insert() {
        let records = mk_record(100);
        let repo = super::Repo::new(None).await;
        assert!(repo.is_ok());
        let mut repo = repo.unwrap();
        repo.init().await.unwrap();

        for r in records.iter() {
            if let Err(e) = repo.insert(r).await {
                eprintln!("Insert failed {e}");
            }
        }
        
        let ls_result = repo.list_all().await.unwrap();
        for (i, rr) in ls_result.iter().enumerate() {
            assert_eq!(rr, records.get(i).unwrap())
        }
    }
}

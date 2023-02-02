use std::fmt::Display;

use anyhow::{Result, Ok};
use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::model::Record;

struct Repo(SqliteConnection);
struct URL(String);

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

    pub fn new(url: Option<URL>) -> Result<Self> {
        let conn;
        if let Some(u) = url {
            conn = SqliteConnection::establish(&u.0)?;
        } else {
            conn = SqliteConnection::establish(":memory:")?;
        }
        Ok(Repo(conn))
    }

    pub fn init(&self) -> Result<()> {
        todo!()  
    }

    pub fn insert(&self, record: &Record) -> Result<()> {

        todo!()  
    }
    
    pub fn list_all(&self) -> Result<Vec<Record>> {
            
        todo!()  
    }

    pub fn modify(&self, record: &Record) -> Result<Record> {

        todo!()  
    }

}

#[cfg(test)]
mod test_repo {
    use super::Record;
    use chrono::Local;

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
    
        for r in records.iter() {
            repo.insert(r).unwrap();
        }
        
        let ls_result = repo.list_all().unwrap();
        for (i, rr) in ls_result.iter().enumerate() {
            assert_eq!(rr, records.get(i).unwrap())
        }
    }
}

use chrono::NaiveDate;


/// Data abstraction for a single record
pub struct Record {
    pub id: i32,
    pub name: String,
    pub cents: i32,
    pub date: NaiveDate,
    pub category: String,
    pub description: String,
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
        LsQueryOptions { id: None, fuzzy: None, date_from: None, date_to: None, category: None }
    }

    /// Set id for QueryOption
    pub fn id(&mut self, id: i32) -> &Self {
        self.id = Some(id);
        self
    }
}

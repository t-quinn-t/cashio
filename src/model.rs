use std::fmt::Display;

use anyhow::Result;
use chrono::prelude::*;
use cli_table::{Cell, Table, Style};

struct Tbl(Vec<Record>);

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


pub fn print_table(records: Vec<Record>) -> Result<()>{
    let mut t = Vec::new();
    for record in records {
        let mut amount_str = (record.cents/100).to_string();
        amount_str.push('.');
        amount_str.push_str(&(record.cents%100).to_string());
        t.push(vec![
            record.id.cell(),
            amount_str.cell().justify(cli_table::format::Justify::Right),
            record.date.to_string().cell(),
            record.category.to_string().cell().justify(cli_table::format::Justify::Right),
            record.description.to_string().cell(),
        ]);
    }

    let display = t.table()
       .title(vec!["Id", "Amount", "Date", "Category", "Description"])
       .bold(true)
       .display()?;
        
    print!("{}", display);
    Ok(())
}


use chrono::prelude::*;
/*
 * @Author: Quinn Tao @t-quinn-t 
 * @Date: 2023-01-06 14:01:00 
 * @Last Modified by: Quinn Tao
 * @Last Modified time: 2023-01-06 14:02:06
 */
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCmd)   
}

/// Add a record to current database
#[derive(Args)]
pub struct AddCmd {

    /// Name of the record 
    name: String,

    /// Amount of the record
    amount: i32,

    #[arg(short, long)]
    date: Option<String>,

    #[arg(short, long)]
    category: Option<String>,
}
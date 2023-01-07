/*
 * @Author: Quinn Tao @t-quinn-t 
 * @Date: 2023-01-06 15:44:40 
 * @Last Modified by: Quinn Tao
 * @Last Modified time: 2023-01-06 16:09:30
 */
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(AddCmd),
    Ls(ListCmd),
    Mod(ModCmd),
    Rm(RmCmd),
}

/// Add a record to current database
#[derive(Args, Debug)]
pub struct AddCmd {

    /// Name of the record 
    name: String,

    /// Amount of the record
    amount: i32,

    /// Date of this record 
    /// 
    /// Supporting format:
    /// 
    /// - YYYY-mm-dd OR mm/dd/YYYY
    /// 
    /// - mm-dd OR mm/dd, defaulting to current year 
    /// 
    /// - dd defaulting to current year/month 
    date: Option<String>,

    /// The category for this record
    #[arg(short, long)]
    category: Option<String>,

    /// Full description of this record
    #[arg(short, long)]
    description: Option<String>,
}

/// List records in current database, default to list records in current month
#[derive(Args, Debug)]
pub struct ListCmd {
    /// Fuzzy find search string, applied to names and descriptions
    query: Option<String>,

    /// Specify a category
    #[arg(short, default_value = "default")]
    category: Option<String>, 

    /// Specify single month 
    #[arg(short, default_value="current month", conflicts_with="time")]
    month: Option<String>,

    /// Specify single year 
    #[arg(short, default_value="current year", conflicts_with="time")]
    year: Option<String>,

    /// Time range from
    #[arg(long, group="time")]
    from: Option<String>,

    /// Time range to
    #[arg(long, requires="from", group="time")]
    to: Option<String>,
}

#[derive(Args, Debug)]
pub struct ModCmd {
    /// Id of the record to modify
    id: i32,

    /// Name of the record 
    #[arg(short, long)]
    name: Option<String>,

    /// Amount of the record
    #[arg(short, long)]
    amount: Option<i32>,

    /// Date of this record 
    /// 
    /// Supporting format:
    /// 
    /// - YYYY-mm-dd OR mm/dd/YYYY
    /// 
    /// - mm-dd OR mm/dd, defaulting to current year 
    /// 
    /// - dd defaulting to current year/month 
    #[arg(short, long)]
    date: Option<String>,

    /// The category for this record
    #[arg(short, long)]
    category: Option<String>,

    /// Full description of this record
    #[arg(long)]
    description: Option<String>,

    /// Force apply modification
    #[arg(short, long)]
    force: Option<bool>
}

#[derive(Args, Debug)]
pub struct RmCmd {
    /// Id of the record to remove
    id: i32
}




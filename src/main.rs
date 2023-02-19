/*
 * @Author: Quinn Tao @t-quinn-t 
 * @Date: 2023-01-06 15:50:11 
 * @Last Modified by: Quinn Tao
 * @Last Modified time: 2023-01-06 23:42:31
 */


use cashio::{cli::{Cli, Commands}, model::{Record, print_table}};
use chrono::{Local, NaiveTime, NaiveDate};
use clap::Parser;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Add(cmd) => {
            debug!("add command {:?}", cmd);
               
        },
        Commands::Ls(cmd) => {
            debug!("ls command {:?}", cmd);
        },
        Commands::Mod(cmd) => {
            debug!("mod command {:?}", cmd);
        },
        Commands::Rm(cmd) => {
            debug!("rm command {:?}", cmd);
        }
    }
}



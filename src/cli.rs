use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands
}

#[derive(Subcommand)]
pub enum Commands {

}

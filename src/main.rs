mod config;
use std::default;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
   /*  #[clap(subcommand)]
    subcmd: Option<SubCommand>, */

    /// Reconfigure the program
    #[arg(name = "reconfig", short, long, default_value = "false")]
    reconfig: bool,
}

/* #[derive(Parser)]
enum SubCommand {
    #[clap(name = "add")]
    Add,
    #[clap(name = "remove")]
    Remove,
    #[clap(name = "list")]
    List,
} */

fn main() {
    let args = Cli::parse();
    let config = config::read_config(args.reconfig);
    

    
    config::write_config(&config);
}

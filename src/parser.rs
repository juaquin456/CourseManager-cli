use clap::{ Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommand>,

    /// Reconfigure the program
    #[arg(name = "reconfig", short, long, default_value = "false")]
    pub reconfig: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Get{
        #[command(subcommand)]
        obj: Object,
    },
    Add{
        #[command(subcommand)]
        obj: Object,
    },
    Remove{
        #[command(subcommand)]
        obj: Object,
    },
    List{
        #[command(subcommand)]
        obj: CollObject,
    },
}

#[derive(Subcommand)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Cycle {
        #[arg(name = "id", short, long)]
        id: Option<u16>,
    },
    Course {
        #[arg(name = "id", short, long)]
        id: Option<u16>,
        #[arg(name = "name", short, long)]
        name: Option<String>,
    },
}


#[derive(Subcommand)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollObject {
    Cycles {},
    Courses {
        #[arg(name = "id", short, long)]
        id: Option<u16>,
    },
}
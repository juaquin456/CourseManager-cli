use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Clone)]
pub struct Cycle {
    pub age: u16,
    pub semester: u8,
}

#[derive(Args)]
pub struct Course {
    pub cycle_id: u16,
    pub name: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[command(subcommand)]
        entity: Entity,
    },
    Remove {
        #[command(subcommand)]
        entity: Entity
    },
    List {
        #[arg(value_enum)]
        entity: Entities
    },
    Go {
        #[command(subcommand)]
        entity: Entity,
    },
    Summary {
        #[command(subcommand)]
        entity: Entity,
    },
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Entities {
    Cycles,
    Courses,
}

#[derive(Subcommand)]
pub enum Entity {
    Cycle(Cycle),
    Course(Course),
}
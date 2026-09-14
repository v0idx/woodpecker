use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about=None)] // Read values from Cargo.toml
struct Cli {
    /// What action to enact on the todo list
    #[command(subcommand)]
    action: Option<Actions>,
}

#[derive(Subcommand)]
enum Actions {
    /// Add an item to the todo list
    Add(AddArgs),
    /// Remove the specified item from the todo list
    Remove(RmArgs),
    /// Display further information about a list item
    Info(InfoArgs),
    /// Modify the name and/or description of a list item
    Modify(ModifyArgs),
}

#[derive(Args)]
struct AddArgs {
    /// Name of the list item
    item_name: String,
    /// Optionally provide an item description for further information
    item_details: Option<String>,
}

#[derive(Args)]
struct RmArgs {
    /// ID of the item to remove
    item_id: u16,
}

#[derive(Args)]
struct InfoArgs {
    /// ID of the item to retrieve further information of
    item_id: u16,
}

#[derive(Args)]
struct ModifyArgs {
    /// ID of the item to modify
    item_id: u16,
    /// Which part of the item to modify, leave blank to modify both
    #[arg(value_enum)]
    target: Option<Targets>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Targets {
    /// Modify the `name` of the list item
    Name,
    /// Modify the `description` of the list item
    Description,
}

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Some(v) => match v {
            Actions::Add(_args) => {
                println!("add used");
            }
            Actions::Remove(_args) => {
                println!("remove used");
            }
            Actions::Info(_args) => {
                println!("info used");
            }
            Actions::Modify(_args) => {
                println!("modify used");
            }
        },
        None => {
            println!("listing items");
        }
    }
}

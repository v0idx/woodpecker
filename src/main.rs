use clap::{Args, Parser, Subcommand, ValueEnum};
use woodpecker_todo::TodoList;

#[derive(Parser)]
#[command(version, about, long_about=None)] // Read values from Cargo.toml
struct Cli {
    /// What action to enact on the todo list
    #[command(subcommand)]
    action: Option<Actions>,

    /// Generate an example configuration file
    #[arg(long, exclusive(true))]
    generate_config: bool,

    /// Set config file path
    #[arg(short, long, exclusive(true))]
    config_path: Option<String>,

    /// Optionally just add straight to the list! (Details unsupported when using this shorthand)
    list_item: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
enum Actions {
    /// Add an item to the todo list
    Add(AddArgs),
    /// Remove the specified item from the todo list
    Remove(RmArgs),
    /// Display further information about a list item
    Info(InfoArgs),
    /// Modify the name and/or description of a list item
    Modify(ModifyArgs),
    /// Display the current list, also occurs when left blank
    List,
    /// Empty list
    Clear,
}

#[derive(Args, Debug)]
struct AddArgs {
    /// Name of the list item
    item_name: Vec<String>,
    /// Optionally provide an item description for further information
    #[arg(short,long,num_args(1..))]
    details: Option<Vec<String>>,
}

#[derive(Args, Debug)]
struct RmArgs {
    /// ID of the item to remove
    item_id: u16,
}

#[derive(Args, Debug)]
struct InfoArgs {
    /// ID of the item to retrieve further information of
    item_id: u16,
}

#[derive(Args, Debug)]
struct ModifyArgs {
    /// ID of the item to modify
    item_id: u16,
    /// Which part of the item to modify, leave blank to modify both
    #[arg(value_enum)]
    target: Option<Targets>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Targets {
    /// Modify the `name` of the list item
    Name,
    /// Modify the `description` of the list item
    Description,
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list: TodoList = TodoList::new();
    // TODO: Rethink this ordering, currently required to be able to load a configuration file to create a configuration file
    // Seems shortsighted
    if cli.generate_config {
        println!("Generating example configuration file, ignoring any other arguments!");
        todo_list.generate_config();
    } else if let Some(config_path) = cli.config_path {
        todo_list.set_config(config_path);
    } else if let Some(list_item) = cli.list_item {
        todo_list.add_item(&list_item, &None);
        todo_list.write_list();
    } else {
        match &cli.action {
            Some(v) => {
                match v {
                    Actions::Add(args) => {
                        todo_list.add_item(&args.item_name, &args.details);
                    }
                    Actions::Remove(args) => {
                        todo_list.remove_item(args.item_id);
                    }
                    Actions::Info(args) => {
                        todo_list.item_info(args.item_id);
                    }
                    Actions::Modify(args) => {
                        let mode = match args.target {
                            Some(v) => match v {
                                Targets::Name => 1,
                                Targets::Description => 2,
                            },
                            None => 3,
                        };
                        todo_list.modify_item(args.item_id, mode);
                    }
                    Actions::List => {
                        todo_list.display_list();
                    }
                    Actions::Clear => {
                        todo_list.clear_list();
                    }
                }
                todo_list.write_list();
            }
            None => {
                todo_list.display_list();
            }
        }
    }
}

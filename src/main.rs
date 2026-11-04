use app_dirs2::*;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::Path;
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

fn config_gen() {
    // Need to decide whether to copy a bundled .conf OR to refactor this to then serialize defaults to a .conf
    // Leaning towards the second option as it will provide more flexibility and can be done with serde most likely
    // Also need to decide upon a preferred layout for .conf, focusing on readability and ease-of-use
    // May need to roll own serialize/deserialize for nice looking .conf if serde doesn't play nicely

    /*
       What do we need to store in the .conf realistically
       - default behaviours
       - config file-path
       - list file-path

       // In Future \\
       - network location for retrieval
       - sync times (cron-like?)
    */

    // Will move configuration logic to seperate source
    let path = match app_root(AppDataType::UserConfig, &woodpecker_todo::APP_INFO) {
        Ok(v) => v,
        Err(e) => panic!("Could not create/locate user config directory,\n{e}"),
    };

    let path = path.join(Path::new("woodpecker.conf"));
    let path_display = path.display();

    println!("We will place the config file @ {path_display}");
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list: TodoList = TodoList::new();

    if cli.generate_config {
        println!("Generating example configuration file, ignoring any other arguments!");
        config_gen();
    } else if cli.list_item.is_some() {
        todo_list.add_item(&cli.list_item.unwrap(), &None);
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

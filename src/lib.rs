use app_dirs2::*;
use chrono::{DateTime, Local};
use rmp_serde::{Deserializer, Serializer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Write, prelude::*, stdin, stdout};
use std::path::Path;
use toml_scaffold::TomlScaffold;

const APP_INFO: AppInfo = AppInfo {
    name: "woodpecker",
    author: "Eloise Nash",
};

enum WoodpeckerError {
    Success,
    CreateFileError,
    WriteFileError,
}

fn write_to_file(buf: &[u8], loc: &Path) -> WoodpeckerError {
    let mut file = match File::create(loc) {
        Ok(f) => f,
        Err(_) => return WoodpeckerError::CreateFileError,
    };

    match file.write_all(buf) {
        Ok(_) => return WoodpeckerError::Success,
        Err(_) => return WoodpeckerError::WriteFileError,
    }
}

pub fn gen_def_config() {
    let conf = Config::new(
        &app_root(AppDataType::UserConfig, &APP_INFO)
            .unwrap()
            .join(Path::new("woodpecker.conf"))
            .display()
            .to_string(),
        &app_root(AppDataType::UserData, &APP_INFO)
            .unwrap()
            .join(Path::new("list.mpk"))
            .display()
            .to_string(),
    );

    let toml = conf.to_scaffold().unwrap();
    let path = Path::new(&conf.file.config_path);

    match write_to_file(toml.as_bytes(), path) {
        WoodpeckerError::Success => {
            println!(
                "Successfully written config file to {}!",
                conf.file.config_path
            )
        }
        WoodpeckerError::CreateFileError => {
            println!("Couldn't create file at {}", conf.file.config_path)
        }
        WoodpeckerError::WriteFileError => {
            println!("Couldn't write to file at {}", conf.file.config_path)
        }
    }
}

#[derive(Serialize, Deserialize, TomlScaffold, JsonSchema)]
struct Config {
    file: FileConfig,
    personalisation: PersonaliseConfig,
}

#[derive(Serialize, Deserialize, TomlScaffold, JsonSchema)]
struct FileConfig {
    config_path: String,
    list_path: String,
}

#[derive(Serialize, Deserialize, TomlScaffold, JsonSchema)]
struct PersonaliseConfig {
    /// Configuration Struct defining strings to change, values wrapped in '~' are used in interporation
    /// These aren't required, but can be used if you wish. Only values wrapped are available per-string.
    empty_list: String,
    remove_200: String,
    remove_404: String,
    clear_list: String,
    modify_404: String,
    modify_200: String,
    modify_name_404: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new(&"DEFAULT".to_string(), &"DEFAULT".to_string())
    }
}

impl Config {
    fn new(conf_path: &String, list_path: &String) -> Config {
        let def_file = FileConfig::new(conf_path, list_path);

        return Config {
            file: def_file,
            personalisation: PersonaliseConfig::new(),
        };
    }
}

impl FileConfig {
    fn new(conf_path: &String, list_path: &String) -> FileConfig {
        return FileConfig {
            config_path: conf_path.to_string(),
            list_path: list_path.to_string(),
        };
    }
}

impl PersonaliseConfig {
    fn new() -> PersonaliseConfig {
        return PersonaliseConfig {
            empty_list: "No items in list, you're all caught up!!".to_string(),
            remove_200: "Peck Peck! Removed item at ~item_id~, with name: ~item~.".to_string(),
            remove_404: "Couldn't find item at ~item_id~, try a different id?".to_string(),
            clear_list: "Time to start fresh!".to_string(),
            modify_404: "How did we get here? Unspecified mode entered.".to_string(),
            modify_200: "Successfully edited item at ~item_id~!".to_string(),
            modify_name_404: "Please ensure a name is entered, items cannot have no name!"
                .to_string(),
        };
    }
}

/// Representation of a List Item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListItem {
    // Short name describing the TODO item
    name: String,
    // Longer description of tasks involved, further information, etc.
    description: String,
    // Date the item was made
    creation_date: DateTime<Local>,
    // If the item was modified, then this will be the `DateTime<Local>` it was, otherwise `None`
    modified: Option<DateTime<Local>>,
}

/// Holds all current list information
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    // BTreeMap to allow for easier ordered display of list items from index
    list: BTreeMap<u16, ListItem>,
    // Current location of the `list.mpk`
    file_path: String,
    // Next available id, reset to 1 when list is emptied.
    next_id: u16,
    // Config file path
    config_path: String,
    // Configuration for current instance
    #[serde(skip_serializing, skip_deserializing)]
    config: Config,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    /// # Initialises the current list.
    ///
    /// If a list is found at the standard location discovered by [app_dirs2](https://crates.io/crates/app_dirs2)
    /// then it will be loaded into memory
    ///
    /// Otherwise a new list will be created, including the file required to store it on disk.
    pub fn init(&mut self) {
        let path = match app_root(AppDataType::UserData, &APP_INFO) {
            Ok(v) => v,
            Err(e) => panic!("Could not create/locate user data directory,\n{e}"),
        };
        let path = path.join(Path::new("list.mpk"));

        let path_display = path.display();

        if path.exists() {
            let mut _file = match File::open(&path) {
                Ok(file) => file,
                Err(e) => panic!("Couldn't open file at {path_display}\n{e}"),
            };

            match &fs::read(&path) {
                Ok(buf) => {
                    if buf.is_empty() {
                        let _file = match File::create(&path) {
                            Ok(file) => file,
                            Err(e) => panic!("Couldn't create file at {path_display}\n{e}"),
                        };
                        self.list = BTreeMap::new();
                        self.file_path = path_display.to_string();
                        self.config_path = "DEFAULT".to_string();
                        self.next_id = 1;
                    } else {
                        let mut de = Deserializer::new(&buf[..]);
                        match <TodoList as Deserialize>::deserialize(&mut de) {
                            Ok(v) => {
                                self.list = v.list;
                                self.file_path = v.file_path;
                                self.config_path = v.config_path;
                                self.next_id = v.next_id;
                            }
                            Err(e) => panic!("Couldn't deserialze list file, aborting...\n{e}"),
                        }
                    }
                }
                Err(e) => panic!("Couldn't read file at {path_display}\n{e}"),
            };
        } else {
            let _file = match File::create(&path) {
                Ok(file) => file,
                Err(e) => panic!("Couldn't create file at {path_display}\n{e}"),
            };

            self.list = BTreeMap::new();
            self.file_path = path_display.to_string();
            self.config_path = "DEFAULT".to_string();
            self.next_id = 1;
        }

        match self.config_path.as_str() {
            // Used when a config file hasn't been set yet
            "DEFAULT" => {
                self.config = Config::new(&self.config_path, &self.file_path);
            }
            // Default Case
            _ => {
                let mut file = match File::open(&self.config_path) {
                    Ok(f) => f,
                    Err(e) => panic!(
                        "Config file path is set, but couldn't open {}!\n{e}",
                        self.config_path
                    ),
                };
                let mut conf_str = String::new();
                match file.read_to_string(&mut conf_str) {
                    Ok(_) => (),
                    Err(e) => panic!("Couldn't read from {}!\n{e}", self.config_path),
                };

                self.config = match toml::from_str(&conf_str) {
                    Ok(v) => v,
                    Err(_) => panic!(
                        "Couldn't deserialize config file! Is it laid out correctly? Consider generating an example with --generate-config"
                    ),
                }
            }
        }
    }

    /// # Creates a new TodoList object
    ///
    /// Initialises the list completely empty, 'zeroing' the object, before initialisations are made.
    pub fn new() -> TodoList {
        let mut ret_list = TodoList {
            list: BTreeMap::new(),
            file_path: String::from(""),
            config_path: String::from(""),
            config: Config::default(),
            next_id: 1,
        };
        TodoList::init(&mut ret_list);
        ret_list
    }

    /// # Generates a default configuration file
    pub fn generate_config(&mut self) {
        let conf_path: String;
        if self.config_path.as_str() == "DEFAULT" {
            conf_path = match app_root(AppDataType::UserConfig, &APP_INFO) {
                Ok(v) => v.join(Path::new("woodpecker.conf")).display().to_string(),
                Err(e) => panic!("Could not create/locate user config directory,\n{e}"),
            };
            self.config_path = conf_path.to_string();
            self.write_list();
        } else {
            conf_path = self.config_path.clone();
        }

        let conf = Config::new(&conf_path, &self.file_path);
        let toml = conf.to_scaffold().unwrap();

        let path = Path::new(&conf_path);

        match write_to_file(toml.as_bytes(), path) {
            WoodpeckerError::Success => {
                println!("Successfully written config file to {conf_path}!")
            }
            WoodpeckerError::CreateFileError => println!("Couldn't create file at {conf_path}"),
            WoodpeckerError::WriteFileError => println!("Couldn't write to file at {conf_path}"),
        }
    }

    /// # Set's the TodoList config path
    ///
    /// The value set here is stored within the *.mpk file
    pub fn set_config(&mut self, conf: String) {
        self.config_path = conf;
        println!("woodpecker config filepath set to {}", self.config_path);
        self.write_list()
    }

    /// # Writes the list object to disk
    ///
    /// Writes to the path stored within the object, as `file_path`
    pub fn write_list(&self) {
        let path = Path::new(&self.file_path);
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(e) => panic!("Couldn't create {}\n({e})", self.file_path),
        };

        let mut to_write = Vec::new();
        match self.serialize(&mut Serializer::new(&mut to_write)) {
            Ok(_) => (),
            Err(e) => panic!("Couldn't serialize list!\n{e}"),
        }

        match file.write_all(&to_write) {
            Ok(_) => (),
            Err(e) => panic!("Couldn't write to {}\n {e}", self.file_path),
        }
    }

    /// # Displays all list items, in key order.
    pub fn display_list(&self) {
        if self.next_id != 1 {
            for (id, item) in &self.list {
                println!("{}: {}", id, item.name);
            }
        } else {
            println!("No items in list, you're all caught up!!");
        }
    }

    /// # Adds a [ListItem] to the list
    pub fn add_item(&mut self, item_name: &[String], item_description: &Option<Vec<String>>) {
        let desc = match item_description {
            Some(v) => &v.join(" "),
            None => &"".to_string(),
        };
        let name_join = item_name.join(" ");
        let item = ListItem {
            name: name_join,
            description: desc.to_string(),
            creation_date: Local::now(),
            modified: None,
        };

        self.list.insert(self.next_id, item);

        self.next_id += 1;
    }

    /// # Removes a given [ListItem] from the list
    pub fn remove_item(&mut self, item_id: u16) {
        let item = match self.list.remove(&item_id) {
            Some(v) => v.name,
            None => {
                println!("Couldn't find item at {item_id}, try a different id?");
                return;
            }
        };
        println!("Peck Peck! Removed item at {item_id}, with name: {item}.",);

        if self.sanitise_ids() {
            println!("You're all caught up, resetting ids to: {}", self.next_id);
        }
    }

    fn sanitise_ids(&mut self) -> bool {
        if self.list.is_empty() {
            self.next_id = 1;
            return true;
        } else {
            return false;
        }
    }

    /// # Retrieve information about the requested [ListItem]
    pub fn item_info(&self, item_id: u16) {
        let item = match self.list.get(&item_id) {
            Some(v) => v,
            None => {
                println!("No item found with id {}", item_id);
                return;
            }
        };
        println!("ID: {}", item_id);
        println!("Name: {}", item.name);
        println!("Description: {}", item.description);
        println!("Date Created: {}", item.creation_date);
        match item.modified {
            Some(v) => println!("Item modified on: {}", v),
            None => println!("Unmodified item"),
        }
    }

    fn handle_input() -> String {
        let mut input: String = String::new();
        stdout().flush().unwrap();
        match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(e) => {
                println!("Unable to read input, aborting.\n{e}");
                "".to_string()
            }
        }
    }

    /// Clears all [ListItem]'s from the list
    pub fn clear_list(&mut self) {
        self.list.clear();
        println!("Time to start fresh!");
        self.sanitise_ids();
    }

    /// Modify a given [ListItem]'s name or description
    pub fn modify_item(&mut self, item_id: u16, mode: u16) {
        let repl_item: Option<ListItem> = match mode {
            // Changing name of item.
            1 => {
                print!("Please enter new item name: ");
                let input: String = TodoList::handle_input();
                if input.is_empty() {
                    None
                } else {
                    match self.list.get(&item_id) {
                        Some(v) => Some(ListItem {
                            name: input.trim().to_string(),
                            description: v.description.clone(),
                            creation_date: v.creation_date,
                            modified: Some(Local::now()),
                        }),
                        None => {
                            println!("Unable to retrieve item at {}, aborting", item_id);
                            None
                        }
                    }
                }
            }
            // Changing description of item.
            2 => {
                print!("Please enter new item description: ");
                let input: String = TodoList::handle_input();

                match self.list.get(&item_id) {
                    Some(v) => Some(ListItem {
                        name: v.name.clone(),
                        description: input.trim().to_string(),
                        creation_date: v.creation_date,
                        modified: Some(Local::now()),
                    }),
                    None => {
                        println!("Unable to retrieve item at {}, aborting.", item_id);
                        None
                    }
                }
            }
            // Changing both.
            3 => {
                print!("Please enter item name: ");
                let name: String = TodoList::handle_input();
                if name.is_empty() {
                    None
                } else {
                    print!("Please enter item description: ");
                    let desc: String = TodoList::handle_input();
                    match self.list.get(&item_id) {
                        Some(v) => Some(ListItem {
                            name: name.trim().to_string(),
                            description: desc.trim().to_string(),
                            creation_date: v.creation_date,
                            modified: Some(Local::now()),
                        }),
                        None => {
                            println!("Unable to retrieve item at {}, aborting.", item_id);
                            None
                        }
                    }
                }
            }
            _ => {
                println!("How did we get here? Unspecified mode entered.");
                None
            }
        };

        match repl_item {
            Some(v) => {
                match self.list.remove(&item_id) {
                    Some(_) => (),
                    None => {
                        println!("Error modifying item at {}, aborting.", item_id);
                        return;
                    }
                }
                self.list.insert(item_id, v);
                println!("Successfully edited item at {}!", item_id);
            }
            None => {
                println!("Please ensure a name is entered, items cannot have no name!");
            }
        }
    }
}

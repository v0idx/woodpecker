use app_dirs2::*;
use chrono::{DateTime, Local};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Write, stdin, stdout};
use std::path::Path;

const APP_INFO: AppInfo = AppInfo {
    name: "woodpecker",
    author: "Eloise Nash",
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListItem {
    name: String,
    description: String,
    creation_date: DateTime<Local>,
    modified: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    list: BTreeMap<u16, ListItem>,
    file_path: String,
    next_id: u16,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
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
                        self.next_id = 1;
                    } else {
                        let mut de = Deserializer::new(&buf[..]);
                        match <TodoList as Deserialize>::deserialize(&mut de) {
                            Ok(v) => {
                                self.list = v.list;
                                self.file_path = v.file_path;
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
            self.next_id = 1;
        }
    }

    pub fn new() -> TodoList {
        let mut ret_list = TodoList {
            list: BTreeMap::new(),
            file_path: String::from(""),
            next_id: 1,
        };
        TodoList::init(&mut ret_list);
        ret_list
    }

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

    pub fn display_list(&self) {
        if self.next_id != 1 {
            for (id, item) in &self.list {
                println!("{}: {}", id, item.name);
            }
        } else {
            println!("No items in list, you're all caught up!!");
        }
    }

    pub fn add_item(&mut self, item_name: &String, item_description: &Option<String>) {
        let desc = match item_description {
            Some(v) => v,
            None => &"".to_string(),
        };
        let item = ListItem {
            name: item_name.to_string(),
            description: desc.to_string(),
            creation_date: Local::now(),
            modified: None,
        };

        self.list.insert(self.next_id, item);

        self.next_id += 1;
    }

    pub fn remove_item(&mut self, item_id: u16) {
        let item = match self.list.remove(&item_id) {
            Some(v) => v,
            None => {
                println!("Couldn't find item at {item_id}, try a different id?");
                return;
            }
        };
        println!(
            "Peck Peck! Removed item at {item_id}, with name: {}.",
            item.name
        );

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

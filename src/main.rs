use std::{env, process, collections, fs, io, path, error};

use serde_json;

fn get_db_path() -> path::PathBuf {
    env::home_dir().unwrap().join(path::Path::new(".todo"))
}

struct ToDo {
    map: collections::HashMap<String, bool>,
}

impl ToDo {
    fn insert(&mut self, key: String) {
        // insert new item in map with default value as true
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(get_db_path())?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn new() -> Result<ToDo, io::Error> {
       let f = fs::OpenOptions::new()
           .write(true)
           .create(true)
           .read(true)
           .open(get_db_path())?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(ToDo { map }),
            Err(e) if e.is_eof() => Ok(ToDo {
                map: collections::HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
    
        }
    }
}

fn main() {
    let action: String = match env::args().nth(1) {
        Some(action) => action,
        None => {
            println!("Please specify the action.");
            process::exit(1);
        }
    };
    let item: String = match env::args().nth(2) {
        Some(item) => item,
        None => {
            println!("Please specify the item.");
            process::exit(1)
        }
    };

    let mut todo = ToDo::new().unwrap();

    if action ==  "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("Error Occured : {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("Error Occured : {}", why),
            }
            None => println!("'{}' not presern in the list", item),
        };
    }
}
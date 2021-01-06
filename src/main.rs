use std::{env, process, collections, fs, io, path, io::{Read}, str::{FromStr}};

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

    fn save(self) -> Result<(), io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        fs::write(get_db_path(), content)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn new() -> Result<ToDo, io::Error> {
       let mut f = fs::OpenOptions::new()
           .write(true)
           .create(true)
           .read(true)
           .open(get_db_path())?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: collections::HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(ToDo { map })
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
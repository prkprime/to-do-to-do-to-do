use std::{env, process, collections, fs, io};

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
        let mut db_path: String = match env::home_dir() {
            Some(path) => path.into_os_string().into_string().unwrap(),
            None => "".to_string(),
        };
        db_path.push_str("/.todo");
        fs::write(db_path, content)
    }
}

fn main() {
    let action: String = match env::args().nth(1) {
        Some(action) => action,
        None => {
            println!("Please specify the action. Exiting...");
            process::exit(0);
        }
    };
    let item: String = match env::args().nth(2) {
        Some(item) => item,
        None => {
            println!("Please specify the item. Exiting...");
            process::exit(0)
        }
    };
}
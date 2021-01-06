use std::{env, process};

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

    println!("{} {}", action, item);
}

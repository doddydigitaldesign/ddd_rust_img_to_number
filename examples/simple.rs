use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use ddd_rust_img_to_number::img_to_number;

fn main() {
    println!("Path to the file:");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();

    let result = img_to_number(&mut input_string);

    match result {
        Ok(value) => {
            println!("Result: {}", value);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    println!("Please enter a file.");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)?;

    let tmp = input_string.trim();
    let path_input = Path::new(&tmp);
    let file_content = std::fs::read(path_input.as_os_str());

    match file_content {
        Ok(res) => {
            let mut hasher = DefaultHasher::new();
            res.hash(&mut hasher);
            let finished_hash = hasher.finish();

            println!("Result:");
            println!("{}", finished_hash);
            return Ok(());
        }
        Err(err) => {
            println!("Error: {}", err);
            return Err(err);
        }
    }
}

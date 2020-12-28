use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{prelude::*, Error},
    path::Path,
};
pub fn img_to_number(path_str: &mut String) -> Result<u64, Error> {
    let tmp = path_str.trim();
    let path_input = Path::new(&tmp);

    let path_canonicalized = path_input.canonicalize()?;
    let path_os_string = path_canonicalized.as_os_str();
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path_os_string)?;
    f.read_to_end(&mut buffer)?;

    let mut hasher = DefaultHasher::new();
    buffer.hash(&mut hasher);
    let finished_hash = hasher.finish();
    return Ok(finished_hash);
}

#[cfg(test)]
mod tests {
    use super::img_to_number;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn it_parses_correctly() {
        let mut mock_file = String::from("./mock/my_image.webp");
        let result = img_to_number(&mut mock_file);
        assert_eq!(result.unwrap(), 9373154758004062098u64);
    }
}

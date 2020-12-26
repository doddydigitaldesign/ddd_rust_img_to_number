extern crate image;

use image::GenericImageView;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let count = env::args().count();
    let file = if count >= 2 {
        let file_arg = env::args().nth(1).unwrap();
        println!("File: {}", file_arg);
        file_arg
    } else {
        panic!("Please enter a file.")
    };

    let file_name = file.split(" ").find(|x| x != &"--file").unwrap();
    let img = image::open(&Path::new(&file_name)).unwrap();

    let result = img.pixels().into_iter().fold(0, |acc, (_x, _y, pixel)| {
        acc + (pixel[0] as u32) + (pixel[1] as u32) + (pixel[2] as u32) + (pixel[3] as u32)
    });

    println!("Result: {}", result);
    generate_output(result);
}

fn generate_output(res: u32) {
    if !Path::new("output/").exists() {
        fs::create_dir("output/").unwrap();
    }
    let mut out_file: String = String::from("output/");
    out_file.push_str(&res.to_string());
    out_file.push_str(".txt");
    fs::write(&out_file, res.to_string()).unwrap();
}

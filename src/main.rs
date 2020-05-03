extern crate image;

use image::GenericImageView;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let file = if env::args().count() == 2 {
        println!("File: {}", env::args().nth(1).unwrap());
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let img = image::open(&Path::new(&file)).unwrap();

    let result = img.pixels().into_iter().fold(0, |acc, (_x, _y, pixel)| {
        acc + pixel[0] as u128 * pixel[1] as u128 * pixel[2] as u128 * pixel[3] as u128
    });

    println!("Result: {}", result);
    generate_output(result);
}

fn generate_output(res: u128) {
    if !Path::new("output/").exists() {
        fs::create_dir("output/").unwrap();
    }
    let mut out_file: String = String::from("output/");
    out_file.push_str(&res.to_string());
    out_file.push_str(".txt");
    fs::write(&out_file, res.to_string()).unwrap();
}

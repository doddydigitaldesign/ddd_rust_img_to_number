extern crate clap;
extern crate image;

// use clap::{App, Arg, SubCommand};
use image::GenericImageView;
use std::env;
use std::path::Path;

fn main() {
    let file = if env::args().count() == 2 {
        println!("File: {}", env::args().nth(1).unwrap());
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let img = image::open(&Path::new(&file)).unwrap();

    let result = img.pixels().into_iter().fold(0, |acc, (x, y, pixel)| {
        acc + ((x + 1) / (y + 1))
            + pixel[0] as u32
            + pixel[1] as u32
            + pixel[2] as u32
            + pixel[3] as u32
    });

    println!("Result: {}", result);
}

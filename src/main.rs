extern crate exif;
extern crate glob;

use self::glob::glob;
use std::{env, process};
use std::fs::{create_dir_all, rename, File};
use std::path::{Display, Path};

fn make_dirs(date_time: &str) {
    let mut split_date_time_spaces = date_time.split_whitespace();

    match split_date_time_spaces.next() {
        Some(e) => {
            let replace_date_hyphens = str::replace(e, "-", "/");
            let dir_to_create = "./photos/".to_owned() + &replace_date_hyphens;

            println!("{:?}", dir_to_create);

            create_dir_all(dir_to_create);
        },
        None => {
            println!("{:?}", "No dates exist.")
        }
    };

}

fn read_exif_date_data(image_path: &Display) {

    let image_path_str: &str = &image_path.to_string();
    let path = Path::new(image_path_str);

    let file = File::open(path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();

    match reader.get_field(exif::Tag::DateTime, false) {
        Some(data) => {
            let date_time: &str = &data.value.display_as(data.tag).to_string();
            make_dirs(&date_time);
        }
        None => {
            println!("{:?}", "No dates exist.")
        }
    };
}

fn read_photo_library(white_list_file_types: Vec<&str>, photos_dir_str: &str) {
    for file_type in &white_list_file_types {
        let glob_path = photos_dir_str.to_owned() + "**/*." + file_type.to_owned();
        for entry in glob(&glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let image_path: Display = path.display();
                    read_exif_date_data(&image_path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let photos_dir_str: &str = &args[1];

    if args.len() != 2 {
        println!("Did not input the right amount of arguments!  Just two please.");
        process::exit(1);
    } 

    let photos_dir_path = Path::new(photos_dir_str);
    let white_list_file_types: Vec<&str> = vec!["jpeg", "jpg", "JPEG", "JPG"];

    if photos_dir_path.is_dir() {
        read_photo_library(white_list_file_types, photos_dir_str);
    }
}
extern crate clap;
extern crate regex;
extern crate reqwest;

use std::path::Path;
use std::fs::*;
use std::io::Read;
use clap::{App, Arg};
use regex::Regex;

fn create_filename(dirname: &str, filename: &str) -> String {
    let result = [dirname, filename].join("/");
    let result = result + ".jpg";
    return result;
}

fn do_download(filename: &str, url: &str) {
    if ! Path::new(filename).exists() {
        let mut f = File::create(filename).unwrap();
        println!("{}", ["Download: ", filename].join(""));
        let mut res = reqwest::get(url).unwrap();
        res.copy_to(&mut f).unwrap();
    } else {
        println!("{}", ["Skipped: ", filename].join(""));
    }
}

fn imagenet_download(url_list: &str) {
    let mut contents = String::new();
    {
        let mut f = File::open(url_list).unwrap();
        f.read_to_string(&mut contents).unwrap();
    }
    let lines: Vec<&str> = contents.split("\n").collect();
    for line in lines {
        if line == "" {
            continue;
        }
        let re = Regex::new(" +").unwrap();
        let id_url: Vec<&str> = re.split(line).collect();
        let id = id_url[0];
        let url = id_url[1];
        let dirname_filename: Vec<&str> = id.split("_").collect();
        let dirname = dirname_filename[0];
        let filename = create_filename(dirname, dirname_filename[1]);
        if ! Path::new(&dirname).exists() {
            create_dir(&dirname).unwrap();
        }
        do_download(&filename, url);
    }
}

fn main() {
    let matches = App::new("imagenet-downloader")
        .version("0.0.1")
        .author("miyagaw61 <miyagaw61@gmail.com>")
        .about("ImageNet Downloader")
        .arg(Arg::with_name("url_list")
             .takes_value(true)
             .required(true)
             )
        .get_matches();
    imagenet_download(matches.value_of("url_list").unwrap());
}

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate serde;

pub mod fuse;

use std::io;
use std::path::{Path,PathBuf};
use std::fs::{read_dir,ReadDir,File};
use serde_json::{Value,Error};
use std::io::Read;

use fuse::*;


fn iter_dir(path: &Path, f: &mut FnMut(&Path)) {
    if let Ok(dir) = read_dir(path) {
        f(path);
        for d in dir {
            if let Ok(d) = d {
                iter_dir(&d.path(), f);
            }
        }
    }
}


enum Query {
    Class(String),
    Property(String,String),
    Error(String)
}

use Query::*;


fn parse_input(input: &str) -> Query {
    let s: Vec<&str> = input.split(".").collect();
    match s.len() {
        0 => Error("Input needs at least one word".to_string()),
        1 => Class(s[0].to_string()),
        2 => Property(s[0].to_string(), s[1].to_string()),
        _ => Error("Too many inputs".to_string()),
    }
}

struct Docs {
    paths: Vec<PathBuf>
}

impl Docs {
    pub fn new(api_root: &Path) -> Docs {
        let mut paths = Vec::new();
        iter_dir(api_root, &mut |p| {
            paths.push(p.to_path_buf());
        });
        Docs {
            paths: paths
        }
    }

    pub fn get_class(&self, class_name: &str) -> Option<Item> {
        for p in &self.paths {
            let f_name = p.file_name().unwrap().to_str().unwrap();
            if f_name == class_name {
                let mut result = p.clone();
                result.set_file_name(format!("{}.json", class_name));
                return Some(parse_doc_file(&result));
            }
        }
        None
    }
}




fn main() {
    let api_root = Path::new("resources\\api");
    let docs = Docs::new(api_root);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input.to_lowercase();
        let mut input = input.trim();

        let query = parse_input(input);

        match query {
            Class(name) => {
                if let Some(class) = docs.get_class(&name) {
                    println!("Found class: {}", name);
                    class.print_properties();
                }
            },
            Property(class_n, prop_n) => {
                if let Some(class) = docs.get_class(&class_n) {
                    if let Some(prop) = class.get_property(&prop_n) {
                        println!("Found property: {}", prop_n);
                        if let Some(comment) = prop.comment {
                            println!(" Summary: {}", comment.brief);
                            println!("    Type: {}", prop.returns.title);
                        }
                    }
                }
            },
            Error(e) => {
                println!("Error: {}", e);
            }
        }


    }
}

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate serde;

use std::io;
use std::path::{Path,PathBuf};
use std::fs::{read_dir,ReadDir,File};
use serde_json::{Value,Error};
use std::io::Read;

#[derive(Serialize,Deserialize,Debug)]
struct Item {
    entity: Entity,
    tableOfContents: TableOfContents,
}

#[derive(Serialize,Deserialize,Debug)]
struct TableOfContents {
    uxProperties: Vec<PropertyItem>
}

#[derive(Serialize,Deserialize,Debug)]
struct PropertyItem {

}

#[derive(Serialize,Deserialize,Debug)]
struct Entity {
    comment: Comment
}

#[derive(Serialize,Deserialize,Debug)]
struct Comment {
    brief: String,
    full: String,
}

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

fn main() {
    let api_root = Path::new("resources\\api");

    let mut paths = Vec::new();
    iter_dir(api_root, &mut |p| {
        paths.push(p.to_path_buf());
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();


        let mut input = input.to_lowercase();
        let mut input = input.trim();
        println!("foo: {}", input);

        for p in &paths {
            let f_name = p.file_name().unwrap().to_str().unwrap();
            if f_name == input {
                let mut result = p.clone();
                let mut content = String::new();
                result.set_file_name(format!("{}.json", input));
                let mut file = File::open(result).unwrap();

                file.read_to_string(&mut content);

                let res: Item = serde_json::from_str(&content).unwrap();

                println!("{:?}", res);
            }
        }
    }
}

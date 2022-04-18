use std::{env, process, fs};

pub fn read_resource_file(filename: &str) {
    let current_dir = match env::current_dir() {
        Ok(pathbuf) => pathbuf,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let cur_dir = match current_dir.to_str() {
        Some(dir_str) => dir_str,
        None => process::exit(1),
    }.to_string();
    let path: String = cur_dir + "/resources/" + filename; // String + &str　であれば結合できる

    let f = fs::read_to_string(path).expect("file not found");
    dbg!(f);
}

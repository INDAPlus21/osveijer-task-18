use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    let in_path = Path::new("../token.txt");
    let out_path = Path::new("../index_list.txt");

    let lines = read_lines(in_path);
    let mut data = "".to_string();
    let mut current = "a";
    let mut poses: String = "".to_string();

    for i in &lines {
        let words = i.split_whitespace().map(|w| w.trim()).collect::<Vec<&str>>();
        if current != words[0] {
            data += current;
            data += &poses;
            data += "\n";
            current = words[0];
            poses = "".to_string();
        }
        poses += " ";
        poses += words[1];
    }

    write_to_file(out_path, data);
}

fn read_lines(_p: &Path) -> Vec<String> {
    let lines: Vec<String>;

    match File::open(_p) {
        Ok(f) => {
            lines = io::BufReader::new(f).lines().map(|l| l.ok().unwrap()).collect();
        
        },
        _ => {
            println!("Unable to read file");
            std::process::exit(1);
        }
    };

    lines
}

fn write_to_file(path: &Path, data: String) {
    let file = File::create(path).unwrap();
        let mut file = io::BufWriter::new(file);
        file.write_all(data.as_bytes()).expect("error writing");
}
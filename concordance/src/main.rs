use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Seek, Read};
use clap::Parser;

#[derive(Parser)]
struct Args{
    word: String
}

fn main() {
    let path = Path::new("../magic_file.txt");

    let args = Args::parse();

    let key = hash(&args.word);

    let mut reader;
    match File::open(path) {
        Ok(f) => {
            reader = io::BufReader::new(f);
        
        },
        _ => {
            println!("Unable to read text file");
            std::process::exit(1);
        }
    };
    reader.seek_relative(key as i64).expect("error seeking magic file");

    let mut buf: String;
    let mut raw_buf = vec![]; 
    reader.read_until(10,&mut raw_buf).expect("error reading index list");
    buf = String::from_utf8_lossy(&raw_buf).to_string();

    while buf.split_whitespace().collect::<Vec<&str>>().len() == 0 || buf.split_whitespace().collect::<Vec<&str>>()[0] != args.word {
        let mut raw_buf = vec![];
        reader.read_until(10, &mut raw_buf).expect("error reading magic file");
        buf = String::from_utf8_lossy(&raw_buf).to_string();
    }

    let index_path = Path::new("../index_list.txt");

    let mut index_reader;
    match File::open(index_path) {
        Ok(f) => {
            index_reader = io::BufReader::new(f);
        
        },
        _ => {
            println!("Unable to read text file");
            std::process::exit(1);
        }
    };
    index_reader.seek_relative(buf.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()[1].parse().expect("error geting inex for index list")).expect("error seeking magic file");

    let mut raw_data = vec![]; 
    index_reader.read_until(10,&mut raw_data).expect("error reading index list");
    let mut data = String::from_utf8_lossy(&raw_data).split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    data.remove(0);

    match data.len() {
        1 => println!("Det finns en förekomst av ordet."),
        _ => println!("Det finns {} förekomster av ordet.", data.len())
    };

    let offset = 30i64;
    let text_path = Path::new("../korpus.txt");
    let mut text_reader;
    match File::open(text_path) {
        Ok(f) => {
            text_reader = io::BufReader::new(f);
        
        },
        _ => {
            println!("Unable to read text file");
            std::process::exit(1);
        }
    };
    for i in data {
        let mut buffer = vec![0u8; offset as usize * 2 + args.word.len()];
        text_reader.rewind().expect("error reading textfile");
        text_reader.seek_relative(i.trim().parse::<i64>().unwrap() - offset).expect("error reading textfile");
        text_reader.read_exact(&mut buffer).expect("error reading textfile");
        let out: String = String::from_utf8_lossy(&buffer).to_string();
        let new_out = out.replace("\n", " ");
        println!("{}", new_out);
    }
}

fn hash(str: &String) -> usize {
    let mut hash: usize = 0;

    let mut p: usize = 8;
    let m: usize = 10000000;
    for _c in str.encode_utf16() {
        hash += _c as usize * p;
        p *= 8;
    }

    hash = hash % m;

    hash
}

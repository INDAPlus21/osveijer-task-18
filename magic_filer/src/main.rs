use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};
use indicatif;

#[derive(Clone)]
struct Entry {
    data: String,
    key: usize
}

impl Entry {
    fn new(_data: &String, byte_index: usize) -> Entry {
        let word = &_data.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()[0];
        let k = hash(word);
        let d = word.to_string() + " " + &byte_index.to_string() + "\n";
        Entry {
            data: d,
            key: k
        }
    }
}

fn main() {
    let in_path = Path::new("../index_list.txt");
    let out_path = Path::new("../magic_file.txt");

    println!("reading file...");

    let lines = read_lines(in_path);

    println!("creating entries");
    
    let mut entries: Vec<Entry> = Vec::new();

    let ce_bar = indicatif::ProgressBar::new(lines.len() as u64);

    let mut byte_index = 0usize;
    for i in &lines {
        let entry = Entry::new(&i, byte_index);
        byte_index += i.as_bytes().len() + 1;
        entries.push(entry);
        ce_bar.inc(1);
    }

    ce_bar.finish();

    println!("sorting...");

    let len = entries.len();
    sort_entries(&mut entries, 0, len);

    println!("writing...");

    let file = File::create(out_path).unwrap();
    let mut file = io::LineWriter::new(file);
    let mut file_len: usize = 0;

    while entries.len() > 0 {
        let mut write = "".to_string();
        if (entries[0].key as i64 - file_len as i64) > 0 {
            for _ in 0..(entries[0].key - file_len - 1) {
                write += ".";
                file_len += 1;
            }
            write += "\n";
            file_len += 1;
            file.write_all(write.as_bytes()).expect("unable to write");
        }
        let data_bytes =  entries[0].data.as_bytes();
        file.write_all(data_bytes).expect("unable to write");
        file_len += data_bytes.len();
        entries.remove(0);
    }

    println!("Done");

}

// recursive quicksort of entries based on keys
fn sort_entries(entries: &mut Vec<Entry>, start:usize, end:usize) {
    if start == end || end == 0 {return;}
    let pivot = entries[end-1].key;
    let mut cur_index = start;
    for i in start..end {
        if entries[i].key <= pivot {
            if i != cur_index {swap_entries(entries, i, cur_index);}
            cur_index += 1;
        }
    }
    sort_entries(entries, start, cur_index - 1);
    sort_entries(entries, cur_index, end);
}

// swap entries
fn swap_entries(entries: &mut Vec<Entry>, i:usize, j:usize) {
    let temp = entries[i].clone();
    entries[i] = entries[j].clone();
    entries[j] = temp;
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

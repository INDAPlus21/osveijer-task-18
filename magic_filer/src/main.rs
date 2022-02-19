use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Clone)]
struct Entry {
    data: String,
    key: usize
}

impl Entry {
    fn new(_data: &String) -> Entry {
        let k = hash(&_data.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()[0]);
        let d = "#".to_string() + &_data;
        Entry {
            data: d,
            key: k
        }
    }

    fn end(&self) -> usize {
        self.key + self.data.len()
    }
}

fn main() {
    let in_path = Path::new("../index_list.txt");
    let out_path = Path::new("../magic_file.txt");

    println!("reading file...");

    let lines = read_lines(in_path);

    println!("creating entries...");
    
    let mut entries: Vec<Entry> = Vec::new();

    for i in &lines {
        let mut entry = Entry::new(&i);
        move_key(&mut entry, 0, &entries);
        entries.push(entry);
    }

    println!("sorting...");

    let len = entries.len();
    sort_entries(&mut entries, 0, len);

    println!("creating data string...");

    let mut data = "".to_string();

    while entries.len() > 0 {
        for _ in 0..(entries[0].key - data.len()) {
            data += ".";
        }
        data += &entries[0].data;
        entries.remove(0);
    }

    println!("writing...");

    write_to_file(out_path, data);

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

// move keys to make sure they do not conflict witk another entry
fn move_key(entry: &mut Entry, index: usize, entries: &Vec<Entry>) {
    if index >= entries.len() {return;}
    if (entry.key < entries[index].key && entry.end() > entries[index].key) || (entry.key >= entries[index].key && entry.key < entries[index].end()) {
        entry.key = entries[index].end();
    }
    move_key(entry, index + 1, entries);
}

fn hash(str: &String) -> usize {
    let mut hash: usize = 0;

    let mut p: usize = 2;
    let m: usize = 1000000;
    for _c in str.encode_utf16() {
        hash += _c as usize * p;
        p *= 2;
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

fn write_to_file(path: &Path, data: String) {
    let file = File::create(path).unwrap();
        let mut file = io::BufWriter::new(file);
        file.write_all(data.as_bytes());
}

use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::fs::File;

fn main() {
let arguments: Vec<String>=  env::args().collect();
//first argument is program name, second is first argument passed to the program
let source_dir = arguments.get(1).unwrap();

let source_path = Path::new(source_dir);

let _ = visit_dirs(source_path,  &|entry: &DirEntry| process_entry(entry));

}


fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}


fn process_entry(entry: &DirEntry) {

let file = File::open(entry.path()).unwrap();

let mut incr = 0 ;

let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let _ = result.unwrap();
        incr = incr + 1;
    }

    println!("{:?},{}", entry.path(),incr);
}


//"/mnt/c/Users/Elvis Gbaguidi/GolandProjects/gftp/Files_Downloaded_from_SFTP/dia-es/2024-02-27T10h42m27s"
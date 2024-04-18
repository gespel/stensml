use std::{
    fs,
    fs::File,
    io::{prelude::*, BufReader},
    env,
};

pub fn read_file_to_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("File read error");
    let buf = BufReader::new(file);
    
    buf.lines().map(|x| x.expect("could not parse line")).collect()
}

pub fn write_lines_to_file(filename: String, lines: Vec<String>) {
    std::fs::write(&filename, lines.join("\n")).expect("failed to write to file");
    println!("{} was transpiled!", filename);

}
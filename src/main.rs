mod html;
use crate::html::HTMLTranspiler;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    env,
};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Error: No project name defined");
        std::process::exit(1);
    }
    let project_name = args[1].clone();
    //println!("{}", project_name);
    let c = read_file_to_lines(format!("{}.sml", project_name.clone()));
    //let contents = fs::read_to_string("test.sml").expect("Error while reading file!");
    //println!("{:?}", c);
    //println!("{:?}", transpile(c));
    let htmlt = HTMLTranspiler::new();
    write_lines_to_file(format!("{}.html", project_name.clone()), htmlt.transpile(c));
}

fn read_file_to_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("File read error");
    let buf = BufReader::new(file);
    
    buf.lines().map(|x| x.expect("could not parse line")).collect()
}

fn write_lines_to_file(filename: String, lines: Vec<String>) {
    std::fs::write(filename, lines.join("\n")).expect("failed to write to file");
}


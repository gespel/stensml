use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


fn main() {
    let c = read_file_to_lines("test.sml".to_string());
    //let contents = fs::read_to_string("test.sml").expect("Error while reading file!");
    //println!("{:?}", c);
    //println!("{:?}", transpile(c));
    write_lines_to_file("test.html".to_string(), transpile(c));
}

fn read_file_to_lines(filename: String) -> Vec<String> {
    let out: Vec<String> = Vec::new();
    let file = File::open(filename).expect("File read error");
    let buf = BufReader::new(file);
    
    buf.lines().map(|x| x.expect("could not parse line")).collect()
}

fn write_lines_to_file(filename: String, lines: Vec<String>) {
    std::fs::write(filename, lines.join("\n")).expect("failed to write to file");
}

fn transpile(input: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    let mut intable = false;
    for i in 0..input.len() {
        let currentline = input[i].clone();
        let cleanline = (&currentline[1..]).to_string();

        let operator = currentline.chars().nth(0).unwrap();
        let mut part: String;
        match operator {
            '-' => {
                if intable {
                    if currentline.len() == 1 {
                        intable = false;
                        part = format!("</ul>");
                    }
                    else {
                        part = format!("<li>{}</li>", cleanline);
                    }
                }
                else {
                    intable = true;
                    part = format!("<ul><li>{}</li>", cleanline);
                }
            }
            '_' => {
                part = format!("&rarr; {}<br>", cleanline);
            }
            '=' => {
                part = format!("<h1>{}</h1><br>", cleanline); 
            }
            _ => {
                part = format!("{}<br>", currentline);
            }

        }
        out.push(part);
    }

    out
}

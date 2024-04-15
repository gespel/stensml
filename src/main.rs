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
    write_lines_to_file(format!("{}.html", project_name.clone()), transpile(c));
}

fn read_file_to_lines(filename: String) -> Vec<String> {
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
        let part: String;
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

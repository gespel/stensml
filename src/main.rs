mod tools;
mod transpilers;
use crate::transpilers::html_transpiler::HTMLTranspiler;
use crate::transpilers::transpiler::Transpiler;
use crate::tools::{
    write_lines_to_file,
    read_file_to_lines
};

use std::{
    fs,
    env,
};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let dir_path = "./";
        let entries = fs::read_dir(dir_path).expect("Error while reading directory!");
        let htmlt = HTMLTranspiler::new();

        for entry in entries {
            let file_name = entry.expect("Error while fetching file!").file_name().into_string();
            
            if let Ok(file_name) = file_name {
                if file_name.ends_with(".sml") {
                    let c = read_file_to_lines(file_name.clone());
                    write_lines_to_file(format!("{}.html", file_name.trim_end_matches(".sml")), htmlt.transpile(c));
                }
            }
        }
    }
    else {
        let project_name = args[1].clone();
        let c = read_file_to_lines(format!("{}.sml", project_name.clone()));

        let htmlt = HTMLTranspiler::new();
        write_lines_to_file(format!("{}.html", project_name.clone()), htmlt.transpile(c));
    }
}




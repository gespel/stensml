use crate::transpilers::transpiler::Transpiler;


pub struct HTMLTranspiler {

}

impl Transpiler for HTMLTranspiler {
    fn transpile(&self, input: Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        out.push("<style>body { font-family: Arial, sans-serif; background-color: #163325; color: white; }</style>".to_string());

        let mut intable = false;
        
        for i in 0..input.len() {
            let currentline = input[i].clone();
            if currentline.len() > 0 {
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
                        part = format!("<h2>{}</h2><br>", cleanline); 
                    }
                    _ => {
                        part = format!("{}<br>", currentline);
                    }
        
                }
                out.push(part);
            }
            else if currentline.len() == 0 {
                out.push("<br>".to_string());
            }
        }
        out
    }
}

impl HTMLTranspiler {
    pub fn new() -> HTMLTranspiler {
        HTMLTranspiler {

        }
    }
    

    
}


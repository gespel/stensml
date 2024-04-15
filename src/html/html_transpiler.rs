pub struct HTMLTranspiler {

}

impl HTMLTranspiler {
    pub fn new() -> HTMLTranspiler {
        HTMLTranspiler {

        }
    }

    pub fn transpile(&self, input: Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
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
                        part = format!("<h1>{}</h1><br>", cleanline); 
                    }
                    _ => {
                        part = format!("{}<br>", currentline);
                    }
        
                }
                out.push(part);
            } 
        }
        out
    }
}


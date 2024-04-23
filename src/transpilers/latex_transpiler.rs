use crate::transpilers::transpiler::Transpiler;

pub struct LatexTranspiler {

}

impl Transpiler for LatexTranspiler {
    fn transpile(&self, input: Vec<String>) -> Vec<String> {
        todo!()
    }
}

impl LatexTranspiler {
    pub fn new() -> LatexTranspiler {
        LatexTranspiler {
            
        }
    }
}
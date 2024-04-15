pub trait Transpiler {
    fn transpile(&self, input: Vec<String>) -> Vec<String>;
}
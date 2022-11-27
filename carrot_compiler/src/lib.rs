pub mod tokeniser;
pub mod prepreprocessor;
pub mod util;

pub fn compile(input: &str) {
    let preprocessed = prepreprocessor::preprocess(input);
    
}
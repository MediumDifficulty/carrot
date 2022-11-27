use std::process::exit;

pub mod tokeniser;
pub mod prepreprocessor;
pub mod util;
pub mod preprocessor;

pub fn compile(input: &str) {
    let prepreprocessed = print_err!(prepreprocessor::prepreprocess(input));
    let preprocessed = preprocessor::preprocess(prepreprocessed);
}
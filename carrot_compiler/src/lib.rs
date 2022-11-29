pub mod tokeniser;
pub mod prepreprocessor;
pub mod util;
pub mod preprocessor;

pub fn compile(input: &str) -> Result<(), String> {
    let prepreprocessed = prepreprocessor::prepreprocess(input)?;
    let preprocessed = preprocessor::preprocess(prepreprocessed)?;
    let tokenised = tokeniser::tokenise(preprocessed)?;

    Ok(())
}
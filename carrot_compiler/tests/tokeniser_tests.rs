#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::*, prepreprocessor, tokeniser::tokenise};

    #[test]
    fn tokeniser_test() {
        let preprocessed = preprocess(prepreprocessor::prepreprocess("a(sd)f)").unwrap());
        println!("Preprocessed: {:?}", preprocessed);
        println!("Tokenised: {:?}", tokenise(preprocessed.unwrap()));
    }
}

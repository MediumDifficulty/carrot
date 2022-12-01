#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::*, prepreprocessor, tokeniser::tokenise};

    #[test]
    fn tokeniser_test() {
        let preprocessed = preprocess(prepreprocessor::prepreprocess("
        let(abc, \"abc\");
        ").unwrap());
        println!("Preprocessed: {:?}", preprocessed);
        println!("Tokenised: {:?}", tokenise(preprocessed.unwrap()));
    }
}

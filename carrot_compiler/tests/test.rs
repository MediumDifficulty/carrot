#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::{self}, prepreprocessor, tokeniser::{self}, funcifier};

    #[test]
    fn test() {
        let input = "
        let(abc, \"abc\");
        print(\"asdf\");
        let(two, add(1, 1));
        let(four, mul(add(1, 1), add(1, 1)));
        def(asdf, (
            let(a, 2);
            let(b, 3);
            return(add(a, b));
        ));
        ";

        let prepreprocessed = prepreprocessor::prepreprocess(input.clone()).unwrap();
        println!("prepreprocessed: {:?}", prepreprocessed);

        let preprocessed = preprocessor::preprocess(prepreprocessed.clone()).unwrap();
        println!("preprocessed: {:?}", preprocessed);

        let tokenised = tokeniser::tokenise(preprocessed.clone()).unwrap();
        println!("tokenised: {:?}", tokenised);

        let function_tree = funcifier::funcify(tokenised.clone()).unwrap();
        println!("function_tree: {:?}", function_tree);
    }
}

#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::*, prepreprocessor};

    #[test]
    fn preprocessor_test() {
        println!("{:?}", preprocess(prepreprocessor::prepreprocess("te\" \\\"  \" s t").unwrap()));
    }
}

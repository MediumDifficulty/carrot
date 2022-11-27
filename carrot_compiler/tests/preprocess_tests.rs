#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::*, prepreprocessor};

    #[test]
    fn test() {
        println!("{:?}", preprocess(prepreprocessor::prepreprocess("te\" \\\"  \" s t").unwrap()));
    }
}

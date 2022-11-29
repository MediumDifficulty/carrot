#[cfg(test)]
mod tests {
    use carrot_compiler::prepreprocessor::*;

    #[test]
    fn pre_preprocessor_test() {
        println!("{:?}", prepreprocess("\\\"test"))
    }
}

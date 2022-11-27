#[cfg(test)]
mod tests {
    use carrot_compiler::prepreprocessor::*;

    #[test]
    fn test() {
        println!("{:?}", prepreprocess("\\\"test"))
    }
}

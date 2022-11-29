#[cfg(test)]
mod tests {
    use carrot_compiler::{preprocessor::*, prepreprocessor, compile};

    #[test]
    fn test() {
        compile("as\" \" df");
    }
}

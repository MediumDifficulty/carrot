use crate::{prepreprocessor::Escaped, return_err};

pub fn preprocess(input: Vec<Escaped>) -> Vec<Escaped> {
    let mut preprocessed = Vec::new();

    let mut in_string = false;
    for esc in input {
        if let Escaped::Quote = esc {
            preprocessed.push(esc);
            continue;
        }

        if return_err!(esc.char()) == '"' {
            in_string = !in_string;
            preprocessed.push(Escaped::Char('"'));
            continue;
        }

        if return_err!(esc.char()).is_whitespace() && !in_string {
            continue;
        }

        preprocessed.push(esc)
    }

    preprocessed
}
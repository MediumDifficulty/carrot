use crate::prepreprocessor::Escaped;

pub fn preprocess(input: Vec<Escaped>) -> Result<Vec<Escaped>, String> {
    let mut preprocessed = Vec::new();

    let mut in_string = false;
    for esc in input {
        if let Escaped::Quote = esc {
            preprocessed.push(esc);
            continue;
        }

        if esc.char()? == '"' {
            in_string = !in_string;
            preprocessed.push(Escaped::Char('"'));
            continue;
        }

        if esc.char()?.is_whitespace() && !in_string {
            continue;
        }

        preprocessed.push(esc)
    }

    Ok(preprocessed)
}
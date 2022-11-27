use crate::{err_str, return_err};

pub fn prepreprocess(input: &str) -> Result<Vec<Escaped>, String> {
    let escaped_chars = input.chars()
        .map(|c| Escaped::Char(c))
        .collect::<Vec<Escaped>>();

    let mut escaped = Vec::new();
    let mut is_escaped = false;
    for c in escaped_chars.iter() {
        let c_char = return_err!(c.char());

        if c_char == '\\' && is_escaped {
            escaped.push(Escaped::Char('\\'));
            is_escaped = false;
            continue;
        }

        if c_char == '\\' {
            is_escaped = true;
            continue;
        }

        if is_escaped {
            escaped.push(
                return_err!(Escaped::from_char(return_err!(c.char())))
            );
            is_escaped = false;
            continue;
        }

        escaped.push(*c);
    }

    Ok(escaped)
}

#[derive(Clone, Copy, Debug)]
pub enum Escaped {
    Char(char),
    Quote
}

impl Escaped {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '"' => Ok(Self::Quote),
            _ => err_str!("Invalid escaped character")
        }
    }

    pub fn char(&self) -> Result<char, String> {
        match self {
            Self::Char(c) => Ok(*c),
            _ => err_str!("/!\\ Cannot get character from anything other than Escaped::Char")
        }
    }
}
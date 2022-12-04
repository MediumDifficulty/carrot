use crate::util::ErrStr;

pub fn prepreprocess(input: &str) -> Result<Vec<Escaped>, String> {
    let escaped_chars = input.chars()
        .map(Escaped::Char)
        .collect::<Vec<Escaped>>();

    let mut escaped = Vec::new();
    let mut is_escaped = false;
    for c in escaped_chars.iter() {
        let c_char = c.char()?;

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
                Escaped::from_char(c.char()?)?
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
            _ => "Invalid escaped character".to_err()
        }
    }

    pub fn char(&self) -> Result<char, String> {
        match self {
            Self::Char(c) => Ok(*c),
            _ => "PPP /!\\ Cannot get character from anything other than Escaped::Char".to_err()
        }
    }

    pub fn escaped_char(&self) -> Result<char, String> {
        match self {
            Self::Quote => Ok('"'),
            _ => "PPP /!\\ Cannot get escaped char from Escaped::Char".to_err()
        }
    }

    pub fn is_escaped(&self) -> bool {
        !matches!(self, Self::Char(_))
    }
}
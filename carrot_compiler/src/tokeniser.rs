use crate::{prepreprocessor::Escaped, util::ErrStr};

pub fn tokenise(input: Vec<Escaped>) -> Result<Vec<Token>, String> {
    let mut state = State::Single;

    let mut buffer = String::new();
    let mut tokens = Vec::new();

    for e in input.iter() {
        if e.is_escaped() {
            buffer.push(e.escaped_char()?);
            continue;
        }

        let c = e.char()?;

        if Token::char_is_single(c) {
            flush(state, &mut tokens, &mut buffer)?;
            tokens.push(Token::from_single(c)?);
            state = State::Single;
            continue;
        }

        if c.is_numeric() && state != State::String {
            buffer.push(c);
            state = State::Number;
            continue;
        }

        if c == '"' {
            if state == State::String {
                flush(state, &mut tokens, &mut buffer)?;
                state = State::Single;
            } else {
                state = State::String;
            }
            continue;
        }

        if state != State::String {
            state = State::Identifier;
        }
        buffer.push(c);
    }

    Ok(tokens)
}

fn flush(
    state: State,
    tokens: &mut Vec<Token>,
    buffer: &mut String
) -> Result<(), String> {
    if buffer.is_empty() {
        return Ok(());
    }

    tokens.push(match state {
        State::String => Token::String(buffer.to_owned()),
        State::Number => Token::Number(buffer.parse().or_else(|_| "Could not parse number".to_err()).unwrap()),
        State::Identifier => Token::Identifier(buffer.to_owned()),
        State::Single => return "T /!\\ Tried to clear buffer when state is nothing".to_err(),
    });
    buffer.clear();

    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Single,
    String,
    Number,
    Identifier
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String),
    String(String),
    Number(isize),
    Open,
    Close,
    Seperator,
    ArgumentSeperator,
}

impl Token {
    fn char_is_single(c: char) -> bool {
        matches!(c, '(' | ')' | ';' | ',')
    }

    fn from_single(c: char) -> Result<Self, String> {
        match c {
            '(' => Ok(Token::Open),
            ')' => Ok(Token::Close),
            ';' => Ok(Token::Seperator),
            ',' => Ok(Token::ArgumentSeperator),
            _ => "T /!\\ Could not get token from single".to_err()
        }
    }
}
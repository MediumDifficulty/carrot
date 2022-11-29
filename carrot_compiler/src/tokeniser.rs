use crate::{prepreprocessor::Escaped, util::ErrStr};

pub fn tokenise(input: Vec<Escaped>) -> Result<Vec<Token>, String> {
    let mut state = None;
    let mut last_state = None;

    let mut buffer = String::new();
    let mut tokens = Vec::new();

    for (i, e) in input.iter().enumerate() {
        if e.is_escaped() {
            buffer.push(e.escaped_char()?);
            continue;
        }

        let c = e.char()?;

        if Token::char_is_single(c) {
            flush(&mut last_state, &mut state, &mut tokens, &mut buffer)?;
            tokens.push(Token::from_single(c)?);
            state = Some(State::Single);
            continue;
        }

        if c.is_numeric() {
            buffer.push(c);
            state = Some(State::Number);
            continue;
        }

        if c == '"' {
            if compare_states(state, Some(State::String)) {
                state = None;
            } else {
                state = Some(State::String);
            }
            continue;
        }

        state = Some(State::Identifier);
        buffer.push(c);

        if !compare_states(state, last_state) || (i >= input.len() - 1) {
            flush(&mut last_state, &mut state, &mut tokens, &mut buffer)?
        }
    }

    Ok(tokens)
}

fn flush<'a>(
    last_state: &'a mut Option<State>,
    state: &'a mut Option<State>,
    tokens: &mut Vec<Token>,
    buffer: &mut String
) -> Result<(), String> {
    if last_state.is_none() {
        *last_state = *state;
    } else {        
        tokens.push(match state.unwrap() {
            State::String => Token::String(buffer.to_owned()),
            State::Number => Token::Number(buffer.parse().or_else(|_| return "Could not parse number".to_err()).unwrap()),
            State::Identifier => Token::Identifier(buffer.to_owned()),
            State::Single => return "T /!\\ Tried to clear buffer when state is nothing".to_err(),
        });
        buffer.clear();
    }

    Ok(())
}

fn compare_states(a: Option<State>, b: Option<State>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() && b.is_some() {
        return false;
    }

    if a.is_some() && b.is_none() {
        return false;
    }


    a.unwrap() == b.unwrap()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Single,
    String,
    Number,
    Identifier
}

#[derive(Debug)]
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
        match c {
            '(' => true,
            ')' => true,
            ';' => true,
            ',' => true,
            _ => false
        }
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
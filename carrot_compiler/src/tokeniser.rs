pub fn tokenise(input: &str) {

}

pub enum Token {
    Identifier(String),
    String(String),
    Number(isize),
    Open,
    Close,
    Seperator,
}
#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Illegal(usize),
    EndOfFile(usize),

    Assign(usize),   // =
    Plus(usize),     // +
    Minus(usize),    // -
    Asterisk(usize), // *
    Slash(usize),    // /
    Colon(usize),    // :
    Dot(usize),      // .
    LT(usize),       // <
    GT(usize),       // >
    LParen(usize),   // (
    RParen(usize),   // )
    LBrace(usize),   // {
    RBrace(usize),   // }

    Let(usize), // let
}

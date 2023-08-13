#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(usize),
    EndOfFile(usize),

    // -- symbol
    Assign(usize),   // =
    Plus(usize),     // +
    Minus(usize),    // -
    Asterisk(usize), // *
    Slash(usize),    // /
    Colon(usize),    // :
    Dot(usize),      // .
    LT(usize),       // <
    GT(usize),       // >

    Identifier(usize, String),
    Integer(usize, i64),

    // -- reservedid
    Let(usize),      // let
    In(usize),       // in
    Where(usize),    // where
}

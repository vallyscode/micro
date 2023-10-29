#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Illegal(usize),
    EndOfFile(usize),

    Assign(usize),   // =
    Plus(usize),     // +
    Minus(usize),    // -
    Asterisk(usize), // *
    Bang(usize),     // !
    Slash(usize),    // /
    Colon(usize),    // :
    Dot(usize),      // .
    LT(usize),       // <
    GT(usize),       // >
    LE(usize),       // <=
    GE(usize),       // >=
    EQ(usize),       // ==
    NE(usize),       // !=
    LParen(usize),   // (
    RParen(usize),   // )
    LBrace(usize),   // {
    RBrace(usize),   // }
    Let(usize),      // let

    Integer(usize, i32),
    Boolean(usize, bool),


    Identifier(usize, String),
}

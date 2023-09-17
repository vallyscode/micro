use std::str::{Chars, FromStr};

use crate::lexeme::Lexeme;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next(&mut self) -> Lexeme {
        self.skip_whitespaces();

        let word: &str = self.read_word();
        let len: usize = word.len();

        match word {
            "=" => Lexeme::Assign(self.position - 1),
            "/" => Lexeme::Slash(self.position - 1),
            "-" => Lexeme::Minus(self.position - 1),
            "+" => Lexeme::Plus(self.position - 1),
            "." => Lexeme::Dot(self.position - 1),
            "" => Lexeme::EndOfFile(self.position - 1),
            "let" => Lexeme::Let(self.position - 3),
            _ => {
                if let Ok(n) = i32::from_str(word) {
                    return Lexeme::Integer(self.position - len, n);
                }
                return Lexeme::Illegal(self.position - 1);
            }
        }
    }

    fn read_word(&mut self) -> &str {
        let mut len: usize = 0;
        let mut chars = self.input.chars();
        while let Some(c) = chars.next() {
            if is_whitespace(c) {
                break;
            }
            self.position += 1;
            len += c.len_utf8();
        }
        let word = &self.input[0..len];
        self.input = &self.input[len..];
        word
    }

    fn skip_whitespaces(&mut self) {
        let mut len: usize = 0;
        let mut chars: Chars<'_> = self.input.chars();
        while let Some(c) = chars.next() {
            if is_whitespace(c) {
                self.position += 1;
                len += c.len_utf8();
                continue;
            }
            break;
        }
        self.input = &self.input[len..];
    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn is_letter(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_lexer() {
        let text = "abc";
        let lexer = Lexer::new(text);
        assert_eq!(lexer.input, text);
    }

    #[test]
    fn should_parse_lexemes() {
        let text = "
         = 
         /
         -
         3
         +
         10
         let
         .
         ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Lexeme::Assign(10));
        assert_eq!(lexer.next(), Lexeme::Slash(22));
        assert_eq!(lexer.next(), Lexeme::Minus(33));
        assert_eq!(lexer.next(), Lexeme::Integer(44, 3));
        assert_eq!(lexer.next(), Lexeme::Plus(55));
        assert_eq!(lexer.next(), Lexeme::Integer(66, 10));
        assert_eq!(lexer.next(), Lexeme::Let(78));
        assert_eq!(lexer.next(), Lexeme::Dot(91));
        assert_eq!(lexer.next(), Lexeme::EndOfFile(101));
    }

    #[test]
    fn should_return_illegal_token() {
        let text = "?";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Lexeme::Illegal(0));
    }
}

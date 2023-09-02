use std::str::Chars;

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

        match word {
            "=" => Lexeme::Assign(self.position - 1),
            _ => Lexeme::EndOfFile(self.position - 1),
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
    fn should_parse_lexeme() {
        let text = " = ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Lexeme::Assign(1));
        assert_eq!(lexer.next(), Lexeme::EndOfFile(2));
    }

    // #[test]
    // fn should_return_illegal_token() {
    //     let text = "?";
    //     let mut lexer = Lexer::new(text);
    //     assert_eq!(lexer.next(), Lexeme::Illegal(0));
    // }
}

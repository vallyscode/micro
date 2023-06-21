#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(usize),
    EndOfFile(usize),
    Assign(usize),
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next(&mut self) -> Token {
        if let Some(c) = self.input.chars().next() {
            let token = match c {
                '=' => Token::Assign(self.position),
                _ => Token::Illegal(self.position),
            };
            self.position += 1;
            self.input = &self.input[c.len_utf8()..];
            return token;
        }
        Token::EndOfFile(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_lexer() {
        let text = "+-=";
        let lexer = Lexer::new(text);
        assert_eq!(lexer.input, text);
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn should_parse_known_tokens() {
        let text = "==";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Assign(0));
        assert_eq!(lexer.next(), Token::Assign(1));
        assert_eq!(lexer.next(), Token::EndOfFile(2));
    }

    #[test]
    fn should_return_illegal_token() {
        let text = "?";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Illegal(0));
    }
}

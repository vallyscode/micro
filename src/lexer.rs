#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(usize),
    EndOfFile(usize),
    Assign(usize),
    Plus(usize),
    Identifier(usize, String),
    Let(usize),
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
        self.drop_whitespace();

        if let Some(c) = self.read_char() {
            let token = match c {
                '=' => Token::Assign(self.position - 1),
                '+' => Token::Plus(self.position - 1),
                _ => {
                    if c.is_alphabetic() {
                        let identifier = self.read_identifier(c);
                        return match identifier.as_str() {
                            "let" => Token::Let(self.position - 4),
                            _ => {
                                Token::Identifier(self.position - identifier.len() - 1, identifier)
                            }
                        };
                    }
                    return Token::Illegal(self.position - 1);
                }
            };

            return token;
        }
        Token::EndOfFile(self.position)
    }

    fn read_identifier(&mut self, c: char) -> String {
        let mut identifier = String::from(c);
        while let Some(c) = self.read_char() {
            if c.is_alphabetic() {
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn read_char(&mut self) -> Option<char> {
        let maybe_char = self.input.chars().next();
        if let Some(c) = maybe_char {
            self.position += 1;
            self.input = &self.input[c.len_utf8()..];
        }
        maybe_char
    }

    fn drop_whitespace(&mut self) {
        loop {
            match self.input.chars().next() {
                Some(c) if c == ' ' => {
                    self.position += 1;
                    self.input = &self.input[c.len_utf8()..];
                }
                _ => break,
            }
        }
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
        let text = "==+";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Assign(0));
        assert_eq!(lexer.next(), Token::Assign(1));
        assert_eq!(lexer.next(), Token::Plus(2));
        assert_eq!(lexer.next(), Token::EndOfFile(3));
    }

    #[test]
    fn should_return_illegal_token() {
        let text = "?";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Illegal(0));
    }

    #[test]
    fn should_return_identifier() {
        let text = "foo 123";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Identifier(0, "foo".to_string()));
    }

    #[test]
    fn should_read_expression() {
        let text = "let foo = 10";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Let(0));
        assert_eq!(lexer.next(), Token::Identifier(4, "foo".to_string()));
        assert_eq!(lexer.next(), Token::Assign(8));
    }
}

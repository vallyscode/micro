use crate::toke::Token;

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
                '-' => Token::Minus(self.position - 1),
                '*' => Token::Asterisk(self.position - 1),
                '/' => Token::Slash(self.position - 1),
                ':' => Token::Colon(self.position - 1),
                '.' => Token::Dot(self.position - 1),
                '<' => Token::LT(self.position - 1),
                '>' => Token::GT(self.position - 1),
                '(' => Token::LParen(self.position - 1),
                ')' => Token::RParen(self.position - 1),
                '{' => Token::LBrace(self.position - 1),
                '}' => Token::RBrace(self.position - 1),
                _ => {
                    if c.is_alphabetic() {
                        let identifier = self.read_identifier(c);
                        return match identifier.as_str() {
                            "let" => Token::Let(self.position - 4),
                            "in" => Token::In(self.position - 3),
                            "where" => Token::Where(self.position - 5),
                            _ => {
                                Token::Identifier(self.position - identifier.len() - 1, identifier)
                            }
                        };
                    } else if c.is_digit(10) {
                        let integer: String = self.read_integer(c);
                        return Token::Integer(
                            self.position - integer.len(),
                            integer.parse().unwrap(),
                        );
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

    fn read_integer(&mut self, c: char) -> String {
        let mut integer: String = String::from(c);
        while let Some(c) = self.read_char() {
            if c.is_digit(10) {
                integer.push(c);
            } else {
                break;
            }
        }
        integer
    }

    fn read_char(&mut self) -> Option<char> {
        let maybe_char: Option<char> = self.input.chars().next();
        if let Some(c) = maybe_char {
            self.position += 1;
            self.input = &self.input[c.len_utf8()..];
        }
        maybe_char
    }

    fn drop_whitespace(&mut self) {
        loop {
            match self.input.chars().next() {
                Some(c) if c == ' ' || c == '\t' || c == '\n' || c == '\r' => {
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
        let text = "abc";
        let lexer = Lexer::new(text);
        assert_eq!(lexer.input, text);
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn should_tokenize_short_assignment() {
        let text = "
            x = 10
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Identifier(13, "x".to_string()));
        assert_eq!(lexer.next(), Token::Assign(15));
        assert_eq!(lexer.next(), Token::Integer(18, 10));
        assert_eq!(lexer.next(), Token::EndOfFile(28));
    }

    #[test]
    fn should_tokenize_verbose_assignment() {
        let text = "
            x :: int
            x = 10
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Identifier(13, "x".to_string()));
        assert_eq!(lexer.next(), Token::Colon(15));
        assert_eq!(lexer.next(), Token::Colon(16));
        assert_eq!(lexer.next(), Token::Identifier(18, "int".to_string()));
        assert_eq!(lexer.next(), Token::Identifier(34, "x".to_string()));
        assert_eq!(lexer.next(), Token::Assign(36));
        assert_eq!(lexer.next(), Token::Integer(39, 10));
        assert_eq!(lexer.next(), Token::EndOfFile(49));
    }

    #[test]
    fn should_tokenize_short_function_definition() {
        let text = "
            id a = a
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Identifier(13, "id".to_string()));
        assert_eq!(lexer.next(), Token::Identifier(16, "a".to_string()));
        assert_eq!(lexer.next(), Token::Assign(18));
        assert_eq!(lexer.next(), Token::Identifier(20, "a".to_string()));
        assert_eq!(lexer.next(), Token::EndOfFile(30));
    }

    #[test]
    fn should_tokenize_verbose_function_definition() {
        let text = "
            id :: a -> a
            id a = a
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Identifier(13, "id".to_string()));
        assert_eq!(lexer.next(), Token::Colon(16));
        assert_eq!(lexer.next(), Token::Colon(17));
        assert_eq!(lexer.next(), Token::Identifier(19, "a".to_string()));
        assert_eq!(lexer.next(), Token::Minus(21));
        assert_eq!(lexer.next(), Token::GT(22));
        assert_eq!(lexer.next(), Token::Identifier(24, "a".to_string()));
        assert_eq!(lexer.next(), Token::Identifier(38, "id".to_string()));
        assert_eq!(lexer.next(), Token::Identifier(41, "a".to_string()));
        assert_eq!(lexer.next(), Token::Assign(43));
        assert_eq!(lexer.next(), Token::Identifier(45, "a".to_string()));
        assert_eq!(lexer.next(), Token::EndOfFile(55));
    }

    #[test]
    fn should_return_illegal_token() {
        let text = "?";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Illegal(0));
    }
}

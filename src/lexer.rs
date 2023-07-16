#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(usize),
    EndOfFile(usize),

    // -- symbol
    Assign(usize),
    Plus(usize),
    Minus(usize),
    Asterisk(usize),
    Slash(usize),
    Dot(usize),
    LT(usize),
    GT(usize),

    Identifier(usize, String),
    Integer(usize, i64),

    // -- reservedid
    Let(usize),
    In(usize),
    Where(usize),
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
                '-' => Token::Minus(self.position - 1),
                '*' => Token::Asterisk(self.position - 1),
                '/' => Token::Slash(self.position - 1),
                '.' => Token::Dot(self.position - 1),
                '<' => Token::LT(self.position - 1),
                '>' => Token::GT(self.position - 1),
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
                        let integer = self.read_integer(c);
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
        let mut integer = String::from(c);
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
    fn should_parse_assignment() {
        let text = "
            let x = 10
        ";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Let(13));
        assert_eq!(lexer.next(), Token::Identifier(17, "x".to_string()));
        assert_eq!(lexer.next(), Token::Assign(19));
        assert_eq!(lexer.next(), Token::Integer(22, 10));
    }

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
    fn should_read_let_expression() {
        let text = "let foo = 369";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Let(0));
        assert_eq!(lexer.next(), Token::Identifier(4, "foo".to_string()));
        assert_eq!(lexer.next(), Token::Assign(8));
        assert_eq!(lexer.next(), Token::Integer(10, 369));
    }

    #[test]
    fn should_read_let_in_expression() {
        let text = "let foo = 3 bar = 6 in foo + bar";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Let(0));
        assert_eq!(lexer.next(), Token::Identifier(4, "foo".to_string()));
        assert_eq!(lexer.next(), Token::Assign(8));
        assert_eq!(lexer.next(), Token::Integer(11, 3));
        assert_eq!(lexer.next(), Token::Identifier(12, "bar".to_string()));
        assert_eq!(lexer.next(), Token::Assign(16));
        assert_eq!(lexer.next(), Token::Integer(19, 6));
        assert_eq!(lexer.next(), Token::In(20));
        assert_eq!(lexer.next(), Token::Identifier(23, "foo".to_string()));
        assert_eq!(lexer.next(), Token::Plus(27));
        assert_eq!(lexer.next(), Token::Identifier(28, "bar".to_string()));
        assert_eq!(lexer.next(), Token::EndOfFile(32));
    }

    #[test]
    fn should_read_let_in_where_expression() {
        let text = "let x = 3 in inc x where inc = + 1";
        let mut lexer = Lexer::new(text);
        assert_eq!(lexer.next(), Token::Let(0));
        assert_eq!(lexer.next(), Token::Identifier(4, "x".to_string()));
        assert_eq!(lexer.next(), Token::Assign(6));
        assert_eq!(lexer.next(), Token::Integer(9, 3));
        assert_eq!(lexer.next(), Token::In(10));
        assert_eq!(lexer.next(), Token::Identifier(13, "inc".to_string()));
        assert_eq!(lexer.next(), Token::Identifier(17, "x".to_string()));
        assert_eq!(lexer.next(), Token::Where(20));
        assert_eq!(lexer.next(), Token::Identifier(25, "inc".to_string()));
        assert_eq!(lexer.next(), Token::Assign(29));
        assert_eq!(lexer.next(), Token::Plus(31));
        assert_eq!(lexer.next(), Token::Integer(33, 1));
        assert_eq!(lexer.next(), Token::EndOfFile(34));
    }
}

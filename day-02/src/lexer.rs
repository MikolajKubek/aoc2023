pub mod lexer {
    pub struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: char,
    }

    impl Lexer {
        pub fn new(input: &str) -> Self {
            let mut lexer = Lexer {
                input: input.to_owned(),
                position: 0,
                read_position: 0,
                ch: '\0',
            };

            lexer.read_char();

            lexer
        }

        fn read_char(&mut self) {
            if self.read_position >= self.input.len() {
                self.ch = '\0';
            } else {
                self.ch = self.input.chars().nth(self.read_position).unwrap();
            }
            self.position = self.read_position;
            self.read_position += 1;
        }

        pub fn next_token(&mut self) -> Token {
            self.skip_whitespace();

            let token: Token = match self.ch {
                ';' => Token {
                    t_type: TokenType::Semicolon,
                    literal: ";".to_owned(),
                },
                ',' => Token {
                    t_type: TokenType::Comma,
                    literal: ",".to_owned(),
                },
                ':' => Token {
                    t_type: TokenType::Colon,
                    literal: ":".to_owned(),
                },
                '\0' => Token {
                    t_type: TokenType::Eof,
                    literal: "\0".to_owned(),
                },
                _ => {
                    if self.ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        let t_type = Token::lookup_identifier(&literal);
                        Token { t_type, literal }
                    } else if self.ch.is_digit(10) {
                        let literal = self.read_number();
                        Token {
                            t_type: TokenType::Number,
                            literal,
                        }
                    } else {
                        Token {
                            t_type: TokenType::Illegal,
                            literal: "".to_owned(),
                        }
                    }
                }
            };

            self.read_char();
            token
        }

        fn read_number(&mut self) -> String {
            let position = self.position;
            while self.peek_char().is_digit(10) {
                self.read_char();
            }
            self.input[position..self.position + 1].to_owned()
        }

        fn read_identifier(&mut self) -> String {
            let position = self.position;

            while self.peek_char().is_alphabetic() {
                self.read_char();
            }

            self.input[position..self.position + 1].to_owned()
        }

        fn skip_whitespace(&mut self) {
            while self.ch == ' ' || self.ch == '\n' {
                self.read_char();
            }
        }

        fn peek_char(&self) -> char {
            if self.read_position >= self.input.len() {
                '\0'
            }
            else {
                self.input.chars().nth(self.read_position).unwrap()
            }
        }
    }

    #[derive(Debug)]
    pub struct Token {
        pub t_type: TokenType,
        pub literal: String,
    }

    impl Token {
        fn lookup_identifier(identifier: &str) -> TokenType {
            match identifier {
                "Game" => TokenType::Game,
                "red" => TokenType::ColorRed,
                "green" => TokenType::ColorGreen,
                "blue" => TokenType::ColorBlue,
                _ => TokenType::Illegal,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum TokenType {
        Game,
        Number,
        Colon,
        Comma,
        Semicolon,
        ColorRed,
        ColorGreen,
        ColorBlue,
        Eof,
        Illegal,
    }
}

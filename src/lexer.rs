use crate::symbols::NUMBERS;
use crate::Token;

pub struct Lexer<'a> {
    pub input_code: &'a str,
    pub pos: i32,
    pub current_char: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input_code: &'a str) -> Lexer {
        Lexer {
            current_char: "".to_string(),
            input_code,
            pos: -1,
        }
    }

    pub fn get_next_token(&mut self) -> Token<String> {
        while self.current_char != "EOF" {

            if self.current_char.len() == 0 {
                self.advance();
            }

            if self.current_char == " " {
                self.skip_whitespace();
                continue;
            }

            if self.is_number() {
                let token = Token {
                    ttype: crate::token::TokenTypes::INTEGER,
                    value: self.create_number_representation(),
                };

                return token;
            }

            if self.current_char == "+" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::SUM,
                    value: "+".to_string(),
                };
            }

            if self.current_char == "-" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::MIN,
                    value: "-".to_string(),
                };
            }

            if self.current_char == "*" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::MUL,
                    value: "*".to_string(),
                };
            }

            if self.current_char == "/" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::DIV,
                    value: "/".to_string(),
                };
            }

            if self.current_char == "(" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::LPAREN,
                    value: "(".to_string(),
                };
            }

            if self.current_char == ")" {
                self.advance();
                return Token {
                    ttype: crate::token::TokenTypes::RPAREN,
                    value: ")".to_string(),
                };
            }

            self.error();
        }

        Token {
            ttype: crate::token::TokenTypes::EOF,
            value: "EOF".to_string(),
        }
    }

    fn create_number_representation(&mut self) -> String {
        let mut res: String = "".to_string();

        while self.current_char != " " && self.is_number() {
            res = res + &self.current_char;
            self.advance();
        }

        res
    }

    fn advance(&mut self) {
        self.pos = self.pos + 1;

        if self.pos > (self.input_code.len() - 1).try_into().unwrap() {
            self.current_char = "EOF".to_owned();
        } else {
            let cursor_position = self.pos.try_into().unwrap();
            let current_char_string = self.input_code.chars().nth(cursor_position).unwrap();
            self.current_char = current_char_string.to_string();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == " " {
            self.advance()
        }
    }

    fn error(&self) {
        panic!("Invalid token");
    }

    fn is_number(&self) -> bool {
        NUMBERS.contains(&self.current_char)
    }
}

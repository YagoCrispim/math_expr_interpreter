use crate::{
    token::{Token, TokenTypes},
    Lexer,
};

pub struct Interpreter<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Option<Token<String>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let next_token = lexer.get_next_token();

        Interpreter {
            lexer,
            current_token: Some(next_token),
        }
    }

    fn eat(&mut self, token_type: TokenTypes) {
        if self.current_token.as_ref().unwrap().ttype == token_type {
            let current_token = Some(self.lexer.get_next_token());
            self.current_token = current_token;
        } else {
            println!(
                "Expected type: {:?} --> {:?} => {:?}",
                token_type,
                self.current_token.as_ref().unwrap().ttype,
                self.current_token.as_ref().unwrap().value
            );
            self.error("Invalid syntax. Interpreter:103")
        }
    }

    fn error(&self, message: &str) -> ! {
        panic!("{}", message);
    }

    pub fn expr(&mut self) -> i32 {
        let mut result = self.term();

        while self.current_token.is_some() {
            let token = self.current_token.as_ref().unwrap();

            match token.ttype {
                TokenTypes::EOF => break,
                TokenTypes::SUM => {
                    self.eat(TokenTypes::SUM);
                    let term = self.term();
                    result = result + term;
                }
                TokenTypes::MIN => {
                    self.eat(TokenTypes::MIN);
                    let term = self.term();
                    result = result - term;
                }
                _ => break,
            }
        }

        result
    }

    fn term(&mut self) -> i32 {
        let mut result = self.factor();

        while self.current_token.is_some() {
            let token = self.current_token.as_ref().unwrap();

            match token.ttype {
                TokenTypes::MUL => {
                    self.eat(TokenTypes::MUL);
                    result = result * self.factor();
                }
                TokenTypes::DIV => {
                    self.eat(TokenTypes::DIV);
                    result = result / self.factor();
                }
                _ => break
            }
        }

        result
    }

    fn factor(&mut self) -> i32 {
        let result: i32;
        let token = self.current_token.as_ref().unwrap();

        match token.ttype {
            TokenTypes::INTEGER => {
                result = token.clone().value.parse::<i32>().unwrap();
                self.eat(TokenTypes::INTEGER);

                result
            }
            TokenTypes::LPAREN => {
                self.eat(TokenTypes::LPAREN);

                result = self.expr();

                self.eat(TokenTypes::RPAREN);

                result
            }
            _ => {
                self.error("Invalid syntax. Interpreter:78");
            }
        }
    }
}

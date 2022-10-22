mod interpreter;
mod lexer;
mod symbols;
mod token;
mod utils;

use interpreter::Interpreter;
use lexer::Lexer;
use token::Token;

fn main() {
    println!("Simple math interpreter. Ex: 10 + 12 * (3 * 3)");

    loop {
        println!("\nExpression: ");

        let mut code = String::new();
        
        std::io::stdin().read_line(&mut code).unwrap();
        code = code.trim().to_string();

        let lexer = Lexer::new(&code);
        let mut interpreter = Interpreter::new(lexer);

        let res = interpreter.expr();

        println!("res: {}", res);
    }
    
    // debug
    // let lexer = Lexer::new("10 + 12 * (3 * 3)");
    // let mut interpreter = Interpreter::new(lexer);
    // let res = interpreter.expr();
    // println!("res: {}", res);
    
}

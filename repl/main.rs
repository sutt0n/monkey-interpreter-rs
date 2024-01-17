use std::io::{stdin, Write};

use monkey_lib::lexer::lexer::Lexer;
use monkey_lib::token::token::TokenEnum;

const PROMPT: &str = ">> ";

pub fn main() {
    
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let mut lexer = match stdin().read_line(&mut buffer) {
            Ok(_bytes) => {
                let lex = Lexer::new(&buffer);
                lex
            },
            Err(error) => panic!("Error: {}", error)
        };

        loop {
            let token = lexer.next_token();
            if token.token_type == TokenEnum::EOF {
                break;
            }

            println!("{:?}", token);
        }
    }
}

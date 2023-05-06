use std::{fs, error::Error};

use lang::{split_tokens, initial_tokenize, parse_blocks};

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("code.txt")?;
    
    let split_data: Vec<String> = split_tokens(&data);
    let tokens: Vec<lang::Token> = initial_tokenize(split_data);
    let code: lang::Block = parse_blocks(tokens);

    println!("code: {:#?}", code);
    

    Ok(())
}

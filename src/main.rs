use std::{fs, error::Error};

use lang::lexer::Block;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("code.txt")?;
    
    let code = Block::from_string(data);

    println!("code: {:#?}", code);
    
    Ok(())
}

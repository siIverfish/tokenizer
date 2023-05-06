use std::{fs, error::Error};


#[derive(Debug, PartialEq)]
enum BlockKind {
    Code,
    Data,
}

#[derive(Debug, PartialEq)]
struct Block {
    tokens: Vec<Token>,
    kind: BlockKind
}

impl Block {
    fn new(kind: BlockKind) -> Self {
        let tokens: Vec<Token> = Vec::new();

        Block { tokens, kind }
    }

    fn new_code() -> Self {
        Self::new(BlockKind::Code)
    }
    
    fn new_data() -> Self {
        Self::new(BlockKind::Data)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Name(String),
    Value(String),
    StartFunction,
    Set,
    End,
    None,

    OpenBlock(BlockKind),
    CloseBlock(BlockKind),

    Block(Block),
    Function(Block, Block),
}

impl Token {
    fn from_string(string: &str) -> Token {
        match string.trim() {
            "{" => Token::OpenBlock (BlockKind::Code),
            "}" => Token::CloseBlock(BlockKind::Code),
            "(" => Token::OpenBlock (BlockKind::Data),
            ")" => Token::CloseBlock(BlockKind::Data),

            ";"  => Token::End,
            ":=" => Token::Set,
            ""   => Token::None,

            "function" => Token::StartFunction,

            string if string.ends_with("\"") && string.starts_with("\"") => Token::Value(string.to_string()),
            string => Token::Name(string.to_string()),
        }
    }
}

fn lex(string: String) {
    let tokens: Vec<Token> = string.split([' ', '\n'])
        .map(Token::from_string)
        .filter(|x| x != &Token::None)
        .collect();

    let mut blocks: Vec<Block> = vec![Block::new_code()];

    for token in tokens.into_iter() {
        match token {
            Token::OpenBlock(kind) => blocks.push(Block::new(kind)),
            Token::CloseBlock(kind) => {
                let last_block = blocks.pop().expect("unmatched parentheses?");
                assert_eq!(last_block.kind, kind, "mismatched parentheses?");
                blocks.last_mut().expect("unmatched parentheses?").tokens.push(Token::Block(last_block));
            },

            _ => blocks.last_mut().expect("unmatched parentheses?").tokens.push(token),
        }
    }

    for block in blocks {
        println!("token: {:?}", block);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("code.txt")?;
    lex(data);

    Ok(())
}

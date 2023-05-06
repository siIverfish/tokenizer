#[derive(Debug, PartialEq)]
pub enum BlockKind {
    Code,
    Data,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    tokens: Vec<Token>,
    kind: BlockKind
}

impl Block {
    fn new(kind: BlockKind) -> Self {
        let tokens: Vec<Token> = Vec::new();

        Block { tokens, kind }
    }

    pub fn from_string(string: String) -> Self {
        let split_data: Vec<String> = split_tokens(&string);
        let tokens: Vec<Token> = initial_tokenize(split_data);
        let code: Block = parse_blocks(tokens);
        let code: Block = parse_functions(code);

        code
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Name(String),
    Value(String),
    StartFunction,
    Set,
    End,
    None,

    OpenBlock(BlockKind),
    CloseBlock(BlockKind),

    BlockToken(Block),
    Function { data: Block, code: Block }, // todo make function be work
}

impl Token {
    fn from_string(string: String) -> Token {
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

pub fn split_tokens(string: &String) -> Vec<String> {
    let parentheses_and_semicolons: String = String::from("(){};");

    let mut chars: Vec<char> = string.chars().rev().collect();
    let mut token_strs: Vec<String> = Vec::new();

    // reversed for speed
    while !chars.is_empty() {
        match chars.as_slice() {
            // we don't need spaces or newlines
            &[.., ' ' | '\n'] => { chars.pop(); },
            
            // special character cases to immediately stop
            &[.., c] if parentheses_and_semicolons.contains(c) => { 
                chars.pop(); 
                token_strs.push(c.to_string());
            }

            // the set symbol (:=)
            &[.., '=',':'] => {
                chars.truncate( chars.len().saturating_sub(2) );
                token_strs.push(":=".to_string());
            }

            // 'function' keyword... this is very haunted
            &[.., 'n', 'o', 'i', 't', 'c', 'n', 'u', 'f'] => {
                chars.truncate( chars.len().saturating_sub(8) );
                token_strs.push("function".to_string());
            }

            // process a string value
            &[.., '"'] => {
                let mut string: String = String::new();
                string.push(chars.pop().expect("unreachable")); // remove first quotes
                while chars.last().expect("unterminated string literal") != &'"' {
                    string.push(chars.pop().expect("unterminated string literal"));
                }
                string.push(chars.pop().expect("unreachable i think, maybe unterminated string literal or smthng")); // remove last quotes
                token_strs.push(string);
            }

            // probably a name
            _ => {
                let mut name = String::new();
                while chars.last().map_or(false, |c: &char| c.is_alphabetic()) {
                    name.push( chars.pop().expect("pretty sure no one will see this error message.") );
                }
                token_strs.push(name);
            },
        }
    }

    println!("tokens from `split_tokens`: {:?}", token_strs);//.iter().rev().collect::<Vec<_>>());

    token_strs
}

pub fn initial_tokenize(strings: Vec<String>) -> Vec<Token> {
    let tokens: Vec<Token> = strings
        .into_iter()
        .map(Token::from_string)
        .filter(|x| x != &Token::None)
        .collect();

    tokens
}

// todo this always returns 1-len vec i think
// blocks are recursive anyways
pub fn parse_blocks(tokens: Vec<Token>) -> Block {
    let mut blocks: Vec<Block> = vec![Block::new(BlockKind::Code)];

    for token in tokens.into_iter() {
        match token {
            Token::OpenBlock(kind) => blocks.push(Block::new(kind)),
            Token::CloseBlock(kind) => {
                let last_block = blocks.pop().expect("unmatched parentheses?");
                assert_eq!(last_block.kind, kind, "mismatched parentheses?");
                blocks.last_mut().expect("unmatched parentheses?").tokens.push(Token::BlockToken(last_block));
            },

            _ => blocks.last_mut().expect("unmatched parentheses?").tokens.push(token),
        }
    }

    assert_eq!(blocks.len(), 1, "um you done messed something up with the code blocks or something");

    blocks.into_iter().nth(0).unwrap()
}

pub fn parse_functions(block: Block) -> Block {
    let mut new_block = Block::new(BlockKind::Code);
    let mut iter = block.tokens.into_iter();

    while let Some(token) = iter.next() {
        match token {
            Token::StartFunction => {
                let data_block = if let Some(Token::BlockToken (block)) = iter.next() {
                    assert_eq!(block.kind, BlockKind::Data, "function missing data block");
                    block
                } else { panic!("function missing data block") };

                let code_block = if let Some(Token::BlockToken (block)) = iter.next() {
                    assert_eq!(block.kind, BlockKind::Code, "function missing code block");
                    block
                } else { panic!("function missing code block") };

                let function = Token::Function { data: data_block, code: code_block };

                new_block.tokens.push(function);
            },
            _ => new_block.tokens.push(token),
        }
    }

    new_block
}
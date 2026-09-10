pub enum Token {
    LBrace(BraceType),
    RBrace(BraceType),
    Identifier(String),
    Literal(Literal),
    Semicolon,
}

pub enum BraceType {
    Paren,
    Square,
    Curly,
}

pub enum Literal {
    Number(String),
    String(String),
}

pub enum TokenizeError {
    NumberDecimalDividerDoubled
}

pub fn tokenize(input:&str) -> Result<Vec<Token>, TokenizeError> {
    let mut out = vec![];
    let mut input = input.chars().peekable();
    let mut state = State::Idle;
    loop {
        let Some(char) = input.next() else {break};
        consume_char(char, &mut out, &mut state)?;
        manage_state(char, &mut out, &mut state)?;
    }
    Ok(out)
}

enum State {
    Idle,
    Comment,
    SymbolIdentifier{buffer:String},
    WordIdentifier{buffer:String},
    NumberLiteral{buffer_whole:String, buffer_fraction:String, decimal_placed:bool},
    StringLiteral{buffer:String, place:StringLiteralPlace},
}

enum StringLiteralPlace {
    Open,
    Middle{escaped:bool},
    Close,
}

fn consume_char(char:char, out:&mut Vec<Token>, state:&mut State) -> Result<(),TokenizeError> {
    match state {
        State::Idle => match char {
            '(' => out.push(Token::LBrace(BraceType::Paren)),
            '[' => out.push(Token::LBrace(BraceType::Square)),
            '{' => out.push(Token::LBrace(BraceType::Curly)),
            ')' => out.push(Token::RBrace(BraceType::Paren)),
            ']' => out.push(Token::RBrace(BraceType::Square)),
            '}' => out.push(Token::RBrace(BraceType::Curly)),
            ';' => out.push(Token::Semicolon),
            _ => {}
        },
        State::Comment => {},
        State::SymbolIdentifier { buffer } => buffer.push(char),
        State::WordIdentifier { buffer } => buffer.push(char),
        State::NumberLiteral { buffer_whole, buffer_fraction, decimal_placed } => match (char == '.', *decimal_placed) {
            (false, false) => buffer_whole.push(char),
            (false, true) => buffer_fraction.push(char),
            (true, false) => {},
            (true, true) => return Err(TokenizeError::NumberDecimalDividerDoubled),
        },
        State::StringLiteral { buffer, place } => match place {
            StringLiteralPlace::Middle{escaped} => if char != '\\' || *escaped {buffer.push(char)},
            _ => {},
        },
    }
    Ok(())
}

fn manage_state(char:char, out:&mut Vec<Token>, state:&mut State) -> Result<(), TokenizeError> {
    todo!()
}


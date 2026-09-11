#![expect(dead_code)]

use peek_again::Peekable;

#[derive(Debug)]
pub enum Token {
    LBrace(BraceType),
    RBrace(BraceType),
    Identifier(String),
    Literal(Literal),
    Semicolon,
}

#[derive(Debug)]
pub enum BraceType {
    Paren,
    Square,
    Curly,
}

#[derive(Debug)]
pub enum Literal {
    Number(String, Option<String>),
    String(String),
}

#[derive(Debug)]
pub enum TokenizeError {
    NumberDecimalDividerDoubled,
    InvalidSymbol(char),
    StringLiteralUnclosed,
    StringLiteralPointlessEscape,
}

pub fn tokenize(input:&str) -> Result<Vec<Token>, TokenizeError> {
    let mut out = vec![];
    let mut input = Peekable::new(input.chars());
    let mut state = State::Idle;
    loop {
        let Some(char) = input.next() else {break};
        let peek = input.peek().get().copied();
        let peek_2 = input.peek_2().copied();
        update_state(char, peek, &mut state)?;
        consume_char(char, &mut out, &mut state)?;
        state = possibly_complete_token(peek, peek_2, &mut out, state)?;
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

// only Idle changes state type here
// everything else changes back to Idle when they complete a token
fn update_state(char:char, peek:Option<char>, state:&mut State) -> Result<(),TokenizeError> {
    match state {
        State::Idle => {
            match (char, peek) {
                ('('|')'|'['|']'|'{'|'}'|';', _) => {},
                _ if char.is_whitespace() => {},
                ('/', Some('/')) => *state = State::Comment,
                ('.', Some('0'..'9')) | ('0'..'9', _) => *state = State::NumberLiteral { buffer_whole: String::new(), buffer_fraction: String::new(), decimal_placed: false },
                _ if is_symbol_identifier(char) => *state = State::SymbolIdentifier { buffer: String::new() },
                _ if is_word_identifier_start(char) => *state = State::WordIdentifier { buffer: String::new() },
                ('"', _) => *state = State::StringLiteral { buffer: String::new(), place: StringLiteralPlace::Open },
                _ => return Err(TokenizeError::InvalidSymbol(char))
            }
        },
        _ => {}
    }
    Ok(())
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
            (true, false) => *decimal_placed = true,
            (true, true) => unreachable!(),
        },
        State::StringLiteral { buffer, place } => match place {
            StringLiteralPlace::Open => *place = StringLiteralPlace::Middle { escaped: false },
            StringLiteralPlace::Middle { escaped } => {
                match (*escaped, char) {
                    (false, '\\') => *escaped = true,
                    (false, '"') => {},
                    (true, '\\' | '"') | (false, _) => {
                        buffer.push(char);
                        *escaped = false;
                    },
                    (true, _) => return Err(TokenizeError::StringLiteralPointlessEscape),
                }
            },
            _ => {},
        },
    }
    Ok(())
}

fn possibly_complete_token(peek:Option<char>, peek_2:Option<char>, out:&mut Vec<Token>, mut state:State) -> Result<State, TokenizeError> {
    if should_complete_token(&mut state, peek, peek_2)? {
        complete_token(state, out);
        state = State::Idle;
    }
    Ok(state)
}

fn should_complete_token(state:&mut State, peek:Option<char>, peek_2:Option<char>) -> Result<bool, TokenizeError> {
    Ok(match state {
        State::Idle => false,
        State::Comment => peek == Some('\n'),
        State::SymbolIdentifier {buffer:_} => !peek.is_some_and(is_symbol_identifier),
        State::WordIdentifier {buffer:_} => !peek.is_some_and(is_word_identifier_continue),
        State::NumberLiteral {buffer_whole:_, buffer_fraction:_, decimal_placed} => {
            enum Classification {None, Whitespace, Digit, Dot, Symbol, Word, Other}
            fn classify(peek:Option<char>) -> Result<Classification, TokenizeError> {
                Ok(match peek {
                    None => Classification::None,
                    Some(c) if c.is_whitespace() => Classification::Whitespace,
                    Some('0'..'9') => Classification::Digit,
                    Some('.') => Classification::Dot,
                    Some(c) if is_symbol_identifier(c) => Classification::Symbol,
                    Some(c) if is_word_identifier_start(c) => Classification::Word,
                    Some(_) => Classification::Other,
                })
            }
            match (*decimal_placed, classify(peek)?, classify(peek_2)?) {
                (_, Classification::None | Classification::Whitespace | Classification::Symbol | Classification::Word | Classification::Other, _)
                | (_, Classification::Dot, Classification::Word) => true, // 1.2|.a or 1|.a
                (true, Classification::Dot, Classification::Digit) => return Err(TokenizeError::NumberDecimalDividerDoubled), // 1.2|.3 or 1.|.3
                (true, Classification::Dot, _) => true, // 1.2|.
                _ => false,
            }
        },
        State::StringLiteral { buffer:_, place } => match place {
            StringLiteralPlace::Close => true,
            StringLiteralPlace::Middle { escaped:false } if peek == Some('"') => {
                *place = StringLiteralPlace::Close; // TODO: i dont like that this is done here, but i dont know where else to put it
                false
            },
            _ if peek.is_none() => return Err(TokenizeError::StringLiteralUnclosed),
            _ => false,
        },
    })
}

fn complete_token(state:State, out:&mut Vec<Token>) {
    match state {
        State::Idle => unreachable!(),
        State::Comment => {},
        State::SymbolIdentifier { buffer }
        | State::WordIdentifier { buffer } => out.push(Token::Identifier(buffer)),
        State::NumberLiteral { buffer_whole, buffer_fraction, decimal_placed }
            => out.push(Token::Literal(Literal::Number(buffer_whole, decimal_placed.then(|| buffer_fraction)))),
        State::StringLiteral { buffer, place:_ }
            => out.push(Token::Literal(Literal::String(buffer)))
    }
}

fn is_symbol_identifier(char:char) -> bool {
    "[`-=\\,./~!@#$%^&*_+|:<>?".contains(char)
}

fn is_word_identifier_start(char:char) -> bool {
    char.is_alphabetic() || char == '_'
}

fn is_word_identifier_continue(char:char) -> bool {
    char.is_alphanumeric() || char == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tokenize("hi;").unwrap();
        std::assert_matches!(result[..], [Token::Identifier(_), Token::Semicolon]);
    }
}


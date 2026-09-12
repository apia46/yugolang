mod lexer;
use lexer::{BraceType, Token, TokenizeError};

#[derive(Debug)]
pub struct Scope {
    pub statements:Vec<Statement>,
    pub result:Option<Statement>,
}

#[derive(Debug)]
pub struct Statement(pub Vec<Expression>);

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Scope(Box<Scope>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedBrace,
    UnmatchedBraces,
    UnopenedBrace,
    NumberParseError(String),
    NumberDecimalDividerDoubled,
    StringLiteralPointlessEscape,
    StringLiteralUnclosed,
    InvalidSymbol(char),
}

pub fn parse(input:&str) -> Result<Scope, ParseError> {
    let tokens = lexer::tokenize(input).or_else(|e| Err(match e {
        TokenizeError::NumberDecimalDividerDoubled => ParseError::NumberDecimalDividerDoubled,
        TokenizeError::StringLiteralPointlessEscape => ParseError::StringLiteralPointlessEscape,
        TokenizeError::StringLiteralUnclosed => ParseError::StringLiteralUnclosed,
        TokenizeError::InvalidSymbol(char) => ParseError::InvalidSymbol(char),
    }))?;
    parse_tokens(tokens)
}

fn parse_tokens(tokens:Vec<Token>) -> Result<Scope, ParseError> {
    Ok(parse_scope(tokens.into_iter(), None)?.1)
}

type TokensIter = std::vec::IntoIter<Token>;

fn parse_scope(mut tokens:TokensIter, until:Option<BraceType>) -> Result<(TokensIter, Scope), ParseError> {
    let mut statements = vec![];
    let mut buffer = vec![];
    loop {
        let Some(token) = tokens.next() else {
            if until.is_none() { break }
            else { return Err(ParseError::UnclosedBrace) }
        };
        match token {
            Token::LBrace(brace_type) => {
                let (next_tokens, scope) = parse_scope(tokens, Some(brace_type))?;
                tokens = next_tokens;
                buffer.push(Expression::Scope(Box::new(scope)));
            },
            Token::RBrace(brace_type) => {
                if let Some(expected) = until {
                    if expected == brace_type { break }
                    else { return Err(ParseError::UnmatchedBraces) }
                } else { return Err(ParseError::UnopenedBrace) }
            },
            Token::Semicolon => {
                statements.push(Statement(buffer));
                buffer = vec![];
            },
            Token::Identifier(identifier) => buffer.push(Expression::Identifier(identifier)),
            Token::Literal(literal) => buffer.push(Expression::Literal(match literal {
                lexer::Literal::Number(mut whole_part, fractional_part) => {
                    if whole_part.is_empty() { whole_part = "0".into() }
                    if let Some(mut fractional_part) = fractional_part {
                        if fractional_part.is_empty() { fractional_part = "0".into() }
                        let total = whole_part + "+" + &fractional_part;
                        Literal::Float(total.parse::<f64>().or_else(|_| Err(ParseError::NumberParseError(total)))?)
                    } else {Literal::Int(whole_part.parse::<i64>().or_else(|_| Err(ParseError::NumberParseError(whole_part)))?)}
                },
                lexer::Literal::String(string) => Literal::String(string),
            })),
        }
    }
    Ok((tokens, Scope {
        statements,
        result: (!buffer.is_empty()).then_some(Statement(buffer)),
    }))
}


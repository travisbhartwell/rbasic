#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Comment(String),

    // Variables and Literals
    Variable(String),
    Number(i32),
    BString(String),

    // Binary Operators
    Equals,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
    Multiply,
    Divide,
    Minus,
    Plus,

    // Parens
    LParen,
    RParen,

    // Unary Operators
    Bang,
    UMinus,

    // Keywords
    Goto,
    If,
    Input,
    Let,
    Print,
    Rem,
    Then,
}

impl Token {
    pub fn token_for_string(token_str: &str) -> Option<Token> {
        match token_str {
            "=" => Some(Token::Equals),
            "<" => Some(Token::LessThan),
            ">" => Some(Token::GreaterThan),
            "<=" => Some(Token::LessThanEqual),
            ">=" => Some(Token::GreaterThanEqual),
            "<>" => Some(Token::NotEqual),
            "*" => Some(Token::Multiply),
            "/" => Some(Token::Divide),
            // Yes, this is also Token::UMinus
            "-" => Some(Token::Minus),
            "+" => Some(Token::Plus),
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),
            "!" => Some(Token::Bang),
            "GOTO" => Some(Token::Goto),
            "IF" => Some(Token::If),
            "INPUT" => Some(Token::Input),
            "LET" => Some(Token::Let),
            "PRINT" => Some(Token::Print),
            "REM" => Some(Token::Rem),
            "THEN" => Some(Token::Then),
            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        match *self {
            Token::Equals | Token::LessThan | Token::GreaterThan | Token::LessThanEqual |
            Token::NotEqual | Token::Multiply | Token::Divide | Token::Minus | Token::Plus => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match *self {
            Token::Variable(_) |
            Token::Number(_) |
            Token::BString(_) => true,
            _ => false,
        }
    }

    pub fn operator_precedence(&self) -> Result<u8, String> {
        match *self {
            Token::Multiply | Token::Divide => Ok(10),
            Token::Minus | Token::Plus => Ok(8),
            Token::Equals | Token::LessThan | Token::GreaterThan | Token::LessThanEqual |
            Token::NotEqual => Ok(4),
            _ => Err("Not an operator".to_string()),
        }
    }
}

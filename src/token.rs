#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

/*
Input: var myVariable = 42;
[
    Token { token_type: TokenType::Var, lexeme: "var", literal: None, line: 1 },
    Token { token_type: TokenType::Identifier, lexeme: "myVariable", literal: None, line: 1 },
    Token { token_type: TokenType::Equal, lexeme: "=", literal: None, line: 1 },
    Token { token_type: TokenType::Number, lexeme: "42", literal: Some(LiteralValue::Number(42.0)), line: 1 },
    Token { token_type: TokenType::Semicolon, lexeme: ";", literal: None, line: 1 },
]

Input: print userName;
[
    Token { token_type: TokenType::Print, lexeme: "print", literal: None, line: 1 },
    Token { token_type: TokenType::Identifier, lexeme: "userName", literal: None, line: 1 },
    Token { token_type: TokenType::Semicolon, lexeme: ";", literal: None, line: 1 },
]
*/
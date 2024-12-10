use crate::literal::Literal;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum TokenType {
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
    LeftSquare,
    RightSquare,
    Modulo,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    ElseIf,
    False,
    Fun,
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
    For,
    Break,
    Continue,
    Inherits,

    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    Increment,
    Decrement,

    Eof,
}

impl TokenType {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "aur" => Some(Self::And),
            "jamat" => Some(Self::Class),
            "warna" => Some(Self::Else),
            "agarwarna" => Some(Self::ElseIf), // Not yet implemented
            "jhoot" => Some(Self::False),
            "har" => Some(Self::For),
            "kaam" => Some(Self::Fun),
            "agar" => Some(Self::If),
            "khali" => Some(Self::Nil),
            "ya" => Some(Self::Or),
            "bolo" => Some(Self::Print),
            "wapis" => Some(Self::Return),
            "asli" => Some(Self::Super),
            "yeh" => Some(Self::This),
            "sach" => Some(Self::True),
            "rakho" => Some(Self::Var),
            "jabtak" => Some(Self::While),
            "ifta" => Some(Self::Break), // Not yet implemented
            "safar" => Some(Self::Continue), // Not yet implemented
            "na" => Some(Self::Bang),
            "shamil" => Some(Self::Inherits),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: i64,
    pub position: i64,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        return Self {
            token_type: self.token_type,
            lexeme: self.lexeme.to_string(),
            literal: self.literal.clone(),
            line: self.line,
            position: self.position,
        };
    }
}

impl Token {
    pub fn init(
        token_type: TokenType,
        lexeme: &String,
        literal: Option<Literal>,
        line: i64,
        position: i64,
    ) -> Self {
        return Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
            position,
        };
    }

    pub fn dummy() -> Self {
        return Self {
            token_type: TokenType::Eof,
            lexeme: String::from("dummy"),
            literal: None,
            line: -1,
            position: 0,
        };
    }

    pub fn copy(token: &Token) -> Self {
        return Self {
            token_type: token.token_type,
            lexeme: token.lexeme.to_string(),
            literal: token.literal.clone(),
            line: token.line,
            position: token.position,
        };
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{:?} {} {:?}",
            self.token_type,
            self.lexeme,
            Literal::option_string(self.literal.clone())
        );
    }
}

use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,
    Integer,
    Decimal,
    String,
    Comment,
    Underscore,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Modulo,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    HashLBrace,
    RBrace,
    LBracket,
    RBracket,
    Backtick,

    Pipe,
    PipePipe,
    AmpAmp,
    PipeGreater,
    GreaterGreater,

    DotDot,
    DotDotEqual,

    Mutable,
    Match,
    Let,
    If,
    Else,
    Return,
    Break,
    Nil,
    True,
    False,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token {
            kind,
            location: Location { start, end },
        }
    }

    pub fn keyword(id: &str) -> Option<TokenKind> {
        match id {
            "mut" => Some(TokenKind::Mutable),
            "match" => Some(TokenKind::Match),
            "let" => Some(TokenKind::Let),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "return" => Some(TokenKind::Return),
            "break" => Some(TokenKind::Break),
            "nil" => Some(TokenKind::Nil),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            _ => None,
        }
    }

    pub fn location_range(&self, token: &Token) -> Location {
        Location {
            start: self.location.start,
            end: token.location.start,
        }
    }
}

#[macro_export]
macro_rules! T {
    [ILLEGAL] => { $crate::lexer::TokenKind::Illegal };
    [EOF] => { $crate::lexer::TokenKind::Eof };

    [ID] => { $crate::lexer::TokenKind::Identifier };
    [INT] => { $crate::lexer::TokenKind::Integer };
    [DEC] => { $crate::lexer::TokenKind::Decimal };
    [STR] => { $crate::lexer::TokenKind::String };
    [CMT] => { $crate::lexer::TokenKind::Comment };
    [_] => { $crate::lexer::TokenKind::Underscore };

    [=] => { $crate::lexer::TokenKind::Assign };
    [+] => { $crate::lexer::TokenKind::Plus };
    [-] => { $crate::lexer::TokenKind::Minus };
    [!] => { $crate::lexer::TokenKind::Bang };
    [*] => { $crate::lexer::TokenKind::Asterisk };
    [/] => { $crate::lexer::TokenKind::Slash };
    [%] => { $crate::lexer::TokenKind::Modulo };

    [==] => { $crate::lexer::TokenKind::Equal };
    [!=] => { $crate::lexer::TokenKind::NotEqual };
    [<] => { $crate::lexer::TokenKind::LessThan };
    [<=] => { $crate::lexer::TokenKind::LessThanEqual };
    [>] => { $crate::lexer::TokenKind::GreaterThan };
    [>=] => { $crate::lexer::TokenKind::GreaterThanEqual };

    [,] => { $crate::lexer::TokenKind::Comma };
    [;] => { $crate::lexer::TokenKind::Semicolon };
    [:] => { $crate::lexer::TokenKind::Colon };

    ['('] => { $crate::lexer::TokenKind::LParen };
    [')'] => { $crate::lexer::TokenKind::RParen };
    ['{'] => { $crate::lexer::TokenKind::LBrace };
    ["#{"] => { $crate::lexer::TokenKind::HashLBrace };
    ['}'] => { $crate::lexer::TokenKind::RBrace };
    ['['] => { $crate::lexer::TokenKind::LBracket };
    [']'] => { $crate::lexer::TokenKind::RBracket };
    ['`'] => { $crate::lexer::TokenKind::Backtick };

    [|] => { $crate::lexer::TokenKind::Pipe };
    [||] => { $crate::lexer::TokenKind::PipePipe };
    [&&] => { $crate::lexer::TokenKind::AmpAmp };
    [|>] => { $crate::lexer::TokenKind::PipeGreater };
    [>>] => { $crate::lexer::TokenKind::GreaterGreater };

    [..] => { $crate::lexer::TokenKind::DotDot };
    [..=] => { $crate::lexer::TokenKind::DotDotEqual };

    [MUT] => { $crate::lexer::TokenKind::Mutable };
    [MATCH] => { $crate::lexer::TokenKind::Match };
    [LET] => { $crate::lexer::TokenKind::Let };
    [IF] => { $crate::lexer::TokenKind::If };
    [ELSE] => { $crate::lexer::TokenKind::Else };
    [RETURN] => { $crate::lexer::TokenKind::Return };
    [BREAK] => { $crate::lexer::TokenKind::Break };
    [NIL] => { $crate::lexer::TokenKind::Nil };
    [TRUE] => { $crate::lexer::TokenKind::True };
    [FALSE] => { $crate::lexer::TokenKind::False };
}

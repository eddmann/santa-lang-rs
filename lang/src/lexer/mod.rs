mod token;

#[cfg(test)]
mod tests;

const EOF_CHAR: char = '\0';

use crate::T;
use std::iter::Peekable;
use std::str::Chars;
pub use token::{Location, Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    remaining_chars: Peekable<Chars<'a>>,
    token_buffer: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            remaining_chars: input.chars().peekable(),
            token_buffer: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.token_buffer {
            self.token_buffer = None;
            return token;
        }

        self.consume_while(|ch| ch.is_whitespace());

        let start = self.position;

        let token = match self.consume() {
            '=' => match self.peek() {
                '=' => {
                    self.consume();
                    T![==]
                }
                _ => T![=],
            },
            '!' => match self.peek() {
                '=' => {
                    self.consume();
                    T![!=]
                }
                _ => T![!],
            },
            '<' => match self.peek() {
                '=' => {
                    self.consume();
                    T![<=]
                }
                _ => T![<],
            },
            '>' => match self.peek() {
                '=' => {
                    self.consume();
                    T![>=]
                }
                '>' => {
                    self.consume();
                    T![>>]
                }
                _ => T![>],
            },
            '&' => match self.peek() {
                '&' => {
                    self.consume();
                    T![&&]
                }
                _ => T![ILLEGAL],
            },
            '|' => match self.peek() {
                '|' => {
                    self.consume();
                    T![||]
                }
                '>' => {
                    self.consume();
                    T![|>]
                }
                _ => T![|],
            },
            '#' => match self.peek() {
                '{' => {
                    self.consume();
                    T!["#{"]
                }
                _ => T![ILLEGAL],
            },
            '.' => match self.peek() {
                '.' => {
                    self.consume();
                    match self.peek() {
                        '=' => {
                            self.consume();
                            T![..=]
                        }
                        _ => T![..],
                    }
                }
                _ => T![ILLEGAL],
            },

            '+' => T![+],
            '-' => T![-],
            '*' => T![*],
            '/' => match self.peek() {
                '/' => self.consume_comment(),
                _ => T![/],
            },
            '%' => T![%],

            ';' => T![;],
            ',' => T![,],
            ':' => T![:],
            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => T!['['],
            ']' => T![']'],
            '_' => T![_],
            '@' => T![@],

            '`' => self.consume_backtick(),
            '"' => self.consume_string(),
            '0'..='9' => return self.consume_number(start),
            'a'..='z' | 'A'..='Z' => self.consume_identifier_or_keyword(start),

            EOF_CHAR => T![EOF],
            _ => T![ILLEGAL],
        };

        Token::new(token, start, self.position)
    }

    fn consume_backtick(&mut self) -> TokenKind {
        loop {
            match self.consume() {
                EOF_CHAR => return T![ILLEGAL],
                '`' => {
                    break;
                }
                _ => {}
            }
        }

        T!['`']
    }

    fn consume_string(&mut self) -> TokenKind {
        loop {
            match (self.consume(), self.peek()) {
                (EOF_CHAR, _) => return T![ILLEGAL],
                ('\\', '\\') | ('\\', '"') | ('\\', 'r') | ('\\', 'n') | ('\\', 't') => {
                    self.consume();
                }
                ('"', _) => {
                    break;
                }
                _ => {}
            }
        }

        T![STR]
    }

    fn consume_number(&mut self, start: usize) -> Token {
        self.consume_while(|ch| matches!(ch, '0'..='9' | '_'));

        if self.peek() != '.' {
            return Token::new(T![INT], start, self.position);
        }

        let position = self.position;
        self.consume();

        if self.peek() == '.' {
            self.consume();
            if self.peek() == '=' {
                self.consume();
                self.token_buffer = Some(Token::new(T![..=], position, self.position));
            } else {
                self.token_buffer = Some(Token::new(T![..], position, self.position));
            }

            return Token::new(T![INT], start, position);
        }

        self.consume_while(|ch| matches!(ch, '0'..='9' | '_'));

        Token::new(T![DEC], start, self.position)
    }

    fn consume_identifier_or_keyword(&mut self, start: usize) -> TokenKind {
        self.consume_while(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '?'));

        let source = &self.input[start..self.position];

        match Token::keyword(source) {
            Some(keyword) => keyword,
            None => T![ID],
        }
    }

    fn consume_comment(&mut self) -> TokenKind {
        self.consume_while(|ch| !matches!(ch, '\n' | EOF_CHAR));
        T![CMT]
    }

    fn consume(&mut self) -> char {
        match self.remaining_chars.next() {
            Some(ch) => {
                self.position += ch.len_utf8();
                ch
            }
            None => EOF_CHAR,
        }
    }

    fn peek(&mut self) -> char {
        *self.remaining_chars.peek().unwrap_or(&EOF_CHAR)
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) {
            self.consume();
        }
    }

    pub fn get_source(&self, token: &Token) -> &'a str {
        &self.input[token.source.start..token.source.end]
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            token if token.kind == T![EOF] => None,
            token => Some(token),
        }
    }
}

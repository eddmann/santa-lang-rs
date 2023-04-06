pub mod ast;

#[cfg(test)]
mod tests;

use super::lexer::{Lexer, Location, Token, TokenKind};
use crate::T;
use ast::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
enum Precedence {
    Lowest = 0,
    AndOr,
    Equals,
    LessGreater,
    Composition,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

#[derive(Debug)]
pub struct ParserErr {
    pub message: String,
    pub source: Location,
}

type RStatement = Result<Statement, ParserErr>;
type RExpression = Result<Expression, ParserErr>;
type RExpressions = Result<Vec<Expression>, ParserErr>;

#[inline]
fn infix_binding_precedence(token: &TokenKind) -> Precedence {
    match token {
        T![&&] | T![||] => Precedence::AndOr,
        T![==] | T![!=] | T![=] => Precedence::Equals,
        T![<] | T![<=] | T![>] | T![>=] => Precedence::LessGreater,
        T![>>] | T![|>] | T![..] | T![..=] => Precedence::Composition,
        T![+] | T![-] => Precedence::Sum,
        T![/] | T![*] | T![%] | T!['`'] => Precedence::Product,
        T!['('] => Precedence::Call,
        T!['['] => Precedence::Index,
        _ => Precedence::Lowest,
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            next_token,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserErr> {
        Ok(Program {
            statements: self.parse_statements()?,
        })
    }

    fn next_token(&mut self) {
        self.current_token = self.next_token;
        self.next_token = self.lexer.next_token();
    }

    fn consume_if(&mut self, kind: TokenKind) -> bool {
        if self.current_token.kind != kind {
            return false;
        }

        self.next_token();
        true
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParserErr> {
        if self.current_token.kind == kind {
            let token = self.current_token;
            self.next_token();
            return Ok(token);
        }

        Err(ParserErr {
            message: format!("Expected: {:?}, Actual: {:?}", kind, self.current_token.kind),
            source: self.current_token.source,
        })
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, ParserErr> {
        let mut statements: Vec<Statement> = Vec::new();

        while let Some(statement) = self.parse_statement()? {
            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, ParserErr> {
        match self.current_token.kind {
            T![RETURN] => Ok(Some(self.parse_return_statement()?)),
            T![BREAK] => Ok(Some(self.parse_break_statement()?)),
            T![CMT] => Ok(Some(self.parse_comment_statement()?)),
            T![ID] if self.next_token.kind == T![:] => Ok(Some(self.parse_section_statement()?)),
            T!['}'] | T![EOF] => Ok(None),
            T![ILLEGAL] => Err(ParserErr {
                source: self.current_token.source,
                message: "Illegal token".to_owned(),
            }),
            _ => Ok(Some(self.parse_expression_statement()?)),
        }
    }

    fn parse_return_statement(&mut self) -> RStatement {
        let start = self.expect(T![RETURN])?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.consume_if(T![;]);

        Ok(Statement {
            kind: StatementKind::Return(value),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_break_statement(&mut self) -> RStatement {
        let start = self.expect(T![BREAK])?;

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.consume_if(T![;]);

        Ok(Statement {
            kind: StatementKind::Break(value),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_comment_statement(&mut self) -> RStatement {
        let token = self.expect(T![CMT])?;
        let value = self.lexer.get_source(&token).to_string();

        Ok(Statement {
            kind: StatementKind::Comment(value),
            source: token.source,
        })
    }

    fn parse_section_statement(&mut self) -> RStatement {
        let token = self.expect(T![ID])?;
        let name = self.lexer.get_source(&token).to_string();
        self.expect(T![:])?;
        let body = Box::new(self.parse_block_statement()?);
        self.consume_if(T![;]);

        Ok(Statement {
            kind: StatementKind::Section { name, body },
            source: token.source_range(&self.current_token),
        })
    }

    fn parse_expression_statement(&mut self) -> RStatement {
        let start = self.current_token;

        let expression = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.consume_if(T![;]);

        Ok(Statement {
            kind: StatementKind::Expression(expression),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_block_statement(&mut self) -> RStatement {
        let start = self.current_token;

        let statements = if self.consume_if(T!['{']) {
            let statements = self.parse_statements()?;
            self.expect(T!['}'])?;
            statements
        } else {
            vec![self.parse_expression_statement()?]
        };

        Ok(Statement {
            kind: StatementKind::Block(statements),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> RExpression {
        let mut left = self.parse_prefix_expression()?;

        while precedence < infix_binding_precedence(&self.current_token.kind) {
            if let Some(expr) = self.parse_infix_expression(left.clone())? {
                left = expr;
            } else {
                return Ok(left);
            }
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> RExpression {
        match self.current_token.kind {
            T![ID] => {
                let start = self.current_token;

                let id = self.parse_identifier_expression()?;

                if self.current_token.kind != T![|] {
                    return Ok(id);
                }

                let trailing_lambda = self.parse_function_expression()?;

                Ok(Expression {
                    kind: ExpressionKind::Call {
                        function: Box::new(id),
                        arguments: vec![trailing_lambda],
                    },
                    source: start.source_range(&self.current_token),
                })
            }
            T![INT] => self.parse_integer_expression(),
            T![DEC] => self.parse_decimal_expression(),
            T![STR] => self.parse_string_expression(),
            T![TRUE] | T![FALSE] => self.parse_booleon_expression(),
            T![NIL] => Ok(Expression {
                kind: ExpressionKind::Nil,
                source: self.expect(T![NIL])?.source,
            }),
            T!['('] => self.parse_grouped_expression(),
            T!['['] => self.parse_list_expression(),
            T!['{'] => self.parse_set_expression(),
            T!["#{"] => self.parse_hash_expression(),
            T![IF] => self.parse_if_expression(),
            T![MATCH] => self.parse_match_expression(),
            T![|] | T![||] => self.parse_function_expression(),
            T![!] | T![-] => match &self.next_token.kind {
                T![ID] | T![!] | T![-] | T![INT] | T![DEC] | T![TRUE] | T![FALSE] | T!['('] => {
                    self.parse_prefix_operator_expression()
                }
                _ => self.parse_operator_identifier_expression(),
            },
            T![_] => Ok(Expression {
                kind: ExpressionKind::Placeholder,
                source: self.expect(T![_])?.source,
            }),
            T![LET] => self.parse_let_expression(),
            _ => self.parse_operator_identifier_expression(),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Option<Expression>, ParserErr> {
        match self.current_token.kind {
            T![==]
            | T![!=]
            | T![<]
            | T![<=]
            | T![>]
            | T![>=]
            | T![+]
            | T![-]
            | T![/]
            | T![*]
            | T![%]
            | T![||]
            | T![&&]
            | T!['`'] => Ok(Some(self.parse_infix_operator_expression(left)?)),
            T!['('] => Ok(Some(self.parse_call_expression(left)?)),
            T!['['] => Ok(Some(self.parse_index_expression(left)?)),
            T![..] => Ok(Some(self.parse_exclusive_range_expression(left)?)),
            T![..=] => Ok(Some(self.parse_inclusive_range_expression(left)?)),
            T![=] => Ok(Some(self.parse_assignment_expression(left)?)),
            T![>>] => Ok(Some(self.parse_function_composition_expression(left)?)),
            T![|>] => Ok(Some(self.parse_function_threading_expression(left)?)),
            _ => Ok(None),
        }
    }

    fn parse_prefix_operator_expression(&mut self) -> RExpression {
        let start = self.current_token;

        let operator = match &self.current_token.kind {
            T![!] => Prefix::Bang,
            T![-] => Prefix::Minus,
            _ => {
                return Err(ParserErr {
                    message: format!("{:?} is not legal in the prefix position", self.current_token.kind),
                    source: self.current_token.source,
                })
            }
        };
        self.next_token();

        let right = Box::new(self.parse_expression(Precedence::Prefix)?);

        Ok(Expression {
            kind: ExpressionKind::Prefix { operator, right },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_infix_operator_expression(&mut self, left: Expression) -> RExpression {
        let token = self.current_token;

        let operator = match &token.kind {
            T![+] => Infix::Plus,
            T![-] => Infix::Minus,
            T![*] => Infix::Asterisk,
            T![/] => Infix::Slash,
            T![==] => Infix::Equal,
            T![%] => Infix::Modulo,
            T![!=] => Infix::NotEqual,
            T![<] => Infix::LessThan,
            T![<=] => Infix::LessThanEqual,
            T![>] => Infix::GreaterThan,
            T![>=] => Infix::GreaterThanEqual,
            T![||] => Infix::Or,
            T![&&] => Infix::And,
            T!['`'] => {
                let name = self.lexer.get_source(&token);
                let unquoted = &name[1..name.len() - 1];
                Infix::Call(Box::new(Expression {
                    kind: ExpressionKind::Identifier(unquoted.to_owned()),
                    source: token.source,
                }))
            }
            _ => {
                return Err(ParserErr {
                    message: format!("{:?} is not legal in the infix position", self.current_token.kind),
                    source: self.current_token.source,
                })
            }
        };
        self.next_token();

        let right = Box::new(self.parse_expression(infix_binding_precedence(&token.kind))?);

        Ok(Expression {
            kind: ExpressionKind::Infix {
                operator,
                left: Box::new(left),
                right,
            },
            source: token.source_range(&self.current_token),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> RExpression {
        let start = self.expect(T!['('])?;

        let mut arguments = self.parse_arguments(T![')'])?;
        if self.current_token.kind == T![|] {
            arguments.push(self.parse_function_expression()?);
        }

        Ok(Expression {
            kind: ExpressionKind::Call {
                function: Box::new(function),
                arguments,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_index_expression(&mut self, left: Expression) -> RExpression {
        let start = self.expect(T!['['])?;

        let index = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.expect(T![']'])?;

        Ok(Expression {
            kind: ExpressionKind::Index {
                left: Box::new(left),
                index,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_identifier_expression(&mut self) -> RExpression {
        let token = self.expect(T![ID])?;
        let name = self.lexer.get_source(&token).to_string();

        Ok(Expression {
            kind: ExpressionKind::Identifier(name),
            source: token.source,
        })
    }

    fn parse_operator_identifier_expression(&mut self) -> RExpression {
        match self.current_token.kind {
            T![==] | T![!=] | T![<] | T![<=] | T![>] | T![>=] | T![+] | T![-] | T![/] | T![*] | T![%] => {
                let token = self.expect(self.current_token.kind)?;
                let name = self.lexer.get_source(&token).to_string();
                Ok(Expression {
                    kind: ExpressionKind::Identifier(name),
                    source: token.source,
                })
            }
            _ => Err(ParserErr {
                message: format!("{:?} is not a legal identifier", self.current_token.kind),
                source: self.current_token.source,
            }),
        }
    }

    fn parse_integer_expression(&mut self) -> RExpression {
        let token = self.expect(T![INT])?;
        let value = self.lexer.get_source(&token).to_string();

        Ok(Expression {
            kind: ExpressionKind::Integer(value),
            source: token.source,
        })
    }

    fn parse_decimal_expression(&mut self) -> RExpression {
        let token = self.expect(T![DEC])?;
        let value = self.lexer.get_source(&token).to_string();

        Ok(Expression {
            kind: ExpressionKind::Decimal(value),
            source: token.source,
        })
    }

    fn parse_string_expression(&mut self) -> RExpression {
        let token = self.expect(T![STR])?;

        let string = self.lexer.get_source(&token);
        let unquoted = &string[1..string.len() - 1];

        let mut value = String::with_capacity(unquoted.len());
        let mut characters = unquoted.chars();
        while let Some(character) = characters.next() {
            let unescaped = match character {
                '\\' => match characters.next() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('\\') => '\\',
                    Some('"') => '"',
                    Some(character) => {
                        return Err(ParserErr {
                            message: format!("Unexpected escape sequence: \\{character}"),
                            source: token.source,
                        });
                    }
                    None => {
                        return Err(ParserErr {
                            message: "Unexpected EOF within escape sequence".to_owned(),
                            source: token.source,
                        })
                    }
                },
                character => character,
            };
            value.push(unescaped);
        }

        Ok(Expression {
            kind: ExpressionKind::String(value),
            source: token.source,
        })
    }

    fn parse_booleon_expression(&mut self) -> RExpression {
        let token = self.current_token;
        self.next_token();
        let value = token.kind == T![TRUE];

        Ok(Expression {
            kind: ExpressionKind::Boolean(value),
            source: token.source,
        })
    }

    fn parse_grouped_expression(&mut self) -> RExpression {
        self.expect(T!['('])?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect(T![')'])?;

        Ok(expression)
    }

    fn parse_list_expression(&mut self) -> RExpression {
        let start = self.expect(T!['['])?;

        let elements = self.parse_arguments(T![']'])?;

        Ok(Expression {
            kind: ExpressionKind::List(elements),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_set_expression(&mut self) -> RExpression {
        let start = self.expect(T!['{'])?;

        let elements = self.parse_arguments(T!['}'])?;

        Ok(Expression {
            kind: ExpressionKind::Set(elements),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_hash_expression(&mut self) -> RExpression {
        let start = self.expect(T!["#{"])?;

        let mut elements: Vec<(Expression, Expression)> = Vec::new();

        while self.current_token.kind != T!['}'] {
            if self.current_token.kind == T![ID] && self.next_token.kind != T![:] {
                let name = self.lexer.get_source(&self.current_token).to_string();

                let key = Expression {
                    kind: ExpressionKind::String(name.to_owned()),
                    source: self.current_token.source,
                };
                let value = self.parse_expression(Precedence::Lowest)?;
                elements.push((key, value));
                if !self.consume_if(T![,]) {
                    break;
                }
                continue;
            }

            let key = self.parse_expression(Precedence::Lowest)?;
            self.expect(T![:])?;
            let value = self.parse_expression(Precedence::Lowest)?;
            elements.push((key, value));
            if !self.consume_if(T![,]) {
                break;
            }
        }
        self.expect(T!['}'])?;

        Ok(Expression {
            kind: ExpressionKind::Hash(elements),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_if_expression(&mut self) -> RExpression {
        let start = self.expect(T![IF])?;

        self.consume_if(T!['(']);
        let condition = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.consume_if(T![')']);
        let consequence = Box::new(self.parse_block_statement()?);
        let alternative = if self.consume_if(T![ELSE]) {
            Some(Box::new(self.parse_block_statement()?))
        } else {
            None
        };

        Ok(Expression {
            kind: ExpressionKind::If {
                condition,
                consequence,
                alternative,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_function_expression(&mut self) -> RExpression {
        let start = self.current_token;

        let parameters = if self.consume_if(T![|]) {
            self.parse_parameters(T![|])?
        } else {
            self.expect(T![||])?;
            vec![]
        };
        let body = Box::new(self.parse_block_statement()?);

        Ok(Expression {
            kind: ExpressionKind::Function { parameters, body },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_exclusive_range_expression(&mut self, from: Expression) -> RExpression {
        let start = self.expect(T![..])?;

        match &self.current_token.kind {
            T![ID] | T![INT] | T!['('] | T![-] => {
                let until = Box::new(self.parse_expression(Precedence::Composition)?);
                Ok(Expression {
                    kind: ExpressionKind::ExclusiveRange {
                        from: Box::new(from),
                        until,
                    },
                    source: start.source_range(&self.current_token),
                })
            }
            _ => Ok(Expression {
                kind: ExpressionKind::UnboundedRange { from: Box::new(from) },
                source: start.source_range(&self.current_token),
            }),
        }
    }

    fn parse_inclusive_range_expression(&mut self, from: Expression) -> RExpression {
        let start = self.expect(T![..=])?;

        let to = Box::new(self.parse_expression(Precedence::Composition)?);

        Ok(Expression {
            kind: ExpressionKind::InclusiveRange {
                from: Box::new(from),
                to,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_let_expression(&mut self) -> RExpression {
        let start = self.expect(T![LET])?;

        let is_mutable = self.consume_if(T![MUT]);
        let name = match self.current_token.kind {
            T![ID] => self.parse_identifier_expression()?,
            T!['['] => {
                self.next_token();
                let pattern = self.parse_parameters(T![']'])?;
                Expression {
                    kind: ExpressionKind::IdentifierListPattern(pattern),
                    source: start.source_range(&self.current_token),
                }
            }
            _ => {
                return Err(ParserErr {
                    message: format!("{:?} is not legal within a let identifier", self.current_token.kind),
                    source: self.current_token.source,
                });
            }
        };
        self.expect(T![=])?;
        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        if is_mutable {
            return Ok(Expression {
                kind: ExpressionKind::MutableLet {
                    name: Box::new(name),
                    value,
                },
                source: start.source_range(&self.current_token),
            });
        }

        Ok(Expression {
            kind: ExpressionKind::Let {
                name: Box::new(name),
                value,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_assignment_expression(&mut self, name: Expression) -> RExpression {
        let start = self.expect(T![=])?;

        let value = Box::new(self.parse_expression(Precedence::Equals)?);

        Ok(Expression {
            kind: ExpressionKind::Assign {
                name: Box::new(name),
                value,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_function_composition_expression(&mut self, left: Expression) -> RExpression {
        let start = self.expect(T![>>])?;

        let mut functions = vec![left, self.parse_expression(Precedence::Composition)?];
        while self.consume_if(T![>>]) {
            functions.push(self.parse_expression(Precedence::Composition)?);
        }

        Ok(Expression {
            kind: ExpressionKind::FunctionComposition(functions),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_function_threading_expression(&mut self, initial: Expression) -> RExpression {
        let start = self.expect(T![|>])?;

        let mut functions = vec![self.parse_expression(Precedence::Composition)?];
        while self.consume_if(T![|>]) {
            functions.push(self.parse_expression(Precedence::Composition)?);
        }

        Ok(Expression {
            kind: ExpressionKind::FunctionThread {
                initial: Box::new(initial),
                functions,
            },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_match_expression(&mut self) -> RExpression {
        let start = self.expect(T![MATCH])?;

        let subject = Box::new(self.parse_expression(Precedence::Lowest)?);
        self.expect(T!['{'])?;

        let mut cases = vec![];
        while !self.consume_if(T!['}']) {
            let pattern = Box::new(self.parse_match_pattern()?);
            let guard = if self.consume_if(T![IF]) {
                Some(Box::new(self.parse_expression(Precedence::Lowest)?))
            } else {
                None
            };
            let consequence = Box::new(self.parse_block_statement()?);

            if let Some(guard) = guard {
                cases.push(MatchCase::Guarded {
                    pattern,
                    guard,
                    consequence,
                })
            } else {
                cases.push(MatchCase::Unguarded { pattern, consequence })
            }

            let _ = self.consume_if(T![,]) || self.consume_if(T![CMT]);
        }

        Ok(Expression {
            kind: ExpressionKind::Match { subject, cases },
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_match_pattern(&mut self) -> RExpression {
        match self.current_token.kind {
            T![ID] => self.parse_identifier_expression(),
            T![INT] => self.parse_expression(Precedence::Lowest), // handles ranges as well
            T![DEC] => self.parse_decimal_expression(),
            T![STR] => self.parse_string_expression(),
            T![TRUE] | T![FALSE] => self.parse_booleon_expression(),
            T![_] => Ok(Expression {
                kind: ExpressionKind::Placeholder,
                source: self.expect(T![_])?.source,
            }),
            T!['['] => self.parse_match_list_pattern(),
            T![-] => self.parse_prefix_operator_expression(),
            T![..] => {
                let start = self.expect(T![..])?;
                let name = self.expect(T![ID])?;

                Ok(Expression {
                    kind: ExpressionKind::RestIdentifier(self.lexer.get_source(&name).to_string()),
                    source: start.source_range(&self.current_token),
                })
            }
            _ => Err(ParserErr {
                message: format!("{:?} is not legal in a match pattern", self.current_token.kind),
                source: self.current_token.source,
            }),
        }
    }

    fn parse_match_list_pattern(&mut self) -> RExpression {
        let start = self.expect(T!['['])?;

        let mut pattern = Vec::<Expression>::new();

        if self.consume_if(T![']']) {
            return Ok(Expression {
                kind: ExpressionKind::ListMatchPattern(pattern),
                source: start.source,
            });
        }

        pattern.push(self.parse_match_pattern()?);
        while self.consume_if(T![,]) {
            pattern.push(self.parse_match_pattern()?);
        }
        self.expect(T![']'])?;

        Ok(Expression {
            kind: ExpressionKind::ListMatchPattern(pattern),
            source: start.source_range(&self.current_token),
        })
    }

    fn parse_parameters(&mut self, terminator: TokenKind) -> RExpressions {
        let mut values = Vec::<Expression>::new();

        if self.current_token.kind == terminator {
            return Ok(values);
        }

        let value = match self.current_token.kind {
            T![ID] => self.parse_identifier_expression()?,
            T![_] => {
                let start = self.expect(T![_])?;
                Expression {
                    kind: ExpressionKind::Placeholder,
                    source: start.source_range(&self.current_token),
                }
            }
            T!['['] => {
                let start = self.expect(T!['['])?;
                Expression {
                    kind: ExpressionKind::IdentifierListPattern(self.parse_parameters(T![']'])?),
                    source: start.source_range(&self.current_token),
                }
            }
            T![..] => {
                let start = self.expect(T![..])?;
                let name = self.expect(T![ID])?;
                Expression {
                    kind: ExpressionKind::RestIdentifier(self.lexer.get_source(&name).to_string()),
                    source: start.source_range(&self.current_token),
                }
            }
            _ => self.parse_expression(Precedence::Lowest)?,
        };
        values.push(value);

        while self.consume_if(T![,]) {
            let value = match self.current_token.kind {
                T![ID] => self.parse_identifier_expression()?,
                T![_] => {
                    let start = self.expect(T![_])?;
                    Expression {
                        kind: ExpressionKind::Placeholder,
                        source: start.source_range(&self.current_token),
                    }
                }
                T!['['] => {
                    let start = self.expect(T!['['])?;
                    Expression {
                        kind: ExpressionKind::IdentifierListPattern(self.parse_parameters(T![']'])?),
                        source: start.source_range(&self.current_token),
                    }
                }
                T![..] => {
                    let start = self.expect(T![..])?;
                    let name = self.expect(T![ID])?;
                    Expression {
                        kind: ExpressionKind::RestIdentifier(self.lexer.get_source(&name).to_string()),
                        source: start.source_range(&self.current_token),
                    }
                }
                _ => self.parse_expression(Precedence::Lowest)?,
            };
            values.push(value);
        }

        self.expect(terminator)?;

        Ok(values)
    }

    fn parse_arguments(&mut self, terminator: TokenKind) -> RExpressions {
        let mut values = Vec::<Expression>::new();

        if self.consume_if(terminator) {
            return Ok(values);
        }

        let value = match self.current_token.kind {
            T![..] => {
                let start = self.expect(T![..])?;
                Expression {
                    kind: ExpressionKind::Spread(Box::new(self.parse_expression(Precedence::Lowest)?)),
                    source: start.source_range(&self.current_token),
                }
            }
            _ => self.parse_expression(Precedence::Lowest)?,
        };
        values.push(value);

        while self.consume_if(T![,]) {
            let value = match self.current_token.kind {
                T![..] => {
                    let start = self.expect(T![..])?;
                    Expression {
                        kind: ExpressionKind::Spread(Box::new(self.parse_expression(Precedence::Lowest)?)),
                        source: start.source_range(&self.current_token),
                    }
                }
                _ => self.parse_expression(Precedence::Lowest)?,
            };
            values.push(value);
        }

        self.expect(terminator)?;

        Ok(values)
    }
}

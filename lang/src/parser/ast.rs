use crate::lexer::Location;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub source: Location,
}

pub type Section = Program;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub source: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Return(Box<Expression>),
    Break(Box<Expression>),
    Comment(String),
    Section { name: String, body: Box<Section> },
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Box<Expression>,
    pub guard: Option<Expression>,
    pub consequence: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Bang,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Or,
    And,
    Call(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub source: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Identifier(String),
    RestIdentifier(String),
    Let {
        name: Box<Expression>,
        value: Box<Expression>,
    },
    MutableLet {
        name: Box<Expression>,
        value: Box<Expression>,
    },
    List(Vec<Expression>),
    Set(Vec<Expression>),
    Dictionary(Vec<(Expression, Expression)>),
    InclusiveRange {
        from: Box<Expression>,
        to: Box<Expression>,
    },
    ExclusiveRange {
        from: Box<Expression>,
        until: Box<Expression>,
    },
    UnboundedRange {
        from: Box<Expression>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Match {
        subject: Box<Expression>,
        cases: Vec<MatchCase>,
    },
    Prefix {
        operator: Prefix,
        right: Box<Expression>,
    },
    Infix {
        operator: Infix,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assign {
        name: Box<Expression>,
        value: Box<Expression>,
    },
    FunctionThread {
        initial: Box<Expression>,
        functions: Vec<Expression>,
    },
    FunctionComposition(Vec<Expression>),
    Integer(String),
    Decimal(String),
    String(String),
    Boolean(bool),
    Spread(Box<Expression>),
    IdentifierListPattern(Vec<Expression>),
    ListMatchPattern(Vec<Expression>),
    Placeholder,
    Nil,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: Vec<String> = self.statements.iter().map(|stmt| stmt.to_string()).collect();
        write!(f, "{}", formatted.join("\n"))
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Return(value) => format!("return {};", value),
            Self::Break(value) => format!("break {};", value),
            Self::Comment(value) => format!("//{}", value),
            Self::Section { name, body } => format!("{}: {{{}}}", name, body),
            Self::Expression(expression) => format!("{}", expression),
            Self::Block(statements) => {
                let formatted: Vec<String> = statements.iter().map(|statement| statement.to_string()).collect();
                if statements.len() > 1 {
                    format!("{{{}}}", formatted.join("; "))
                } else {
                    formatted.join("")
                }
            }
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Identifier(name) => name.to_string(),
            Self::RestIdentifier(name) => format!("..{}", name),
            Self::Let { name, value } => format!("let {} = {};", name, value),
            Self::MutableLet { name, value } => format!("let mut {} = {};", name, value),
            Self::List(elements) => {
                let formatted: Vec<String> = elements.iter().map(|element| element.to_string()).collect();
                format!("[{}]", formatted.join(", "))
            }
            Self::Set(elements) => {
                let formatted: Vec<String> = elements.iter().map(|element| element.to_string()).collect();
                format!("{{{}}}", formatted.join(", "))
            }
            Self::Dictionary(elements) => {
                let formatted: Vec<String> = elements
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .collect();
                format!("#{{{}}}", formatted.join(", "))
            }
            Self::InclusiveRange { from, to } => format!("{}..={}", from, to),
            Self::ExclusiveRange { from, until } => format!("{}..{}", from, until),
            Self::UnboundedRange { from } => format!("{}..", from),
            Self::Function { parameters, body } => {
                let formatted: Vec<String> = parameters.iter().map(|parameter| parameter.to_string()).collect();
                format!("|{}| {}", formatted.join(", "), body)
            }
            Self::Index { left, index } => format!("({}[{}])", left, index),
            Self::Call { function, arguments } => {
                let formatted: Vec<String> = arguments.iter().map(|argument| argument.to_string()).collect();
                format!("{}({})", function, formatted.join(", "))
            }
            Self::If {
                condition,
                consequence,
                alternative,
            } => {
                if let Some(alternative) = alternative {
                    format!(
                        "if {} {{\n  {}\n}} else {{\n  {}\n}}",
                        condition, consequence, alternative
                    )
                } else {
                    format!("if {} {{\n  {}\n}}", condition, consequence)
                }
            }
            Self::Match { subject, cases } => {
                let formatted: Vec<String> = cases
                    .iter()
                    .map(|case| {
                        if let Some(guard) = &case.guard {
                            format!("{} if {} {{ {} }}", case.pattern, guard, case.consequence)
                        } else {
                            format!("{} {{ {} }}", case.pattern, case.consequence)
                        }
                    })
                    .collect();
                format!("match {} {{ {} }}", subject, formatted.join(", "))
            }
            Self::Prefix { operator, right } => format!("({}{})", operator, right),
            Self::Infix { left, operator, right } => format!("({} {} {})", left, operator, right),
            Self::Assign { name, value } => format!("({} = {})", name, value),
            Self::FunctionThread { initial, functions } => {
                let formatted: Vec<String> = functions.iter().map(|function| function.to_string()).collect();
                format!("({} |> {})", initial, formatted.join(" |> "))
            }
            Self::FunctionComposition(functions) => {
                let formatted: Vec<String> = functions.iter().map(|function| function.to_string()).collect();
                format!("({})", formatted.join(" >> "))
            }
            Self::Integer(value) => value.to_string(),
            Self::Decimal(value) => value.to_string(),
            Self::String(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Spread(value) => format!("..{}", value),
            Self::IdentifierListPattern(pattern) => {
                let formatted: Vec<String> = pattern.iter().map(|element| element.to_string()).collect();
                format!("[{}]", formatted.join(", "))
            }
            Self::ListMatchPattern(pattern) => {
                let formatted: Vec<String> = pattern.iter().map(|element| element.to_string()).collect();
                format!("[{}]", formatted.join(", "))
            }
            Self::Placeholder => "_".to_owned(),
            Self::Nil => "nil".to_owned(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Minus => "-",
                Prefix::Bang => "!",
            }
        )
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Infix::Plus => "+".to_owned(),
            Infix::Minus => "-".to_owned(),
            Infix::Asterisk => "*".to_owned(),
            Infix::Slash => "/".to_owned(),
            Infix::Modulo => "%".to_owned(),
            Infix::Equal => "==".to_owned(),
            Infix::NotEqual => "!=".to_owned(),
            Infix::LessThan => "<".to_owned(),
            Infix::LessThanEqual => "<=".to_owned(),
            Infix::GreaterThan => ">".to_owned(),
            Infix::GreaterThanEqual => ">=".to_owned(),
            Infix::And => "&&".to_owned(),
            Infix::Or => "||".to_owned(),
            Infix::Call(identifier) => format!("`{}`", identifier),
        };
        write!(f, "{}", s)
    }
}

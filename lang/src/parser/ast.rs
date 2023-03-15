use crate::lexer::Location;

#[derive(Debug)]
pub struct Program {
    pub source: Location,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return {
        source: Location,
        value: Box<Expression>,
    },
    Break {
        source: Location,
        value: Box<Expression>,
    },
    Comment {
        source: Location,
        value: String,
    },
    Section {
        source: Location,
        name: Box<Expression>,
        body: Box<Statement>,
    },
    Expression {
        source: Location,
        expression: Box<Expression>,
    },
    Block {
        source: Location,
        statements: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCase {
    Unguarded {
        pattern: Box<Expression>,
        consequence: Box<Statement>,
    },
    Guarded {
        pattern: Box<Expression>,
        guard: Box<Expression>,
        consequence: Box<Statement>,
    },
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
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Or,
    Modulo,
    And,
    Equal,
    NotEqual,
    Call(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier {
        source: Location,
        name: String,
    },
    Let {
        source: Location,
        name: Box<Expression>,
        value: Box<Expression>,
    },
    MutableLet {
        source: Location,
        name: Box<Expression>,
        value: Box<Expression>,
    },
    List {
        source: Location,
        elements: Vec<Expression>,
    },
    Set {
        source: Location,
        elements: Vec<Expression>,
    },
    Hash {
        source: Location,
        elements: Vec<(Expression, Expression)>,
    },
    InclusiveRange {
        source: Location,
        from: Box<Expression>,
        to: Box<Expression>,
    },
    ExclusiveRange {
        source: Location,
        from: Box<Expression>,
        until: Box<Expression>,
    },
    UnboundedRange {
        source: Location,
        from: Box<Expression>,
    },
    Function {
        source: Location,
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Index {
        source: Location,
        left: Box<Expression>,
        index: Box<Expression>,
    },
    Call {
        source: Location,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    If {
        source: Location,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Match {
        source: Location,
        subject: Box<Expression>,
        cases: Vec<MatchCase>,
    },
    Prefix {
        source: Location,
        operator: Prefix,
        right: Box<Expression>,
    },
    Infix {
        source: Location,
        operator: Infix,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assign {
        source: Location,
        name: Box<Expression>,
        value: Box<Expression>,
    },
    FunctionThread {
        source: Location,
        initial: Box<Expression>,
        functions: Vec<Expression>,
    },
    FunctionComposition {
        source: Location,
        functions: Vec<Expression>,
    },
    Integer {
        source: Location,
        value: String,
    },
    Decimal {
        source: Location,
        value: String,
    },
    String {
        source: Location,
        value: String,
    },
    Boolean {
        source: Location,
        value: bool,
    },
    RestElement {
        source: Location,
        name: Box<Expression>,
    },
    SpreadElement {
        source: Location,
        value: Box<Expression>,
    },
    IdentifierListPattern {
        source: Location,
        pattern: Vec<Expression>,
    },
    ListMatchPattern {
        source: Location,
        pattern: Vec<Expression>,
    },
    Placeholder {
        source: Location,
    },
    Nil {
        source: Location,
    },
}

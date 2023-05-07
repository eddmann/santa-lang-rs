mod evaluator;
mod lexer;
mod parser;
mod runner;

pub use crate::evaluator::environment::Environment;
pub use crate::evaluator::function::{Arguments, ExternalFnDef};
pub use crate::evaluator::object::Object;
pub use crate::evaluator::{Evaluation, RuntimeErr};
pub use crate::lexer::Location;
pub use crate::parser::ast::ExpressionKind;
pub use crate::runner::{run, AoCRunner, RunErr, RunEvaluation, Time};

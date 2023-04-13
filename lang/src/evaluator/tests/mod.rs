macro_rules! test_eval {
    (suite $suite: ident; $( ($source: tt, $expected: tt, $case: ident)),*) => {
        mod $suite {
            $(
                #[test]
                fn $case() {
                    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new($source));
                    let program = parser.parse().unwrap();
                    let mut evaluator = crate::evaluator::Evaluator::new();
                    let actual = match evaluator.evaluate(&program) {
                        Ok(value) => value.to_string(),
                        Err(error) => error.message,
                    };
                    assert_eq!($expected, actual);
                }
            )*
        }
    };
    (suite $suite: ident; sut $sut: tt; $( ($source: tt, $expected: tt, $case: ident)),*) => {
        mod $suite {
            $(
                #[test]
                fn $case() {
                    let source = format!("{}{}", $sut, $source);
                    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new(&source));
                    let program = parser.parse().unwrap();
                    let mut evaluator = crate::evaluator::Evaluator::new();
                    let actual = match evaluator.evaluate(&program) {
                        Ok(value) => value.to_string(),
                        Err(error) => error.message,
                    };
                    assert_eq!($expected, actual);
                }
            )*
        }
    }
}

mod assignment;
mod errors;
mod functions;
mod indexing;
mod literals;
mod matches;
mod operators;
mod sections;

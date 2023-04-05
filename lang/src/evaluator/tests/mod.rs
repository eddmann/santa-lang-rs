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

    }
}

mod assignment;
mod literals;

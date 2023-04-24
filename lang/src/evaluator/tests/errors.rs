use expect_test::expect;

#[test]
fn error() {
    let source = r#"
        1 + "1"
    "#;

    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new(source));
    let program = parser.parse().unwrap();
    let mut evaluator = crate::evaluator::Evaluator::new();
    let actual = format!("{:?}", evaluator.evaluate(&program));

    expect![[r#"Err(RuntimeErr { message: "Unsupported operation: Integer + String", source: 11..21, trace: [] })"#]]
        .assert_eq(&actual);
}

#[test]
fn trace() {
    let source = r#"
        let a = || unknown
        let b = || a()
        let c = || b()
        c()
    "#;

    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new(source));
    let program = parser.parse().unwrap();
    let mut evaluator = crate::evaluator::Evaluator::new();
    let actual = format!("{:?}", evaluator.evaluate(&program));

    expect![[r#"Err(RuntimeErr { message: "Identifier can not be found: unknown", source: 20..27, trace: [47..48, 70..71, 82..83] })"#]].assert_eq(&actual);
}

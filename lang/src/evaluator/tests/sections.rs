use expect_test::{expect, Expect};

#[test]
fn block_section() {
    assert_section(
        "my_section: { \"sample\" }",
        "my_section",
        expect![[r#"
            [
                Program {
                    statements: [
                        Statement {
                            kind: Expression(
                                Expression {
                                    kind: String(
                                        "sample",
                                    ),
                                    source: 14..22,
                                },
                            ),
                            source: 14..23,
                        },
                    ],
                    source: 0..24,
                },
            ]"#]],
    );
}

#[test]
fn expression_section() {
    assert_section(
        "my_section: \"sample\"",
        "my_section",
        expect![[r#"
            [
                Program {
                    statements: [
                        Statement {
                            kind: Expression(
                                Expression {
                                    kind: String(
                                        "sample",
                                    ),
                                    source: 12..20,
                                },
                            ),
                            source: 12..20,
                        },
                    ],
                    source: 0..20,
                },
            ]"#]],
    );
}

#[test]
fn nested_section() {
    assert_section(
        r#"
            section_one: {
                section_two: "sample";
            };
        "#,
        "section_one",
        expect![[r#"
            [
                Program {
                    statements: [
                        Statement {
                            kind: Section {
                                name: "section_two",
                                body: Program {
                                    statements: [
                                        Statement {
                                            kind: Expression(
                                                Expression {
                                                    kind: String(
                                                        "sample",
                                                    ),
                                                    source: 57..65,
                                                },
                                            ),
                                            source: 57..79,
                                        },
                                    ],
                                    source: 44..79,
                                },
                            },
                            source: 44..79,
                        },
                    ],
                    source: 13..80,
                },
            ]"#]],
    );
}

#[test]
fn multiple_sections_with_same_name() {
    assert_section(
        r#"
            my_section: { 1 };
            my_section: { 2 };
        "#,
        "my_section",
        expect![[r#"
            [
                Program {
                    statements: [
                        Statement {
                            kind: Expression(
                                Expression {
                                    kind: Integer(
                                        "1",
                                    ),
                                    source: 27..28,
                                },
                            ),
                            source: 27..29,
                        },
                    ],
                    source: 13..30,
                },
                Program {
                    statements: [
                        Statement {
                            kind: Expression(
                                Expression {
                                    kind: Integer(
                                        "2",
                                    ),
                                    source: 58..59,
                                },
                            ),
                            source: 58..60,
                        },
                    ],
                    source: 44..61,
                },
            ]"#]],
    );
}

fn assert_section(source: &str, name: &str, expected: Expect) {
    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new(source));
    let program = parser.parse();
    let mut evaluator = crate::evaluator::Evaluator::new();
    let environment = crate::evaluator::Environment::new();

    evaluator
        .evaluate_with_environment(&program.expect("Ok"), std::rc::Rc::clone(&environment))
        .expect("Ok");

    let actual = format!("{:#?}", environment.borrow().get_sections(name));
    expected.assert_eq(&actual);
}

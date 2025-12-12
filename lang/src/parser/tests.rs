use super::*;

use expect_test::{Expect, expect};

#[test]
fn integers() {
    assert_ast(
        "
            1;
            1_000_000;
        ",
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Integer(
                                    "1",
                                ),
                                source: 0..1,
                            },
                        ),
                        source: 0..15,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Integer(
                                    "1_000_000",
                                ),
                                source: 15..24,
                            },
                        ),
                        source: 15..25,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..25,
            }"#]],
    );
}

#[test]
fn decimals() {
    assert_ast(
        "
            1.5;
            1_000_000.50;
        ",
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Decimal(
                                    "1.5",
                                ),
                                source: 0..3,
                            },
                        ),
                        source: 0..17,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Decimal(
                                    "1_000_000.50",
                                ),
                                source: 17..29,
                            },
                        ),
                        source: 17..30,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..30,
            }"#]],
    );
}

#[test]
fn strings() {
    assert_ast(
        r#"
            "Hello, world!"
            "\n\t\""
            "µࠒ𒀀"
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: String(
                                    "Hello, world!",
                                ),
                                source: 0..15,
                            },
                        ),
                        source: 0..28,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: String(
                                    "\n\t\"",
                                ),
                                source: 28..36,
                            },
                        ),
                        source: 28..49,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: String(
                                    "µࠒ𒀀",
                                ),
                                source: 49..60,
                            },
                        ),
                        source: 49..60,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..60,
            }"#]],
    );
}

#[test]
fn booleans() {
    assert_ast(
        "
            true;
            false;
        ",
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Boolean(
                                    true,
                                ),
                                source: 0..4,
                            },
                        ),
                        source: 0..18,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Boolean(
                                    false,
                                ),
                                source: 18..23,
                            },
                        ),
                        source: 18..24,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..24,
            }"#]],
    );
}

#[test]
fn nil() {
    assert_ast(
        "nil;",
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Nil,
                                source: 0..3,
                            },
                        ),
                        source: 0..4,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..4,
            }"#]],
    );
}

#[test]
fn comments() {
    assert_ast(
        "
            // full line comment
            1; // end of line comment
        ",
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Comment(
                            "// full line comment",
                        ),
                        source: 0..20,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Integer(
                                    "1",
                                ),
                                source: 33..34,
                            },
                        ),
                        source: 33..58,
                        preceded_by_blank_line: false,
                        trailing_comment: Some(
                            "// end of line comment",
                        ),
                    },
                ],
                source: 0..58,
            }"#]],
    );
}

#[test]
fn list() {
    assert_ast(
        r#"
            [1, 2.5, "Hello, world!", ..xs, ..[true]]
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: List(
                                    [
                                        Expression {
                                            kind: Integer(
                                                "1",
                                            ),
                                            source: 1..2,
                                        },
                                        Expression {
                                            kind: Decimal(
                                                "2.5",
                                            ),
                                            source: 4..7,
                                        },
                                        Expression {
                                            kind: String(
                                                "Hello, world!",
                                            ),
                                            source: 9..24,
                                        },
                                        Expression {
                                            kind: Spread(
                                                Expression {
                                                    kind: Identifier(
                                                        "xs",
                                                    ),
                                                    source: 28..30,
                                                },
                                            ),
                                            source: 26..30,
                                        },
                                        Expression {
                                            kind: Spread(
                                                Expression {
                                                    kind: List(
                                                        [
                                                            Expression {
                                                                kind: Boolean(
                                                                    true,
                                                                ),
                                                                source: 35..39,
                                                            },
                                                        ],
                                                    ),
                                                    source: 34..40,
                                                },
                                            ),
                                            source: 32..40,
                                        },
                                    ],
                                ),
                                source: 0..41,
                            },
                        ),
                        source: 0..41,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..41,
            }"#]],
    );
}

#[test]
fn dictionary() {
    assert_ast(
        r#"
            #{"Hello, world!": #{x}, 1: "2", [1, 2]: 1.4}
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Dictionary(
                                    [
                                        (
                                            Expression {
                                                kind: String(
                                                    "Hello, world!",
                                                ),
                                                source: 2..17,
                                            },
                                            Expression {
                                                kind: Dictionary(
                                                    [
                                                        (
                                                            Expression {
                                                                kind: String(
                                                                    "x",
                                                                ),
                                                                source: 21..22,
                                                            },
                                                            Expression {
                                                                kind: Identifier(
                                                                    "x",
                                                                ),
                                                                source: 21..22,
                                                            },
                                                        ),
                                                    ],
                                                ),
                                                source: 19..23,
                                            },
                                        ),
                                        (
                                            Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 25..26,
                                            },
                                            Expression {
                                                kind: String(
                                                    "2",
                                                ),
                                                source: 28..31,
                                            },
                                        ),
                                        (
                                            Expression {
                                                kind: List(
                                                    [
                                                        Expression {
                                                            kind: Integer(
                                                                "1",
                                                            ),
                                                            source: 34..35,
                                                        },
                                                        Expression {
                                                            kind: Integer(
                                                                "2",
                                                            ),
                                                            source: 37..38,
                                                        },
                                                    ],
                                                ),
                                                source: 33..39,
                                            },
                                            Expression {
                                                kind: Decimal(
                                                    "1.4",
                                                ),
                                                source: 41..44,
                                            },
                                        ),
                                    ],
                                ),
                                source: 0..45,
                            },
                        ),
                        source: 0..45,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..45,
            }"#]],
    );
}

#[test]
fn set() {
    assert_ast(
        r#"
            {1, 2.5, "Hello, world!", ..[true]}
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Set(
                                    [
                                        Expression {
                                            kind: Integer(
                                                "1",
                                            ),
                                            source: 1..2,
                                        },
                                        Expression {
                                            kind: Decimal(
                                                "2.5",
                                            ),
                                            source: 4..7,
                                        },
                                        Expression {
                                            kind: String(
                                                "Hello, world!",
                                            ),
                                            source: 9..24,
                                        },
                                        Expression {
                                            kind: Spread(
                                                Expression {
                                                    kind: List(
                                                        [
                                                            Expression {
                                                                kind: Boolean(
                                                                    true,
                                                                ),
                                                                source: 29..33,
                                                            },
                                                        ],
                                                    ),
                                                    source: 28..34,
                                                },
                                            ),
                                            source: 26..34,
                                        },
                                    ],
                                ),
                                source: 0..35,
                            },
                        ),
                        source: 0..35,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..35,
            }"#]],
    );
}

#[test]
fn ranges() {
    assert_ast(
        r#"
            1..10; x..y; -1..1; 1..-1; (1)..(1);
            1..=10; x..=y; -1..=1; 1..=-1; (1)..=(1);
            1..; x..; -1..; (1)..;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: ExclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 0..1,
                                    },
                                    until: Expression {
                                        kind: Integer(
                                            "10",
                                        ),
                                        source: 3..5,
                                    },
                                },
                                source: 1..5,
                            },
                        ),
                        source: 0..7,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: ExclusiveRange {
                                    from: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 7..8,
                                    },
                                    until: Expression {
                                        kind: Identifier(
                                            "y",
                                        ),
                                        source: 10..11,
                                    },
                                },
                                source: 8..11,
                            },
                        ),
                        source: 7..13,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: ExclusiveRange {
                                    from: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 14..15,
                                            },
                                        },
                                        source: 13..15,
                                    },
                                    until: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 17..18,
                                    },
                                },
                                source: 15..18,
                            },
                        ),
                        source: 13..20,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: ExclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 20..21,
                                    },
                                    until: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 24..25,
                                            },
                                        },
                                        source: 23..25,
                                    },
                                },
                                source: 21..25,
                            },
                        ),
                        source: 20..27,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: ExclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 28..29,
                                    },
                                    until: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 33..34,
                                    },
                                },
                                source: 30..35,
                            },
                        ),
                        source: 27..49,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: InclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 49..50,
                                    },
                                    to: Expression {
                                        kind: Integer(
                                            "10",
                                        ),
                                        source: 53..55,
                                    },
                                },
                                source: 50..55,
                            },
                        ),
                        source: 49..57,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: InclusiveRange {
                                    from: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 57..58,
                                    },
                                    to: Expression {
                                        kind: Identifier(
                                            "y",
                                        ),
                                        source: 61..62,
                                    },
                                },
                                source: 58..62,
                            },
                        ),
                        source: 57..64,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: InclusiveRange {
                                    from: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 65..66,
                                            },
                                        },
                                        source: 64..66,
                                    },
                                    to: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 69..70,
                                    },
                                },
                                source: 66..70,
                            },
                        ),
                        source: 64..72,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: InclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 72..73,
                                    },
                                    to: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 77..78,
                                            },
                                        },
                                        source: 76..78,
                                    },
                                },
                                source: 73..78,
                            },
                        ),
                        source: 72..80,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: InclusiveRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 81..82,
                                    },
                                    to: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 87..88,
                                    },
                                },
                                source: 83..89,
                            },
                        ),
                        source: 80..103,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: UnboundedRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 103..104,
                                    },
                                },
                                source: 104..106,
                            },
                        ),
                        source: 103..108,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: UnboundedRange {
                                    from: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 108..109,
                                    },
                                },
                                source: 109..111,
                            },
                        ),
                        source: 108..113,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: UnboundedRange {
                                    from: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 114..115,
                                            },
                                        },
                                        source: 113..115,
                                    },
                                },
                                source: 115..117,
                            },
                        ),
                        source: 113..119,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: UnboundedRange {
                                    from: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 120..121,
                                    },
                                },
                                source: 122..124,
                            },
                        ),
                        source: 119..125,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..125,
            }"#]],
    );
}

#[test]
fn let_assignments() {
    assert_ast(
        r#"
            let x = 1;
            let mut y = 1;
            y = 2;
            let [a, b, [c, d], ..e] = [1, 2, [3, 4], 5];
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Let {
                                    name: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 4..5,
                                    },
                                    value: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 8..9,
                                    },
                                },
                                source: 0..9,
                            },
                        ),
                        source: 0..23,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: MutableLet {
                                    name: Expression {
                                        kind: Identifier(
                                            "y",
                                        ),
                                        source: 31..32,
                                    },
                                    value: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 35..36,
                                    },
                                },
                                source: 23..36,
                            },
                        ),
                        source: 23..50,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Assign {
                                    name: Expression {
                                        kind: Identifier(
                                            "y",
                                        ),
                                        source: 50..51,
                                    },
                                    value: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 54..55,
                                    },
                                },
                                source: 52..55,
                            },
                        ),
                        source: 50..69,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Let {
                                    name: Expression {
                                        kind: IdentifierListPattern(
                                            [
                                                Expression {
                                                    kind: Identifier(
                                                        "a",
                                                    ),
                                                    source: 74..75,
                                                },
                                                Expression {
                                                    kind: Identifier(
                                                        "b",
                                                    ),
                                                    source: 77..78,
                                                },
                                                Expression {
                                                    kind: IdentifierListPattern(
                                                        [
                                                            Expression {
                                                                kind: Identifier(
                                                                    "c",
                                                                ),
                                                                source: 81..82,
                                                            },
                                                            Expression {
                                                                kind: Identifier(
                                                                    "d",
                                                                ),
                                                                source: 84..85,
                                                            },
                                                        ],
                                                    ),
                                                    source: 80..86,
                                                },
                                                Expression {
                                                    kind: RestIdentifier(
                                                        "e",
                                                    ),
                                                    source: 88..91,
                                                },
                                            ],
                                        ),
                                        source: 69..93,
                                    },
                                    value: Expression {
                                        kind: List(
                                            [
                                                Expression {
                                                    kind: Integer(
                                                        "1",
                                                    ),
                                                    source: 96..97,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "2",
                                                    ),
                                                    source: 99..100,
                                                },
                                                Expression {
                                                    kind: List(
                                                        [
                                                            Expression {
                                                                kind: Integer(
                                                                    "3",
                                                                ),
                                                                source: 103..104,
                                                            },
                                                            Expression {
                                                                kind: Integer(
                                                                    "4",
                                                                ),
                                                                source: 106..107,
                                                            },
                                                        ],
                                                    ),
                                                    source: 102..108,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "5",
                                                    ),
                                                    source: 110..111,
                                                },
                                            ],
                                        ),
                                        source: 95..112,
                                    },
                                },
                                source: 69..112,
                            },
                        ),
                        source: 69..113,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..113,
            }"#]],
    );
}

#[test]
fn if_expressions() {
    assert_ast(
        r#"
            if 2 < 5 { 1 }
            if 3 > 5 { 1 } else { 2 }
            if let x = 1 { x }
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: If {
                                    condition: Expression {
                                        kind: Infix {
                                            operator: LessThan,
                                            left: Expression {
                                                kind: Integer(
                                                    "2",
                                                ),
                                                source: 3..4,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 7..8,
                                            },
                                        },
                                        source: 5..9,
                                    },
                                    consequence: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Integer(
                                                                "1",
                                                            ),
                                                            source: 11..12,
                                                        },
                                                    ),
                                                    source: 11..13,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 9..27,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                    alternative: None,
                                },
                                source: 0..27,
                            },
                        ),
                        source: 0..27,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: If {
                                    condition: Expression {
                                        kind: Infix {
                                            operator: GreaterThan,
                                            left: Expression {
                                                kind: Integer(
                                                    "3",
                                                ),
                                                source: 30..31,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 34..35,
                                            },
                                        },
                                        source: 32..36,
                                    },
                                    consequence: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Integer(
                                                                "1",
                                                            ),
                                                            source: 38..39,
                                                        },
                                                    ),
                                                    source: 38..40,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 36..42,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                    alternative: Some(
                                        Statement {
                                            kind: Block(
                                                [
                                                    Statement {
                                                        kind: Expression(
                                                            Expression {
                                                                kind: Integer(
                                                                    "2",
                                                                ),
                                                                source: 49..50,
                                                            },
                                                        ),
                                                        source: 49..51,
                                                        preceded_by_blank_line: false,
                                                        trailing_comment: None,
                                                    },
                                                ],
                                            ),
                                            source: 47..65,
                                            preceded_by_blank_line: false,
                                            trailing_comment: None,
                                        },
                                    ),
                                },
                                source: 27..65,
                            },
                        ),
                        source: 27..65,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: If {
                                    condition: Expression {
                                        kind: Let {
                                            name: Expression {
                                                kind: Identifier(
                                                    "x",
                                                ),
                                                source: 72..73,
                                            },
                                            value: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 76..77,
                                            },
                                        },
                                        source: 68..78,
                                    },
                                    consequence: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Identifier(
                                                                "x",
                                                            ),
                                                            source: 80..81,
                                                        },
                                                    ),
                                                    source: 80..82,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 78..83,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                    alternative: None,
                                },
                                source: 65..83,
                            },
                        ),
                        source: 65..83,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..83,
            }"#]],
    );
}

#[test]
fn prefix_operators() {
    assert_ast(
        r#"
            -1;
            --1;
            4 - -4;
            !true;
            !!true;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Prefix {
                                    operator: Minus,
                                    right: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 1..2,
                                    },
                                },
                                source: 0..2,
                            },
                        ),
                        source: 0..16,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Prefix {
                                    operator: Minus,
                                    right: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 18..19,
                                            },
                                        },
                                        source: 17..19,
                                    },
                                },
                                source: 16..19,
                            },
                        ),
                        source: 16..33,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Minus,
                                    left: Expression {
                                        kind: Integer(
                                            "4",
                                        ),
                                        source: 33..34,
                                    },
                                    right: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "4",
                                                ),
                                                source: 38..39,
                                            },
                                        },
                                        source: 37..39,
                                    },
                                },
                                source: 35..39,
                            },
                        ),
                        source: 33..53,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Prefix {
                                    operator: Bang,
                                    right: Expression {
                                        kind: Boolean(
                                            true,
                                        ),
                                        source: 54..58,
                                    },
                                },
                                source: 53..58,
                            },
                        ),
                        source: 53..72,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Prefix {
                                    operator: Bang,
                                    right: Expression {
                                        kind: Prefix {
                                            operator: Bang,
                                            right: Expression {
                                                kind: Boolean(
                                                    true,
                                                ),
                                                source: 74..78,
                                            },
                                        },
                                        source: 73..78,
                                    },
                                },
                                source: 72..78,
                            },
                        ),
                        source: 72..79,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..79,
            }"#]],
    );
}

#[test]
fn infix_operators() {
    assert_ast(
        r#"
            1 + 1;
            1 - 2;
            1 / 2;
            3 % 4;
            4 == 5 || 4 != 7;
            5 > 10 && 4 < 8;
            5 >= 3 && 4 <= 2;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Plus,
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 0..1,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 4..5,
                                    },
                                },
                                source: 2..5,
                            },
                        ),
                        source: 0..19,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Minus,
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 19..20,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 23..24,
                                    },
                                },
                                source: 21..24,
                            },
                        ),
                        source: 19..38,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Slash,
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 38..39,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 42..43,
                                    },
                                },
                                source: 40..43,
                            },
                        ),
                        source: 38..57,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Modulo,
                                    left: Expression {
                                        kind: Integer(
                                            "3",
                                        ),
                                        source: 57..58,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "4",
                                        ),
                                        source: 61..62,
                                    },
                                },
                                source: 59..62,
                            },
                        ),
                        source: 57..76,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Or,
                                    left: Expression {
                                        kind: Infix {
                                            operator: Equal,
                                            left: Expression {
                                                kind: Integer(
                                                    "4",
                                                ),
                                                source: 76..77,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 81..82,
                                            },
                                        },
                                        source: 78..83,
                                    },
                                    right: Expression {
                                        kind: Infix {
                                            operator: NotEqual,
                                            left: Expression {
                                                kind: Integer(
                                                    "4",
                                                ),
                                                source: 86..87,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "7",
                                                ),
                                                source: 91..92,
                                            },
                                        },
                                        source: 88..92,
                                    },
                                },
                                source: 83..92,
                            },
                        ),
                        source: 76..106,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: And,
                                    left: Expression {
                                        kind: Infix {
                                            operator: GreaterThan,
                                            left: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 106..107,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "10",
                                                ),
                                                source: 110..112,
                                            },
                                        },
                                        source: 108..113,
                                    },
                                    right: Expression {
                                        kind: Infix {
                                            operator: LessThan,
                                            left: Expression {
                                                kind: Integer(
                                                    "4",
                                                ),
                                                source: 116..117,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "8",
                                                ),
                                                source: 120..121,
                                            },
                                        },
                                        source: 118..121,
                                    },
                                },
                                source: 113..121,
                            },
                        ),
                        source: 106..135,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: And,
                                    left: Expression {
                                        kind: Infix {
                                            operator: GreaterThanEqual,
                                            left: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 135..136,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "3",
                                                ),
                                                source: 140..141,
                                            },
                                        },
                                        source: 137..142,
                                    },
                                    right: Expression {
                                        kind: Infix {
                                            operator: LessThanEqual,
                                            left: Expression {
                                                kind: Integer(
                                                    "4",
                                                ),
                                                source: 145..146,
                                            },
                                            right: Expression {
                                                kind: Integer(
                                                    "2",
                                                ),
                                                source: 150..151,
                                            },
                                        },
                                        source: 147..151,
                                    },
                                },
                                source: 142..151,
                            },
                        ),
                        source: 135..152,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..152,
            }"#]],
    );
}

#[test]
fn function_literals() {
    assert_ast(
        r#"
            |x, y| { x + y; };
            |x, y| x + y;
            |x, [y, ..ys], ..z| x
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Function {
                                    parameters: [
                                        Expression {
                                            kind: Identifier(
                                                "x",
                                            ),
                                            source: 1..2,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "y",
                                            ),
                                            source: 4..5,
                                        },
                                    ],
                                    body: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Infix {
                                                                operator: Plus,
                                                                left: Expression {
                                                                    kind: Identifier(
                                                                        "x",
                                                                    ),
                                                                    source: 9..10,
                                                                },
                                                                right: Expression {
                                                                    kind: Identifier(
                                                                        "y",
                                                                    ),
                                                                    source: 13..14,
                                                                },
                                                            },
                                                            source: 11..14,
                                                        },
                                                    ),
                                                    source: 9..16,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 7..17,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                },
                                source: 0..17,
                            },
                        ),
                        source: 0..31,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Function {
                                    parameters: [
                                        Expression {
                                            kind: Identifier(
                                                "x",
                                            ),
                                            source: 32..33,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "y",
                                            ),
                                            source: 35..36,
                                        },
                                    ],
                                    body: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Infix {
                                                                operator: Plus,
                                                                left: Expression {
                                                                    kind: Identifier(
                                                                        "x",
                                                                    ),
                                                                    source: 38..39,
                                                                },
                                                                right: Expression {
                                                                    kind: Identifier(
                                                                        "y",
                                                                    ),
                                                                    source: 42..43,
                                                                },
                                                            },
                                                            source: 40..43,
                                                        },
                                                    ),
                                                    source: 38..57,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 38..57,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                },
                                source: 31..57,
                            },
                        ),
                        source: 31..57,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Function {
                                    parameters: [
                                        Expression {
                                            kind: Identifier(
                                                "x",
                                            ),
                                            source: 58..59,
                                        },
                                        Expression {
                                            kind: IdentifierListPattern(
                                                [
                                                    Expression {
                                                        kind: Identifier(
                                                            "y",
                                                        ),
                                                        source: 62..63,
                                                    },
                                                    Expression {
                                                        kind: RestIdentifier(
                                                            "ys",
                                                        ),
                                                        source: 65..69,
                                                    },
                                                ],
                                            ),
                                            source: 61..70,
                                        },
                                        Expression {
                                            kind: RestIdentifier(
                                                "z",
                                            ),
                                            source: 72..75,
                                        },
                                    ],
                                    body: Statement {
                                        kind: Block(
                                            [
                                                Statement {
                                                    kind: Expression(
                                                        Expression {
                                                            kind: Identifier(
                                                                "x",
                                                            ),
                                                            source: 77..78,
                                                        },
                                                    ),
                                                    source: 77..78,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            ],
                                        ),
                                        source: 77..78,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                },
                                source: 57..78,
                            },
                        ),
                        source: 57..78,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..78,
            }"#]],
    );
}

#[test]
fn call_expressions() {
    assert_ast(
        r#"
            add(1, 2);
            add(x + y, 3);
            add(..xs);
            1 `add` 2;
            x `add` y;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "add",
                                        ),
                                        source: 0..3,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Integer(
                                                "1",
                                            ),
                                            source: 4..5,
                                        },
                                        Expression {
                                            kind: Integer(
                                                "2",
                                            ),
                                            source: 7..8,
                                        },
                                    ],
                                },
                                source: 3..9,
                            },
                        ),
                        source: 0..23,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "add",
                                        ),
                                        source: 23..26,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Infix {
                                                operator: Plus,
                                                left: Expression {
                                                    kind: Identifier(
                                                        "x",
                                                    ),
                                                    source: 27..28,
                                                },
                                                right: Expression {
                                                    kind: Identifier(
                                                        "y",
                                                    ),
                                                    source: 31..32,
                                                },
                                            },
                                            source: 29..32,
                                        },
                                        Expression {
                                            kind: Integer(
                                                "3",
                                            ),
                                            source: 34..35,
                                        },
                                    ],
                                },
                                source: 26..36,
                            },
                        ),
                        source: 23..50,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "add",
                                        ),
                                        source: 50..53,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Spread(
                                                Expression {
                                                    kind: Identifier(
                                                        "xs",
                                                    ),
                                                    source: 56..58,
                                                },
                                            ),
                                            source: 54..58,
                                        },
                                    ],
                                },
                                source: 53..59,
                            },
                        ),
                        source: 50..73,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Call(
                                        Expression {
                                            kind: Identifier(
                                                "add",
                                            ),
                                            source: 75..80,
                                        },
                                    ),
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 73..74,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 81..82,
                                    },
                                },
                                source: 75..82,
                            },
                        ),
                        source: 73..96,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Call(
                                        Expression {
                                            kind: Identifier(
                                                "add",
                                            ),
                                            source: 98..103,
                                        },
                                    ),
                                    left: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 96..97,
                                    },
                                    right: Expression {
                                        kind: Identifier(
                                            "y",
                                        ),
                                        source: 104..105,
                                    },
                                },
                                source: 98..105,
                            },
                        ),
                        source: 96..106,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..106,
            }"#]],
    );
}

#[test]
fn partial_application_using_placeholders() {
    assert_ast(
        r#"
            _ + 2;
            1 + _;
            +(_, 2);
            +(1, _)
            _ `add` 2;
            1 `add` _;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Plus,
                                    left: Expression {
                                        kind: Placeholder,
                                        source: 0..1,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 4..5,
                                    },
                                },
                                source: 2..5,
                            },
                        ),
                        source: 0..19,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Plus,
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 19..20,
                                    },
                                    right: Expression {
                                        kind: Placeholder,
                                        source: 23..24,
                                    },
                                },
                                source: 21..24,
                            },
                        ),
                        source: 19..38,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "+",
                                        ),
                                        source: 38..39,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Placeholder,
                                            source: 40..41,
                                        },
                                        Expression {
                                            kind: Integer(
                                                "2",
                                            ),
                                            source: 43..44,
                                        },
                                    ],
                                },
                                source: 39..45,
                            },
                        ),
                        source: 38..59,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "+",
                                        ),
                                        source: 59..60,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Integer(
                                                "1",
                                            ),
                                            source: 61..62,
                                        },
                                        Expression {
                                            kind: Placeholder,
                                            source: 64..65,
                                        },
                                    ],
                                },
                                source: 60..79,
                            },
                        ),
                        source: 59..79,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Call(
                                        Expression {
                                            kind: Identifier(
                                                "add",
                                            ),
                                            source: 81..86,
                                        },
                                    ),
                                    left: Expression {
                                        kind: Placeholder,
                                        source: 79..80,
                                    },
                                    right: Expression {
                                        kind: Integer(
                                            "2",
                                        ),
                                        source: 87..88,
                                    },
                                },
                                source: 81..88,
                            },
                        ),
                        source: 79..102,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Infix {
                                    operator: Call(
                                        Expression {
                                            kind: Identifier(
                                                "add",
                                            ),
                                            source: 104..109,
                                        },
                                    ),
                                    left: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 102..103,
                                    },
                                    right: Expression {
                                        kind: Placeholder,
                                        source: 110..111,
                                    },
                                },
                                source: 104..111,
                            },
                        ),
                        source: 102..112,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..112,
            }"#]],
    );
}

#[test]
fn infix_operators_as_identifiers() {
    assert_ast(
        r#"
            call(+, -, /, %, >, <, >=, <=)
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "call",
                                        ),
                                        source: 0..4,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Identifier(
                                                "+",
                                            ),
                                            source: 5..6,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "-",
                                            ),
                                            source: 8..9,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "/",
                                            ),
                                            source: 11..12,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "%",
                                            ),
                                            source: 14..15,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                ">",
                                            ),
                                            source: 17..18,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "<",
                                            ),
                                            source: 20..21,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                ">=",
                                            ),
                                            source: 23..25,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "<=",
                                            ),
                                            source: 27..29,
                                        },
                                    ],
                                },
                                source: 4..30,
                            },
                        ),
                        source: 0..30,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..30,
            }"#]],
    );
}

#[test]
fn function_composition() {
    assert_ast(
        r#"
            inc >> _ + 1 >> add(1)
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: FunctionComposition(
                                    [
                                        Expression {
                                            kind: Identifier(
                                                "inc",
                                            ),
                                            source: 0..3,
                                        },
                                        Expression {
                                            kind: Infix {
                                                operator: Plus,
                                                left: Expression {
                                                    kind: Placeholder,
                                                    source: 7..8,
                                                },
                                                right: Expression {
                                                    kind: Integer(
                                                        "1",
                                                    ),
                                                    source: 11..12,
                                                },
                                            },
                                            source: 9..13,
                                        },
                                        Expression {
                                            kind: Call {
                                                function: Expression {
                                                    kind: Identifier(
                                                        "add",
                                                    ),
                                                    source: 16..19,
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Integer(
                                                            "1",
                                                        ),
                                                        source: 20..21,
                                                    },
                                                ],
                                            },
                                            source: 19..22,
                                        },
                                    ],
                                ),
                                source: 4..22,
                            },
                        ),
                        source: 0..22,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..22,
            }"#]],
    );
}

#[test]
fn function_threading() {
    assert_ast(
        r#"
            [1, 2, 3] |> map(|x| x + 1);
            1 |> add(1) |> |a| { a + 1 } |> inc |> _ + 1;
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: FunctionThread {
                                    initial: Expression {
                                        kind: List(
                                            [
                                                Expression {
                                                    kind: Integer(
                                                        "1",
                                                    ),
                                                    source: 1..2,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "2",
                                                    ),
                                                    source: 4..5,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "3",
                                                    ),
                                                    source: 7..8,
                                                },
                                            ],
                                        ),
                                        source: 0..10,
                                    },
                                    functions: [
                                        Expression {
                                            kind: Call {
                                                function: Expression {
                                                    kind: Identifier(
                                                        "map",
                                                    ),
                                                    source: 13..16,
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Function {
                                                            parameters: [
                                                                Expression {
                                                                    kind: Identifier(
                                                                        "x",
                                                                    ),
                                                                    source: 18..19,
                                                                },
                                                            ],
                                                            body: Statement {
                                                                kind: Block(
                                                                    [
                                                                        Statement {
                                                                            kind: Expression(
                                                                                Expression {
                                                                                    kind: Infix {
                                                                                        operator: Plus,
                                                                                        left: Expression {
                                                                                            kind: Identifier(
                                                                                                "x",
                                                                                            ),
                                                                                            source: 21..22,
                                                                                        },
                                                                                        right: Expression {
                                                                                            kind: Integer(
                                                                                                "1",
                                                                                            ),
                                                                                            source: 25..26,
                                                                                        },
                                                                                    },
                                                                                    source: 23..26,
                                                                                },
                                                                            ),
                                                                            source: 21..26,
                                                                            preceded_by_blank_line: false,
                                                                            trailing_comment: None,
                                                                        },
                                                                    ],
                                                                ),
                                                                source: 21..26,
                                                                preceded_by_blank_line: false,
                                                                trailing_comment: None,
                                                            },
                                                        },
                                                        source: 17..26,
                                                    },
                                                ],
                                            },
                                            source: 16..27,
                                        },
                                    ],
                                },
                                source: 10..27,
                            },
                        ),
                        source: 0..41,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: FunctionThread {
                                    initial: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 41..42,
                                    },
                                    functions: [
                                        Expression {
                                            kind: Call {
                                                function: Expression {
                                                    kind: Identifier(
                                                        "add",
                                                    ),
                                                    source: 46..49,
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Integer(
                                                            "1",
                                                        ),
                                                        source: 50..51,
                                                    },
                                                ],
                                            },
                                            source: 49..53,
                                        },
                                        Expression {
                                            kind: Function {
                                                parameters: [
                                                    Expression {
                                                        kind: Identifier(
                                                            "a",
                                                        ),
                                                        source: 57..58,
                                                    },
                                                ],
                                                body: Statement {
                                                    kind: Block(
                                                        [
                                                            Statement {
                                                                kind: Expression(
                                                                    Expression {
                                                                        kind: Infix {
                                                                            operator: Plus,
                                                                            left: Expression {
                                                                                kind: Identifier(
                                                                                    "a",
                                                                                ),
                                                                                source: 62..63,
                                                                            },
                                                                            right: Expression {
                                                                                kind: Integer(
                                                                                    "1",
                                                                                ),
                                                                                source: 66..67,
                                                                            },
                                                                        },
                                                                        source: 64..68,
                                                                    },
                                                                ),
                                                                source: 62..68,
                                                                preceded_by_blank_line: false,
                                                                trailing_comment: None,
                                                            },
                                                        ],
                                                    ),
                                                    source: 60..70,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            },
                                            source: 56..70,
                                        },
                                        Expression {
                                            kind: Identifier(
                                                "inc",
                                            ),
                                            source: 73..76,
                                        },
                                        Expression {
                                            kind: Infix {
                                                operator: Plus,
                                                left: Expression {
                                                    kind: Placeholder,
                                                    source: 80..81,
                                                },
                                                right: Expression {
                                                    kind: Integer(
                                                        "1",
                                                    ),
                                                    source: 84..85,
                                                },
                                            },
                                            source: 82..85,
                                        },
                                    ],
                                },
                                source: 43..85,
                            },
                        ),
                        source: 41..86,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..86,
            }"#]],
    );
}

#[test]
fn trailing_lambdas() {
    assert_ast(
        r#"
            with (x) |y| { y };
            [1, 2, 3] |> each |x| { puts(x) };
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Call {
                                    function: Expression {
                                        kind: Identifier(
                                            "with",
                                        ),
                                        source: 0..4,
                                    },
                                    arguments: [
                                        Expression {
                                            kind: Identifier(
                                                "x",
                                            ),
                                            source: 6..7,
                                        },
                                        Expression {
                                            kind: Function {
                                                parameters: [
                                                    Expression {
                                                        kind: Identifier(
                                                            "y",
                                                        ),
                                                        source: 10..11,
                                                    },
                                                ],
                                                body: Statement {
                                                    kind: Block(
                                                        [
                                                            Statement {
                                                                kind: Expression(
                                                                    Expression {
                                                                        kind: Identifier(
                                                                            "y",
                                                                        ),
                                                                        source: 15..16,
                                                                    },
                                                                ),
                                                                source: 15..17,
                                                                preceded_by_blank_line: false,
                                                                trailing_comment: None,
                                                            },
                                                        ],
                                                    ),
                                                    source: 13..18,
                                                    preceded_by_blank_line: false,
                                                    trailing_comment: None,
                                                },
                                            },
                                            source: 9..18,
                                        },
                                    ],
                                },
                                source: 5..18,
                            },
                        ),
                        source: 0..32,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: FunctionThread {
                                    initial: Expression {
                                        kind: List(
                                            [
                                                Expression {
                                                    kind: Integer(
                                                        "1",
                                                    ),
                                                    source: 33..34,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "2",
                                                    ),
                                                    source: 36..37,
                                                },
                                                Expression {
                                                    kind: Integer(
                                                        "3",
                                                    ),
                                                    source: 39..40,
                                                },
                                            ],
                                        ),
                                        source: 32..42,
                                    },
                                    functions: [
                                        Expression {
                                            kind: Call {
                                                function: Expression {
                                                    kind: Identifier(
                                                        "each",
                                                    ),
                                                    source: 45..49,
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Function {
                                                            parameters: [
                                                                Expression {
                                                                    kind: Identifier(
                                                                        "x",
                                                                    ),
                                                                    source: 51..52,
                                                                },
                                                            ],
                                                            body: Statement {
                                                                kind: Block(
                                                                    [
                                                                        Statement {
                                                                            kind: Expression(
                                                                                Expression {
                                                                                    kind: Call {
                                                                                        function: Expression {
                                                                                            kind: Identifier(
                                                                                                "puts",
                                                                                            ),
                                                                                            source: 56..60,
                                                                                        },
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Identifier(
                                                                                                    "x",
                                                                                                ),
                                                                                                source: 61..62,
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                    source: 60..64,
                                                                                },
                                                                            ),
                                                                            source: 56..64,
                                                                            preceded_by_blank_line: false,
                                                                            trailing_comment: None,
                                                                        },
                                                                    ],
                                                                ),
                                                                source: 54..65,
                                                                preceded_by_blank_line: false,
                                                                trailing_comment: None,
                                                            },
                                                        },
                                                        source: 50..65,
                                                    },
                                                ],
                                            },
                                            source: 45..65,
                                        },
                                    ],
                                },
                                source: 42..65,
                            },
                        ),
                        source: 32..66,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..66,
            }"#]],
    );
}

#[test]
fn collection_indexing() {
    assert_ast(
        r#"
            col[1];
            col[-1];
            col[2..=5];
            col[2..5];
            col[-2..];
            col[0..=-2];
            col[0..-2];
            col["key"];
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 0..3,
                                    },
                                    index: Expression {
                                        kind: Integer(
                                            "1",
                                        ),
                                        source: 4..5,
                                    },
                                },
                                source: 3..6,
                            },
                        ),
                        source: 0..20,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 20..23,
                                    },
                                    index: Expression {
                                        kind: Prefix {
                                            operator: Minus,
                                            right: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 25..26,
                                            },
                                        },
                                        source: 24..26,
                                    },
                                },
                                source: 23..27,
                            },
                        ),
                        source: 20..41,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 41..44,
                                    },
                                    index: Expression {
                                        kind: InclusiveRange {
                                            from: Expression {
                                                kind: Integer(
                                                    "2",
                                                ),
                                                source: 45..46,
                                            },
                                            to: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 49..50,
                                            },
                                        },
                                        source: 46..50,
                                    },
                                },
                                source: 44..51,
                            },
                        ),
                        source: 41..65,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 65..68,
                                    },
                                    index: Expression {
                                        kind: ExclusiveRange {
                                            from: Expression {
                                                kind: Integer(
                                                    "2",
                                                ),
                                                source: 69..70,
                                            },
                                            until: Expression {
                                                kind: Integer(
                                                    "5",
                                                ),
                                                source: 72..73,
                                            },
                                        },
                                        source: 70..73,
                                    },
                                },
                                source: 68..74,
                            },
                        ),
                        source: 65..88,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 88..91,
                                    },
                                    index: Expression {
                                        kind: UnboundedRange {
                                            from: Expression {
                                                kind: Prefix {
                                                    operator: Minus,
                                                    right: Expression {
                                                        kind: Integer(
                                                            "2",
                                                        ),
                                                        source: 93..94,
                                                    },
                                                },
                                                source: 92..94,
                                            },
                                        },
                                        source: 94..96,
                                    },
                                },
                                source: 91..97,
                            },
                        ),
                        source: 88..111,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 111..114,
                                    },
                                    index: Expression {
                                        kind: InclusiveRange {
                                            from: Expression {
                                                kind: Integer(
                                                    "0",
                                                ),
                                                source: 115..116,
                                            },
                                            to: Expression {
                                                kind: Prefix {
                                                    operator: Minus,
                                                    right: Expression {
                                                        kind: Integer(
                                                            "2",
                                                        ),
                                                        source: 120..121,
                                                    },
                                                },
                                                source: 119..121,
                                            },
                                        },
                                        source: 116..121,
                                    },
                                },
                                source: 114..122,
                            },
                        ),
                        source: 111..136,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 136..139,
                                    },
                                    index: Expression {
                                        kind: ExclusiveRange {
                                            from: Expression {
                                                kind: Integer(
                                                    "0",
                                                ),
                                                source: 140..141,
                                            },
                                            until: Expression {
                                                kind: Prefix {
                                                    operator: Minus,
                                                    right: Expression {
                                                        kind: Integer(
                                                            "2",
                                                        ),
                                                        source: 144..145,
                                                    },
                                                },
                                                source: 143..145,
                                            },
                                        },
                                        source: 141..145,
                                    },
                                },
                                source: 139..146,
                            },
                        ),
                        source: 136..160,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Index {
                                    left: Expression {
                                        kind: Identifier(
                                            "col",
                                        ),
                                        source: 160..163,
                                    },
                                    index: Expression {
                                        kind: String(
                                            "key",
                                        ),
                                        source: 164..169,
                                    },
                                },
                                source: 163..170,
                            },
                        ),
                        source: 160..171,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..171,
            }"#]],
    );
}

#[test]
fn sections() {
    assert_ast(
        r#"
            section_one: { "sample" };
            section_two: "sample";
            section_three: {
                section_four: "sample";
            };
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Section {
                            name: "section_one",
                            body: Program {
                                statements: [
                                    Statement {
                                        kind: Expression(
                                            Expression {
                                                kind: String(
                                                    "sample",
                                                ),
                                                source: 15..23,
                                            },
                                        ),
                                        source: 15..24,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                ],
                                source: 0..25,
                            },
                            attributes: [],
                        },
                        source: 0..39,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
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
                                                source: 52..60,
                                            },
                                        ),
                                        source: 52..74,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                ],
                                source: 39..74,
                            },
                            attributes: [],
                        },
                        source: 39..74,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                    Statement {
                        kind: Section {
                            name: "section_three",
                            body: Program {
                                statements: [
                                    Statement {
                                        kind: Section {
                                            name: "section_four",
                                            body: Program {
                                                statements: [
                                                    Statement {
                                                        kind: Expression(
                                                            Expression {
                                                                kind: String(
                                                                    "sample",
                                                                ),
                                                                source: 121..129,
                                                            },
                                                        ),
                                                        source: 121..143,
                                                        preceded_by_blank_line: false,
                                                        trailing_comment: None,
                                                    },
                                                ],
                                                source: 107..143,
                                            },
                                            attributes: [],
                                        },
                                        source: 107..143,
                                        preceded_by_blank_line: false,
                                        trailing_comment: None,
                                    },
                                ],
                                source: 74..144,
                            },
                            attributes: [],
                        },
                        source: 74..145,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..145,
            }"#]],
    );
}

#[test]
fn match_with_primitives() {
    assert_ast(
        r#"
            match x {
                1 { "one" },
                2.0 { "two" },
                true { "three" },
                "four" { 4 }
            }
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Match {
                                    subject: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 6..7,
                                    },
                                    cases: [
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 26..27,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "one",
                                                                    ),
                                                                    source: 30..35,
                                                                },
                                                            ),
                                                            source: 30..36,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 28..37,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Decimal(
                                                    "2.0",
                                                ),
                                                source: 55..58,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "two",
                                                                    ),
                                                                    source: 61..66,
                                                                },
                                                            ),
                                                            source: 61..67,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 59..68,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Boolean(
                                                    true,
                                                ),
                                                source: 86..90,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "three",
                                                                    ),
                                                                    source: 93..100,
                                                                },
                                                            ),
                                                            source: 93..101,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 91..102,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: String(
                                                    "four",
                                                ),
                                                source: 120..126,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: Integer(
                                                                        "4",
                                                                    ),
                                                                    source: 129..130,
                                                                },
                                                            ),
                                                            source: 129..131,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 127..145,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                    ],
                                },
                                source: 0..146,
                            },
                        ),
                        source: 0..146,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..146,
            }"#]],
    );
}

#[test]
fn match_with_ranges() {
    assert_ast(
        r#"
            match x {
                1..3 { "1-2" },
                3..=5 { "3-5" },
                6.. { ">= 6" }
            }
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Match {
                                    subject: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 6..7,
                                    },
                                    cases: [
                                        MatchCase {
                                            pattern: Expression {
                                                kind: ExclusiveRange {
                                                    from: Expression {
                                                        kind: Integer(
                                                            "1",
                                                        ),
                                                        source: 26..27,
                                                    },
                                                    until: Expression {
                                                        kind: Integer(
                                                            "3",
                                                        ),
                                                        source: 29..30,
                                                    },
                                                },
                                                source: 27..31,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "1-2",
                                                                    ),
                                                                    source: 33..38,
                                                                },
                                                            ),
                                                            source: 33..39,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 31..40,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: InclusiveRange {
                                                    from: Expression {
                                                        kind: Integer(
                                                            "3",
                                                        ),
                                                        source: 58..59,
                                                    },
                                                    to: Expression {
                                                        kind: Integer(
                                                            "5",
                                                        ),
                                                        source: 62..63,
                                                    },
                                                },
                                                source: 59..64,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "3-5",
                                                                    ),
                                                                    source: 66..71,
                                                                },
                                                            ),
                                                            source: 66..72,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 64..73,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: UnboundedRange {
                                                    from: Expression {
                                                        kind: Integer(
                                                            "6",
                                                        ),
                                                        source: 91..92,
                                                    },
                                                },
                                                source: 92..95,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        ">= 6",
                                                                    ),
                                                                    source: 97..103,
                                                                },
                                                            ),
                                                            source: 97..104,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 95..118,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                    ],
                                },
                                source: 0..119,
                            },
                        ),
                        source: 0..119,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..119,
            }"#]],
    );
}

#[test]
fn match_with_if_guards() {
    assert_ast(
        r#"
            match x {
                1 if 1 != 2 { "one" },
                2.0 if true && !false { "two" },
                e if e > 3 { "greater than three" }
            }
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Match {
                                    subject: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 6..7,
                                    },
                                    cases: [
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Integer(
                                                    "1",
                                                ),
                                                source: 26..27,
                                            },
                                            guard: Some(
                                                Expression {
                                                    kind: Infix {
                                                        operator: NotEqual,
                                                        left: Expression {
                                                            kind: Integer(
                                                                "1",
                                                            ),
                                                            source: 31..32,
                                                        },
                                                        right: Expression {
                                                            kind: Integer(
                                                                "2",
                                                            ),
                                                            source: 36..37,
                                                        },
                                                    },
                                                    source: 33..38,
                                                },
                                            ),
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "one",
                                                                    ),
                                                                    source: 40..45,
                                                                },
                                                            ),
                                                            source: 40..46,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 38..47,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Decimal(
                                                    "2.0",
                                                ),
                                                source: 65..68,
                                            },
                                            guard: Some(
                                                Expression {
                                                    kind: Infix {
                                                        operator: And,
                                                        left: Expression {
                                                            kind: Boolean(
                                                                true,
                                                            ),
                                                            source: 72..76,
                                                        },
                                                        right: Expression {
                                                            kind: Prefix {
                                                                operator: Bang,
                                                                right: Expression {
                                                                    kind: Boolean(
                                                                        false,
                                                                    ),
                                                                    source: 81..86,
                                                                },
                                                            },
                                                            source: 80..87,
                                                        },
                                                    },
                                                    source: 77..87,
                                                },
                                            ),
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "two",
                                                                    ),
                                                                    source: 89..94,
                                                                },
                                                            ),
                                                            source: 89..95,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 87..96,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Identifier(
                                                    "e",
                                                ),
                                                source: 114..115,
                                            },
                                            guard: Some(
                                                Expression {
                                                    kind: Infix {
                                                        operator: GreaterThan,
                                                        left: Expression {
                                                            kind: Identifier(
                                                                "e",
                                                            ),
                                                            source: 119..120,
                                                        },
                                                        right: Expression {
                                                            kind: Integer(
                                                                "3",
                                                            ),
                                                            source: 123..124,
                                                        },
                                                    },
                                                    source: 121..125,
                                                },
                                            ),
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "greater than three",
                                                                    ),
                                                                    source: 127..147,
                                                                },
                                                            ),
                                                            source: 127..148,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 125..162,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                    ],
                                },
                                source: 0..163,
                            },
                        ),
                        source: 0..163,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..163,
            }"#]],
    );
}

#[test]
fn match_with_list_destructing() {
    assert_ast(
        r#"
            match x {
                [] { "empty" },
                [x] { "one item" },
                [x, ..xs] { "many items" },
                _ { "unknown" } }
            }
        "#,
        expect![[r#"
            Program {
                statements: [
                    Statement {
                        kind: Expression(
                            Expression {
                                kind: Match {
                                    subject: Expression {
                                        kind: Identifier(
                                            "x",
                                        ),
                                        source: 6..7,
                                    },
                                    cases: [
                                        MatchCase {
                                            pattern: Expression {
                                                kind: ListMatchPattern(
                                                    [],
                                                ),
                                                source: 26..27,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "empty",
                                                                    ),
                                                                    source: 31..38,
                                                                },
                                                            ),
                                                            source: 31..39,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 29..40,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: ListMatchPattern(
                                                    [
                                                        Expression {
                                                            kind: Identifier(
                                                                "x",
                                                            ),
                                                            source: 59..60,
                                                        },
                                                    ],
                                                ),
                                                source: 58..62,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "one item",
                                                                    ),
                                                                    source: 64..74,
                                                                },
                                                            ),
                                                            source: 64..75,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 62..76,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: ListMatchPattern(
                                                    [
                                                        Expression {
                                                            kind: Identifier(
                                                                "x",
                                                            ),
                                                            source: 95..96,
                                                        },
                                                        Expression {
                                                            kind: RestIdentifier(
                                                                "xs",
                                                            ),
                                                            source: 98..102,
                                                        },
                                                    ],
                                                ),
                                                source: 94..104,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "many items",
                                                                    ),
                                                                    source: 106..118,
                                                                },
                                                            ),
                                                            source: 106..119,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 104..120,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                        MatchCase {
                                            pattern: Expression {
                                                kind: Placeholder,
                                                source: 138..139,
                                            },
                                            guard: None,
                                            consequence: Statement {
                                                kind: Block(
                                                    [
                                                        Statement {
                                                            kind: Expression(
                                                                Expression {
                                                                    kind: String(
                                                                        "unknown",
                                                                    ),
                                                                    source: 142..151,
                                                                },
                                                            ),
                                                            source: 142..152,
                                                            preceded_by_blank_line: false,
                                                            trailing_comment: None,
                                                        },
                                                    ],
                                                ),
                                                source: 140..154,
                                                preceded_by_blank_line: false,
                                                trailing_comment: None,
                                            },
                                            trailing_comment: None,
                                        },
                                    ],
                                },
                                source: 0..168,
                            },
                        ),
                        source: 0..168,
                        preceded_by_blank_line: false,
                        trailing_comment: None,
                    },
                ],
                source: 0..168,
            }"#]],
    );
}

#[test]
fn infix_precedence() {
    fn case(input: &str, expected: &str) {
        let mut parser = Parser::new(Lexer::new(input.trim()));
        let program = parser.parse();
        let actual = format!("{}", program.expect("Ok"));
        assert_eq!(expected, actual);
    }

    case("-a * b", "((-a) * b)");
    case("!-a", "(!(-a))");
    case("a + b + c", "((a + b) + c)");
    case("a + b - c", "((a + b) - c)");
    case("a * b * c", "((a * b) * c)");
    case("a * b / c", "((a * b) / c)");
    case("a + b / c", "(a + (b / c))");
    case("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)");
    case("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)");
    case("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))");
    case("5 >= 4 == 3 <= 4", "((5 >= 4) == (3 <= 4))");
    case("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))");
    case("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))");
    case("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)");
    case("(5 + 5) * 2", "((5 + 5) * 2)");
    case("2 / (5 + 5)", "(2 / (5 + 5))");
    case("-(5 + 5)", "(-(5 + 5))");
    case("!(true == true)", "(!(true == true))");
    case("a + add(b * c) + d", "((a + add((b * c))) + d)");
    case(
        "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
        "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
    );
    case("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))");
    case("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)");
    case(
        "add(a * b[2], b[1], 2 * [1, 2][1])",
        "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
    );
    case(
        "1 |> add(3) != [1, 2, 3] |> mul",
        "((1 |> add(3)) != ([1, 2, 3] |> mul))",
    );
    case(
        "1 |> add(1) |> |a| { a + 1 } |> inc |> _ + 1",
        "(1 |> add(1) |> |a| (a + 1) |> inc |> (_ + 1))",
    );
}

#[test]
fn invalid_function() {
    assert_error(
        "|ddd { 1 }",
        expect![[r#"
            ParserErr {
                message: "Expected: Pipe, Actual: LBrace",
                source: 5..6,
            }"#]],
    );
}

#[test]
fn invalid_match() {
    assert_error(
        "match d { { 2 }",
        expect![[r#"
            ParserErr {
                message: "LBrace is not legal in a match pattern",
                source: 10..11,
            }"#]],
    );
}

#[test]
fn invalid_if() {
    assert_error(
        "if 3 { 3",
        expect![[r#"
            ParserErr {
                message: "Expected: RBrace, Actual: Eof",
                source: 8..8,
            }"#]],
    );
}

#[test]
fn invalid_infix() {
    assert_error(
        "1 <",
        expect![[r#"
            ParserErr {
                message: "Eof is not a legal identifier",
                source: 3..3,
            }"#]],
    );
}

#[test]
fn illegal_token() {
    assert_error(
        "h@llo",
        expect![[r#"
            ParserErr {
                message: "Unexpected attribute - attributes can only be applied to sections",
                source: 1..2,
            }"#]],
    );
}

fn assert_ast(input: &str, expected: Expect) {
    let mut parser = Parser::new(Lexer::new(input.trim()));
    let program = parser.parse();
    let actual = format!("{:#?}", program.expect("Ok"));
    expected.assert_eq(&actual)
}

fn assert_error(input: &str, expected: Expect) {
    let mut parser = Parser::new(Lexer::new(input.trim()));
    let actual = match parser.parse() {
        Err(err) => format!("{:#?}", err),
        _ => "".to_owned(),
    };
    expected.assert_eq(&actual)
}

use super::*;

use expect_test::{expect, Expect};

#[test]
fn integers() {
    assert_ast(
        "
            1;
            1_000_000;
        ",
        expect![[r#"
            Program {
                source: 0..25,
                statements: [
                    Expression {
                        source: 0..15,
                        expression: Integer {
                            source: 0..1,
                            value: "1",
                        },
                    },
                    Expression {
                        source: 15..25,
                        expression: Integer {
                            source: 15..24,
                            value: "1_000_000",
                        },
                    },
                ],
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
                source: 0..30,
                statements: [
                    Expression {
                        source: 0..17,
                        expression: Decimal {
                            source: 0..3,
                            value: "1.5",
                        },
                    },
                    Expression {
                        source: 17..30,
                        expression: Decimal {
                            source: 17..29,
                            value: "1_000_000.50",
                        },
                    },
                ],
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
                source: 0..60,
                statements: [
                    Expression {
                        source: 0..28,
                        expression: String {
                            source: 0..15,
                            value: "\"Hello, world!\"",
                        },
                    },
                    Expression {
                        source: 28..49,
                        expression: String {
                            source: 28..36,
                            value: "\"\\n\\t\\\"\"",
                        },
                    },
                    Expression {
                        source: 49..60,
                        expression: String {
                            source: 49..60,
                            value: "\"µࠒ𒀀\"",
                        },
                    },
                ],
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
                source: 0..24,
                statements: [
                    Expression {
                        source: 0..18,
                        expression: Boolean {
                            source: 0..4,
                            value: true,
                        },
                    },
                    Expression {
                        source: 18..24,
                        expression: Boolean {
                            source: 18..23,
                            value: false,
                        },
                    },
                ],
            }"#]],
    );
}

#[test]
fn nil() {
    assert_ast(
        "nil;",
        expect![[r#"
            Program {
                source: 0..4,
                statements: [
                    Expression {
                        source: 0..4,
                        expression: Nil {
                            source: 0..3,
                        },
                    },
                ],
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
                source: 0..58,
                statements: [
                    Comment {
                        source: 0..20,
                        value: "// full line comment",
                    },
                    Expression {
                        source: 33..36,
                        expression: Integer {
                            source: 33..34,
                            value: "1",
                        },
                    },
                    Comment {
                        source: 36..58,
                        value: "// end of line comment",
                    },
                ],
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
                source: 0..41,
                statements: [
                    Expression {
                        source: 0..41,
                        expression: List {
                            source: 0..41,
                            elements: [
                                Integer {
                                    source: 1..2,
                                    value: "1",
                                },
                                Decimal {
                                    source: 4..7,
                                    value: "2.5",
                                },
                                String {
                                    source: 9..24,
                                    value: "\"Hello, world!\"",
                                },
                                SpreadElement {
                                    source: 26..28,
                                    value: Identifier {
                                        source: 28..30,
                                        name: "xs",
                                    },
                                },
                                SpreadElement {
                                    source: 32..34,
                                    value: List {
                                        source: 34..40,
                                        elements: [
                                            Boolean {
                                                source: 35..39,
                                                value: true,
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
            }"#]],
    );
}

#[test]
fn hash() {
    assert_ast(
        r#"
            #{"Hello, world!": #{x}, 1: "2", [1, 2]: 1.4}
        "#,
        expect![[r#"
            Program {
                source: 0..45,
                statements: [
                    Expression {
                        source: 0..45,
                        expression: Hash {
                            source: 0..45,
                            elements: [
                                (
                                    String {
                                        source: 2..17,
                                        value: "\"Hello, world!\"",
                                    },
                                    Hash {
                                        source: 19..23,
                                        elements: [
                                            (
                                                String {
                                                    source: 21..22,
                                                    value: "x",
                                                },
                                                Identifier {
                                                    source: 21..22,
                                                    name: "x",
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                (
                                    Integer {
                                        source: 25..26,
                                        value: "1",
                                    },
                                    String {
                                        source: 28..31,
                                        value: "\"2\"",
                                    },
                                ),
                                (
                                    List {
                                        source: 33..39,
                                        elements: [
                                            Integer {
                                                source: 34..35,
                                                value: "1",
                                            },
                                            Integer {
                                                source: 37..38,
                                                value: "2",
                                            },
                                        ],
                                    },
                                    Decimal {
                                        source: 41..44,
                                        value: "1.4",
                                    },
                                ),
                            ],
                        },
                    },
                ],
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
                source: 0..35,
                statements: [
                    Expression {
                        source: 0..35,
                        expression: Set {
                            source: 0..35,
                            elements: [
                                Integer {
                                    source: 1..2,
                                    value: "1",
                                },
                                Decimal {
                                    source: 4..7,
                                    value: "2.5",
                                },
                                String {
                                    source: 9..24,
                                    value: "\"Hello, world!\"",
                                },
                                SpreadElement {
                                    source: 26..28,
                                    value: List {
                                        source: 28..34,
                                        elements: [
                                            Boolean {
                                                source: 29..33,
                                                value: true,
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..125,
                statements: [
                    Expression {
                        source: 0..7,
                        expression: ExclusiveRange {
                            source: 1..5,
                            from: Integer {
                                source: 0..1,
                                value: "1",
                            },
                            until: Integer {
                                source: 3..5,
                                value: "10",
                            },
                        },
                    },
                    Expression {
                        source: 7..13,
                        expression: ExclusiveRange {
                            source: 8..11,
                            from: Identifier {
                                source: 7..8,
                                name: "x",
                            },
                            until: Identifier {
                                source: 10..11,
                                name: "y",
                            },
                        },
                    },
                    Expression {
                        source: 13..20,
                        expression: ExclusiveRange {
                            source: 15..18,
                            from: Prefix {
                                source: 13..15,
                                operator: Minus,
                                right: Integer {
                                    source: 14..15,
                                    value: "1",
                                },
                            },
                            until: Integer {
                                source: 17..18,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 20..27,
                        expression: ExclusiveRange {
                            source: 21..25,
                            from: Integer {
                                source: 20..21,
                                value: "1",
                            },
                            until: Prefix {
                                source: 23..25,
                                operator: Minus,
                                right: Integer {
                                    source: 24..25,
                                    value: "1",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 27..49,
                        expression: ExclusiveRange {
                            source: 30..35,
                            from: Integer {
                                source: 28..29,
                                value: "1",
                            },
                            until: Integer {
                                source: 33..34,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 49..57,
                        expression: InclusiveRange {
                            source: 50..55,
                            from: Integer {
                                source: 49..50,
                                value: "1",
                            },
                            to: Integer {
                                source: 53..55,
                                value: "10",
                            },
                        },
                    },
                    Expression {
                        source: 57..64,
                        expression: InclusiveRange {
                            source: 58..62,
                            from: Identifier {
                                source: 57..58,
                                name: "x",
                            },
                            to: Identifier {
                                source: 61..62,
                                name: "y",
                            },
                        },
                    },
                    Expression {
                        source: 64..72,
                        expression: InclusiveRange {
                            source: 66..70,
                            from: Prefix {
                                source: 64..66,
                                operator: Minus,
                                right: Integer {
                                    source: 65..66,
                                    value: "1",
                                },
                            },
                            to: Integer {
                                source: 69..70,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 72..80,
                        expression: InclusiveRange {
                            source: 73..78,
                            from: Integer {
                                source: 72..73,
                                value: "1",
                            },
                            to: Prefix {
                                source: 76..78,
                                operator: Minus,
                                right: Integer {
                                    source: 77..78,
                                    value: "1",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 80..103,
                        expression: InclusiveRange {
                            source: 83..89,
                            from: Integer {
                                source: 81..82,
                                value: "1",
                            },
                            to: Integer {
                                source: 87..88,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 103..108,
                        expression: UnboundedRange {
                            source: 104..106,
                            from: Integer {
                                source: 103..104,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 108..113,
                        expression: UnboundedRange {
                            source: 109..111,
                            from: Identifier {
                                source: 108..109,
                                name: "x",
                            },
                        },
                    },
                    Expression {
                        source: 113..119,
                        expression: UnboundedRange {
                            source: 115..117,
                            from: Prefix {
                                source: 113..115,
                                operator: Minus,
                                right: Integer {
                                    source: 114..115,
                                    value: "1",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 119..125,
                        expression: UnboundedRange {
                            source: 122..124,
                            from: Integer {
                                source: 120..121,
                                value: "1",
                            },
                        },
                    },
                ],
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
                source: 0..113,
                statements: [
                    Expression {
                        source: 0..23,
                        expression: Let {
                            source: 0..9,
                            name: Identifier {
                                source: 4..5,
                                name: "x",
                            },
                            value: Integer {
                                source: 8..9,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 23..50,
                        expression: MutableLet {
                            source: 23..36,
                            name: Identifier {
                                source: 31..32,
                                name: "y",
                            },
                            value: Integer {
                                source: 35..36,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 50..69,
                        expression: Assign {
                            source: 52..55,
                            name: Identifier {
                                source: 50..51,
                                name: "y",
                            },
                            value: Integer {
                                source: 54..55,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 69..113,
                        expression: Let {
                            source: 69..112,
                            name: IdentifierListPattern {
                                source: 69..93,
                                pattern: [
                                    Identifier {
                                        source: 74..75,
                                        name: "a",
                                    },
                                    Identifier {
                                        source: 77..78,
                                        name: "b",
                                    },
                                    IdentifierListPattern {
                                        source: 80..81,
                                        pattern: [
                                            Identifier {
                                                source: 81..82,
                                                name: "c",
                                            },
                                            Identifier {
                                                source: 84..85,
                                                name: "d",
                                            },
                                        ],
                                    },
                                    RestElement {
                                        source: 88..90,
                                        name: Identifier {
                                            source: 90..91,
                                            name: "e",
                                        },
                                    },
                                ],
                            },
                            value: List {
                                source: 95..112,
                                elements: [
                                    Integer {
                                        source: 96..97,
                                        value: "1",
                                    },
                                    Integer {
                                        source: 99..100,
                                        value: "2",
                                    },
                                    List {
                                        source: 102..108,
                                        elements: [
                                            Integer {
                                                source: 103..104,
                                                value: "3",
                                            },
                                            Integer {
                                                source: 106..107,
                                                value: "4",
                                            },
                                        ],
                                    },
                                    Integer {
                                        source: 110..111,
                                        value: "5",
                                    },
                                ],
                            },
                        },
                    },
                ],
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
                source: 0..83,
                statements: [
                    Expression {
                        source: 0..27,
                        expression: If {
                            source: 0..27,
                            condition: Infix {
                                source: 5..9,
                                operator: LessThan,
                                left: Integer {
                                    source: 3..4,
                                    value: "2",
                                },
                                right: Integer {
                                    source: 7..8,
                                    value: "5",
                                },
                            },
                            consequence: Block {
                                source: 9..27,
                                statements: [
                                    Expression {
                                        source: 11..13,
                                        expression: Integer {
                                            source: 11..12,
                                            value: "1",
                                        },
                                    },
                                ],
                            },
                            alternative: None,
                        },
                    },
                    Expression {
                        source: 27..65,
                        expression: If {
                            source: 27..65,
                            condition: Infix {
                                source: 32..36,
                                operator: GreaterThan,
                                left: Integer {
                                    source: 30..31,
                                    value: "3",
                                },
                                right: Integer {
                                    source: 34..35,
                                    value: "5",
                                },
                            },
                            consequence: Block {
                                source: 36..42,
                                statements: [
                                    Expression {
                                        source: 38..40,
                                        expression: Integer {
                                            source: 38..39,
                                            value: "1",
                                        },
                                    },
                                ],
                            },
                            alternative: Some(
                                Block {
                                    source: 47..65,
                                    statements: [
                                        Expression {
                                            source: 49..51,
                                            expression: Integer {
                                                source: 49..50,
                                                value: "2",
                                            },
                                        },
                                    ],
                                },
                            ),
                        },
                    },
                    Expression {
                        source: 65..83,
                        expression: If {
                            source: 65..83,
                            condition: Let {
                                source: 68..78,
                                name: Identifier {
                                    source: 72..73,
                                    name: "x",
                                },
                                value: Integer {
                                    source: 76..77,
                                    value: "1",
                                },
                            },
                            consequence: Block {
                                source: 78..83,
                                statements: [
                                    Expression {
                                        source: 80..82,
                                        expression: Identifier {
                                            source: 80..81,
                                            name: "x",
                                        },
                                    },
                                ],
                            },
                            alternative: None,
                        },
                    },
                ],
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
                source: 0..79,
                statements: [
                    Expression {
                        source: 0..16,
                        expression: Prefix {
                            source: 0..2,
                            operator: Minus,
                            right: Integer {
                                source: 1..2,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 16..33,
                        expression: Prefix {
                            source: 16..19,
                            operator: Minus,
                            right: Prefix {
                                source: 17..19,
                                operator: Minus,
                                right: Integer {
                                    source: 18..19,
                                    value: "1",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 33..53,
                        expression: Infix {
                            source: 35..39,
                            operator: Minus,
                            left: Integer {
                                source: 33..34,
                                value: "4",
                            },
                            right: Prefix {
                                source: 37..39,
                                operator: Minus,
                                right: Integer {
                                    source: 38..39,
                                    value: "4",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 53..72,
                        expression: Prefix {
                            source: 53..58,
                            operator: Bang,
                            right: Boolean {
                                source: 54..58,
                                value: true,
                            },
                        },
                    },
                    Expression {
                        source: 72..79,
                        expression: Prefix {
                            source: 72..78,
                            operator: Bang,
                            right: Prefix {
                                source: 73..78,
                                operator: Bang,
                                right: Boolean {
                                    source: 74..78,
                                    value: true,
                                },
                            },
                        },
                    },
                ],
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
                source: 0..152,
                statements: [
                    Expression {
                        source: 0..19,
                        expression: Infix {
                            source: 2..5,
                            operator: Plus,
                            left: Integer {
                                source: 0..1,
                                value: "1",
                            },
                            right: Integer {
                                source: 4..5,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 19..38,
                        expression: Infix {
                            source: 21..24,
                            operator: Minus,
                            left: Integer {
                                source: 19..20,
                                value: "1",
                            },
                            right: Integer {
                                source: 23..24,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 38..57,
                        expression: Infix {
                            source: 40..43,
                            operator: Slash,
                            left: Integer {
                                source: 38..39,
                                value: "1",
                            },
                            right: Integer {
                                source: 42..43,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 57..76,
                        expression: Infix {
                            source: 59..62,
                            operator: Modulo,
                            left: Integer {
                                source: 57..58,
                                value: "3",
                            },
                            right: Integer {
                                source: 61..62,
                                value: "4",
                            },
                        },
                    },
                    Expression {
                        source: 76..106,
                        expression: Infix {
                            source: 83..92,
                            operator: Or,
                            left: Infix {
                                source: 78..83,
                                operator: Equal,
                                left: Integer {
                                    source: 76..77,
                                    value: "4",
                                },
                                right: Integer {
                                    source: 81..82,
                                    value: "5",
                                },
                            },
                            right: Infix {
                                source: 88..92,
                                operator: NotEqual,
                                left: Integer {
                                    source: 86..87,
                                    value: "4",
                                },
                                right: Integer {
                                    source: 91..92,
                                    value: "7",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 106..135,
                        expression: Infix {
                            source: 113..121,
                            operator: And,
                            left: Infix {
                                source: 108..113,
                                operator: GreaterThan,
                                left: Integer {
                                    source: 106..107,
                                    value: "5",
                                },
                                right: Integer {
                                    source: 110..112,
                                    value: "10",
                                },
                            },
                            right: Infix {
                                source: 118..121,
                                operator: LessThan,
                                left: Integer {
                                    source: 116..117,
                                    value: "4",
                                },
                                right: Integer {
                                    source: 120..121,
                                    value: "8",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 135..152,
                        expression: Infix {
                            source: 142..151,
                            operator: And,
                            left: Infix {
                                source: 137..142,
                                operator: GreaterThanEqual,
                                left: Integer {
                                    source: 135..136,
                                    value: "5",
                                },
                                right: Integer {
                                    source: 140..141,
                                    value: "3",
                                },
                            },
                            right: Infix {
                                source: 147..151,
                                operator: LessThanEqual,
                                left: Integer {
                                    source: 145..146,
                                    value: "4",
                                },
                                right: Integer {
                                    source: 150..151,
                                    value: "2",
                                },
                            },
                        },
                    },
                ],
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
                source: 0..78,
                statements: [
                    Expression {
                        source: 0..31,
                        expression: Function {
                            source: 0..17,
                            parameters: [
                                Identifier {
                                    source: 1..2,
                                    name: "x",
                                },
                                Identifier {
                                    source: 4..5,
                                    name: "y",
                                },
                            ],
                            body: Block {
                                source: 7..17,
                                statements: [
                                    Expression {
                                        source: 9..16,
                                        expression: Infix {
                                            source: 11..14,
                                            operator: Plus,
                                            left: Identifier {
                                                source: 9..10,
                                                name: "x",
                                            },
                                            right: Identifier {
                                                source: 13..14,
                                                name: "y",
                                            },
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    Expression {
                        source: 31..57,
                        expression: Function {
                            source: 31..57,
                            parameters: [
                                Identifier {
                                    source: 32..33,
                                    name: "x",
                                },
                                Identifier {
                                    source: 35..36,
                                    name: "y",
                                },
                            ],
                            body: Block {
                                source: 38..57,
                                statements: [
                                    Expression {
                                        source: 38..57,
                                        expression: Infix {
                                            source: 40..43,
                                            operator: Plus,
                                            left: Identifier {
                                                source: 38..39,
                                                name: "x",
                                            },
                                            right: Identifier {
                                                source: 42..43,
                                                name: "y",
                                            },
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    Expression {
                        source: 57..78,
                        expression: Function {
                            source: 57..78,
                            parameters: [
                                Identifier {
                                    source: 58..59,
                                    name: "x",
                                },
                                IdentifierListPattern {
                                    source: 61..62,
                                    pattern: [
                                        Identifier {
                                            source: 62..63,
                                            name: "y",
                                        },
                                        RestElement {
                                            source: 65..67,
                                            name: Identifier {
                                                source: 67..69,
                                                name: "ys",
                                            },
                                        },
                                    ],
                                },
                                RestElement {
                                    source: 72..74,
                                    name: Identifier {
                                        source: 74..75,
                                        name: "z",
                                    },
                                },
                            ],
                            body: Block {
                                source: 77..78,
                                statements: [
                                    Expression {
                                        source: 77..78,
                                        expression: Identifier {
                                            source: 77..78,
                                            name: "x",
                                        },
                                    },
                                ],
                            },
                        },
                    },
                ],
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
                source: 0..106,
                statements: [
                    Expression {
                        source: 0..23,
                        expression: Call {
                            source: 3..9,
                            function: Identifier {
                                source: 0..3,
                                name: "add",
                            },
                            arguments: [
                                Integer {
                                    source: 4..5,
                                    value: "1",
                                },
                                Integer {
                                    source: 7..8,
                                    value: "2",
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 23..50,
                        expression: Call {
                            source: 26..36,
                            function: Identifier {
                                source: 23..26,
                                name: "add",
                            },
                            arguments: [
                                Infix {
                                    source: 29..32,
                                    operator: Plus,
                                    left: Identifier {
                                        source: 27..28,
                                        name: "x",
                                    },
                                    right: Identifier {
                                        source: 31..32,
                                        name: "y",
                                    },
                                },
                                Integer {
                                    source: 34..35,
                                    value: "3",
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 50..73,
                        expression: Call {
                            source: 53..59,
                            function: Identifier {
                                source: 50..53,
                                name: "add",
                            },
                            arguments: [
                                SpreadElement {
                                    source: 54..56,
                                    value: Identifier {
                                        source: 56..58,
                                        name: "xs",
                                    },
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 73..96,
                        expression: Infix {
                            source: 75..82,
                            operator: Call(
                                Identifier {
                                    source: 75..80,
                                    name: "`add`",
                                },
                            ),
                            left: Integer {
                                source: 73..74,
                                value: "1",
                            },
                            right: Integer {
                                source: 81..82,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 96..106,
                        expression: Infix {
                            source: 98..105,
                            operator: Call(
                                Identifier {
                                    source: 98..103,
                                    name: "`add`",
                                },
                            ),
                            left: Identifier {
                                source: 96..97,
                                name: "x",
                            },
                            right: Identifier {
                                source: 104..105,
                                name: "y",
                            },
                        },
                    },
                ],
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
                source: 0..112,
                statements: [
                    Expression {
                        source: 0..19,
                        expression: Infix {
                            source: 2..5,
                            operator: Plus,
                            left: Placeholder {
                                source: 0..1,
                            },
                            right: Integer {
                                source: 4..5,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 19..38,
                        expression: Infix {
                            source: 21..24,
                            operator: Plus,
                            left: Integer {
                                source: 19..20,
                                value: "1",
                            },
                            right: Placeholder {
                                source: 23..24,
                            },
                        },
                    },
                    Expression {
                        source: 38..59,
                        expression: Call {
                            source: 39..45,
                            function: Identifier {
                                source: 38..39,
                                name: "+",
                            },
                            arguments: [
                                Placeholder {
                                    source: 40..41,
                                },
                                Integer {
                                    source: 43..44,
                                    value: "2",
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 59..79,
                        expression: Call {
                            source: 60..79,
                            function: Identifier {
                                source: 59..60,
                                name: "+",
                            },
                            arguments: [
                                Integer {
                                    source: 61..62,
                                    value: "1",
                                },
                                Placeholder {
                                    source: 64..65,
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 79..102,
                        expression: Infix {
                            source: 81..88,
                            operator: Call(
                                Identifier {
                                    source: 81..86,
                                    name: "`add`",
                                },
                            ),
                            left: Placeholder {
                                source: 79..80,
                            },
                            right: Integer {
                                source: 87..88,
                                value: "2",
                            },
                        },
                    },
                    Expression {
                        source: 102..112,
                        expression: Infix {
                            source: 104..111,
                            operator: Call(
                                Identifier {
                                    source: 104..109,
                                    name: "`add`",
                                },
                            ),
                            left: Integer {
                                source: 102..103,
                                value: "1",
                            },
                            right: Placeholder {
                                source: 110..111,
                            },
                        },
                    },
                ],
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
                source: 0..30,
                statements: [
                    Expression {
                        source: 0..30,
                        expression: Call {
                            source: 4..30,
                            function: Identifier {
                                source: 0..4,
                                name: "call",
                            },
                            arguments: [
                                Identifier {
                                    source: 5..6,
                                    name: "+",
                                },
                                Identifier {
                                    source: 8..9,
                                    name: "-",
                                },
                                Identifier {
                                    source: 11..12,
                                    name: "/",
                                },
                                Identifier {
                                    source: 14..15,
                                    name: "%",
                                },
                                Identifier {
                                    source: 17..18,
                                    name: ">",
                                },
                                Identifier {
                                    source: 20..21,
                                    name: "<",
                                },
                                Identifier {
                                    source: 23..25,
                                    name: ">=",
                                },
                                Identifier {
                                    source: 27..29,
                                    name: "<=",
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..22,
                statements: [
                    Expression {
                        source: 0..22,
                        expression: FunctionComposition {
                            source: 4..22,
                            functions: [
                                Identifier {
                                    source: 0..3,
                                    name: "inc",
                                },
                                Infix {
                                    source: 9..13,
                                    operator: Plus,
                                    left: Placeholder {
                                        source: 7..8,
                                    },
                                    right: Integer {
                                        source: 11..12,
                                        value: "1",
                                    },
                                },
                                Call {
                                    source: 19..22,
                                    function: Identifier {
                                        source: 16..19,
                                        name: "add",
                                    },
                                    arguments: [
                                        Integer {
                                            source: 20..21,
                                            value: "1",
                                        },
                                    ],
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..86,
                statements: [
                    Expression {
                        source: 0..41,
                        expression: FunctionThread {
                            source: 10..27,
                            initial: List {
                                source: 0..10,
                                elements: [
                                    Integer {
                                        source: 1..2,
                                        value: "1",
                                    },
                                    Integer {
                                        source: 4..5,
                                        value: "2",
                                    },
                                    Integer {
                                        source: 7..8,
                                        value: "3",
                                    },
                                ],
                            },
                            functions: [
                                Call {
                                    source: 16..27,
                                    function: Identifier {
                                        source: 13..16,
                                        name: "map",
                                    },
                                    arguments: [
                                        Function {
                                            source: 17..26,
                                            parameters: [
                                                Identifier {
                                                    source: 18..19,
                                                    name: "x",
                                                },
                                            ],
                                            body: Block {
                                                source: 21..26,
                                                statements: [
                                                    Expression {
                                                        source: 21..26,
                                                        expression: Infix {
                                                            source: 23..26,
                                                            operator: Plus,
                                                            left: Identifier {
                                                                source: 21..22,
                                                                name: "x",
                                                            },
                                                            right: Integer {
                                                                source: 25..26,
                                                                value: "1",
                                                            },
                                                        },
                                                    },
                                                ],
                                            },
                                        },
                                    ],
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 41..86,
                        expression: FunctionThread {
                            source: 43..85,
                            initial: Integer {
                                source: 41..42,
                                value: "1",
                            },
                            functions: [
                                Call {
                                    source: 49..53,
                                    function: Identifier {
                                        source: 46..49,
                                        name: "add",
                                    },
                                    arguments: [
                                        Integer {
                                            source: 50..51,
                                            value: "1",
                                        },
                                    ],
                                },
                                Function {
                                    source: 56..70,
                                    parameters: [
                                        Identifier {
                                            source: 57..58,
                                            name: "a",
                                        },
                                    ],
                                    body: Block {
                                        source: 60..70,
                                        statements: [
                                            Expression {
                                                source: 62..68,
                                                expression: Infix {
                                                    source: 64..68,
                                                    operator: Plus,
                                                    left: Identifier {
                                                        source: 62..63,
                                                        name: "a",
                                                    },
                                                    right: Integer {
                                                        source: 66..67,
                                                        value: "1",
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                },
                                Identifier {
                                    source: 73..76,
                                    name: "inc",
                                },
                                Infix {
                                    source: 82..85,
                                    operator: Plus,
                                    left: Placeholder {
                                        source: 80..81,
                                    },
                                    right: Integer {
                                        source: 84..85,
                                        value: "1",
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..66,
                statements: [
                    Expression {
                        source: 0..32,
                        expression: Call {
                            source: 5..18,
                            function: Identifier {
                                source: 0..4,
                                name: "with",
                            },
                            arguments: [
                                Identifier {
                                    source: 6..7,
                                    name: "x",
                                },
                                Function {
                                    source: 9..18,
                                    parameters: [
                                        Identifier {
                                            source: 10..11,
                                            name: "y",
                                        },
                                    ],
                                    body: Block {
                                        source: 13..18,
                                        statements: [
                                            Expression {
                                                source: 15..17,
                                                expression: Identifier {
                                                    source: 15..16,
                                                    name: "y",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                    Expression {
                        source: 32..66,
                        expression: FunctionThread {
                            source: 42..65,
                            initial: List {
                                source: 32..42,
                                elements: [
                                    Integer {
                                        source: 33..34,
                                        value: "1",
                                    },
                                    Integer {
                                        source: 36..37,
                                        value: "2",
                                    },
                                    Integer {
                                        source: 39..40,
                                        value: "3",
                                    },
                                ],
                            },
                            functions: [
                                Call {
                                    source: 45..65,
                                    function: Identifier {
                                        source: 45..49,
                                        name: "each",
                                    },
                                    arguments: [
                                        Function {
                                            source: 50..65,
                                            parameters: [
                                                Identifier {
                                                    source: 51..52,
                                                    name: "x",
                                                },
                                            ],
                                            body: Block {
                                                source: 54..65,
                                                statements: [
                                                    Expression {
                                                        source: 56..64,
                                                        expression: Call {
                                                            source: 60..64,
                                                            function: Identifier {
                                                                source: 56..60,
                                                                name: "puts",
                                                            },
                                                            arguments: [
                                                                Identifier {
                                                                    source: 61..62,
                                                                    name: "x",
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ],
                                            },
                                        },
                                    ],
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..171,
                statements: [
                    Expression {
                        source: 0..20,
                        expression: Index {
                            source: 3..6,
                            left: Identifier {
                                source: 0..3,
                                name: "col",
                            },
                            index: Integer {
                                source: 4..5,
                                value: "1",
                            },
                        },
                    },
                    Expression {
                        source: 20..41,
                        expression: Index {
                            source: 23..27,
                            left: Identifier {
                                source: 20..23,
                                name: "col",
                            },
                            index: Prefix {
                                source: 24..26,
                                operator: Minus,
                                right: Integer {
                                    source: 25..26,
                                    value: "1",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 41..65,
                        expression: Index {
                            source: 44..51,
                            left: Identifier {
                                source: 41..44,
                                name: "col",
                            },
                            index: InclusiveRange {
                                source: 46..50,
                                from: Integer {
                                    source: 45..46,
                                    value: "2",
                                },
                                to: Integer {
                                    source: 49..50,
                                    value: "5",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 65..88,
                        expression: Index {
                            source: 68..74,
                            left: Identifier {
                                source: 65..68,
                                name: "col",
                            },
                            index: ExclusiveRange {
                                source: 70..73,
                                from: Integer {
                                    source: 69..70,
                                    value: "2",
                                },
                                until: Integer {
                                    source: 72..73,
                                    value: "5",
                                },
                            },
                        },
                    },
                    Expression {
                        source: 88..111,
                        expression: Index {
                            source: 91..97,
                            left: Identifier {
                                source: 88..91,
                                name: "col",
                            },
                            index: UnboundedRange {
                                source: 94..96,
                                from: Prefix {
                                    source: 92..94,
                                    operator: Minus,
                                    right: Integer {
                                        source: 93..94,
                                        value: "2",
                                    },
                                },
                            },
                        },
                    },
                    Expression {
                        source: 111..136,
                        expression: Index {
                            source: 114..122,
                            left: Identifier {
                                source: 111..114,
                                name: "col",
                            },
                            index: InclusiveRange {
                                source: 116..121,
                                from: Integer {
                                    source: 115..116,
                                    value: "0",
                                },
                                to: Prefix {
                                    source: 119..121,
                                    operator: Minus,
                                    right: Integer {
                                        source: 120..121,
                                        value: "2",
                                    },
                                },
                            },
                        },
                    },
                    Expression {
                        source: 136..160,
                        expression: Index {
                            source: 139..146,
                            left: Identifier {
                                source: 136..139,
                                name: "col",
                            },
                            index: ExclusiveRange {
                                source: 141..145,
                                from: Integer {
                                    source: 140..141,
                                    value: "0",
                                },
                                until: Prefix {
                                    source: 143..145,
                                    operator: Minus,
                                    right: Integer {
                                        source: 144..145,
                                        value: "2",
                                    },
                                },
                            },
                        },
                    },
                    Expression {
                        source: 160..171,
                        expression: Index {
                            source: 163..170,
                            left: Identifier {
                                source: 160..163,
                                name: "col",
                            },
                            index: String {
                                source: 164..169,
                                value: "\"key\"",
                            },
                        },
                    },
                ],
            }"#]],
    );
}

#[test]
fn sections() {
    assert_ast(
        r#"
            section_one: { "sample "};
            section_two: "sample";
            section_three: {
                section_four: "sample";
            };
        "#,
        expect![[r#"
            Program {
                source: 0..145,
                statements: [
                    Section {
                        source: 0..39,
                        name: Identifier {
                            source: 0..11,
                            name: "section_one",
                        },
                        body: Block {
                            source: 13..25,
                            statements: [
                                Expression {
                                    source: 15..24,
                                    expression: String {
                                        source: 15..24,
                                        value: "\"sample \"",
                                    },
                                },
                            ],
                        },
                    },
                    Section {
                        source: 39..74,
                        name: Identifier {
                            source: 39..50,
                            name: "section_two",
                        },
                        body: Block {
                            source: 52..74,
                            statements: [
                                Expression {
                                    source: 52..74,
                                    expression: String {
                                        source: 52..60,
                                        value: "\"sample\"",
                                    },
                                },
                            ],
                        },
                    },
                    Section {
                        source: 74..145,
                        name: Identifier {
                            source: 74..87,
                            name: "section_three",
                        },
                        body: Block {
                            source: 89..144,
                            statements: [
                                Section {
                                    source: 107..143,
                                    name: Identifier {
                                        source: 107..119,
                                        name: "section_four",
                                    },
                                    body: Block {
                                        source: 121..143,
                                        statements: [
                                            Expression {
                                                source: 121..143,
                                                expression: String {
                                                    source: 121..129,
                                                    value: "\"sample\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..146,
                statements: [
                    Expression {
                        source: 0..146,
                        expression: Match {
                            source: 0..146,
                            subject: Identifier {
                                source: 6..7,
                                name: "x",
                            },
                            cases: [
                                Unguarded {
                                    pattern: Integer {
                                        source: 26..27,
                                        value: "1",
                                    },
                                    consequence: Block {
                                        source: 28..37,
                                        statements: [
                                            Expression {
                                                source: 30..36,
                                                expression: String {
                                                    source: 30..35,
                                                    value: "\"one\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: Decimal {
                                        source: 55..58,
                                        value: "2.0",
                                    },
                                    consequence: Block {
                                        source: 59..68,
                                        statements: [
                                            Expression {
                                                source: 61..67,
                                                expression: String {
                                                    source: 61..66,
                                                    value: "\"two\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: Boolean {
                                        source: 86..90,
                                        value: true,
                                    },
                                    consequence: Block {
                                        source: 91..102,
                                        statements: [
                                            Expression {
                                                source: 93..101,
                                                expression: String {
                                                    source: 93..100,
                                                    value: "\"three\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: String {
                                        source: 120..126,
                                        value: "\"four\"",
                                    },
                                    consequence: Block {
                                        source: 127..145,
                                        statements: [
                                            Expression {
                                                source: 129..131,
                                                expression: Integer {
                                                    source: 129..130,
                                                    value: "4",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..119,
                statements: [
                    Expression {
                        source: 0..119,
                        expression: Match {
                            source: 0..119,
                            subject: Identifier {
                                source: 6..7,
                                name: "x",
                            },
                            cases: [
                                Unguarded {
                                    pattern: ExclusiveRange {
                                        source: 27..31,
                                        from: Integer {
                                            source: 26..27,
                                            value: "1",
                                        },
                                        until: Integer {
                                            source: 29..30,
                                            value: "3",
                                        },
                                    },
                                    consequence: Block {
                                        source: 31..40,
                                        statements: [
                                            Expression {
                                                source: 33..39,
                                                expression: String {
                                                    source: 33..38,
                                                    value: "\"1-2\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: InclusiveRange {
                                        source: 59..64,
                                        from: Integer {
                                            source: 58..59,
                                            value: "3",
                                        },
                                        to: Integer {
                                            source: 62..63,
                                            value: "5",
                                        },
                                    },
                                    consequence: Block {
                                        source: 64..73,
                                        statements: [
                                            Expression {
                                                source: 66..72,
                                                expression: String {
                                                    source: 66..71,
                                                    value: "\"3-5\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: UnboundedRange {
                                        source: 92..95,
                                        from: Integer {
                                            source: 91..92,
                                            value: "6",
                                        },
                                    },
                                    consequence: Block {
                                        source: 95..118,
                                        statements: [
                                            Expression {
                                                source: 97..104,
                                                expression: String {
                                                    source: 97..103,
                                                    value: "\">= 6\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..163,
                statements: [
                    Expression {
                        source: 0..163,
                        expression: Match {
                            source: 0..163,
                            subject: Identifier {
                                source: 6..7,
                                name: "x",
                            },
                            cases: [
                                Guarded {
                                    pattern: Integer {
                                        source: 26..27,
                                        value: "1",
                                    },
                                    guard: Infix {
                                        source: 33..38,
                                        operator: NotEqual,
                                        left: Integer {
                                            source: 31..32,
                                            value: "1",
                                        },
                                        right: Integer {
                                            source: 36..37,
                                            value: "2",
                                        },
                                    },
                                    consequence: Block {
                                        source: 38..47,
                                        statements: [
                                            Expression {
                                                source: 40..46,
                                                expression: String {
                                                    source: 40..45,
                                                    value: "\"one\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Guarded {
                                    pattern: Decimal {
                                        source: 65..68,
                                        value: "2.0",
                                    },
                                    guard: Infix {
                                        source: 77..87,
                                        operator: And,
                                        left: Boolean {
                                            source: 72..76,
                                            value: true,
                                        },
                                        right: Prefix {
                                            source: 80..87,
                                            operator: Bang,
                                            right: Boolean {
                                                source: 81..86,
                                                value: false,
                                            },
                                        },
                                    },
                                    consequence: Block {
                                        source: 87..96,
                                        statements: [
                                            Expression {
                                                source: 89..95,
                                                expression: String {
                                                    source: 89..94,
                                                    value: "\"two\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Guarded {
                                    pattern: Identifier {
                                        source: 114..115,
                                        name: "e",
                                    },
                                    guard: Infix {
                                        source: 121..125,
                                        operator: GreaterThan,
                                        left: Identifier {
                                            source: 119..120,
                                            name: "e",
                                        },
                                        right: Integer {
                                            source: 123..124,
                                            value: "3",
                                        },
                                    },
                                    consequence: Block {
                                        source: 125..162,
                                        statements: [
                                            Expression {
                                                source: 127..148,
                                                expression: String {
                                                    source: 127..147,
                                                    value: "\"greater than three\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                source: 0..168,
                statements: [
                    Expression {
                        source: 0..168,
                        expression: Match {
                            source: 0..168,
                            subject: Identifier {
                                source: 6..7,
                                name: "x",
                            },
                            cases: [
                                Unguarded {
                                    pattern: ListMatchPattern {
                                        source: 26..27,
                                        pattern: [],
                                    },
                                    consequence: Block {
                                        source: 29..40,
                                        statements: [
                                            Expression {
                                                source: 31..39,
                                                expression: String {
                                                    source: 31..38,
                                                    value: "\"empty\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: ListMatchPattern {
                                        source: 58..62,
                                        pattern: [
                                            Identifier {
                                                source: 59..60,
                                                name: "x",
                                            },
                                        ],
                                    },
                                    consequence: Block {
                                        source: 62..76,
                                        statements: [
                                            Expression {
                                                source: 64..75,
                                                expression: String {
                                                    source: 64..74,
                                                    value: "\"one item\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: ListMatchPattern {
                                        source: 94..104,
                                        pattern: [
                                            Identifier {
                                                source: 95..96,
                                                name: "x",
                                            },
                                            RestElement {
                                                source: 98..102,
                                                name: Identifier {
                                                    source: 100..102,
                                                    name: "xs",
                                                },
                                            },
                                        ],
                                    },
                                    consequence: Block {
                                        source: 104..120,
                                        statements: [
                                            Expression {
                                                source: 106..119,
                                                expression: String {
                                                    source: 106..118,
                                                    value: "\"many items\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                                Unguarded {
                                    pattern: Placeholder {
                                        source: 138..139,
                                    },
                                    consequence: Block {
                                        source: 140..154,
                                        statements: [
                                            Expression {
                                                source: 142..152,
                                                expression: String {
                                                    source: 142..151,
                                                    value: "\"unknown\"",
                                                },
                                            },
                                        ],
                                    },
                                },
                            ],
                        },
                    },
                ],
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
                message: "Illegal token",
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

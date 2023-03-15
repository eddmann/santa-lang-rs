use super::*;

use expect_test::{expect, Expect};

#[test]
fn integer() {
    assert_tokens(
        "1",
        expect![[r#"
        [
            "Token { kind: Integer, location: 0..1 }",
        ]"#]],
    )
}

#[test]
fn integer_with_seperators() {
    assert_tokens(
        "1_000_000",
        expect![[r#"
        [
            "Token { kind: Integer, location: 0..9 }",
        ]"#]],
    )
}

#[test]
fn decimal() {
    assert_tokens(
        "1.5",
        expect![[r#"
        [
            "Token { kind: Decimal, location: 0..3 }",
        ]"#]],
    )
}

#[test]
fn decimal_with_seperators() {
    assert_tokens(
        "1_000_000.50",
        expect![[r#"
        [
            "Token { kind: Decimal, location: 0..12 }",
        ]"#]],
    )
}

#[test]
fn string() {
    assert_tokens(
        "\"Hello, world!\"",
        expect![[r#"
        [
            "Token { kind: String, location: 0..15 }",
        ]"#]],
    )
}

#[test]
fn string_with_escaped_characters() {
    assert_tokens(
        r#""\\r \\n \\t \\\\ \"""#,
        expect![[r#"
        [
            "Token { kind: String, location: 0..21 }",
        ]"#]],
    )
}

#[test]
fn string_with_multibyte_unicode_characters() {
    assert_tokens(
        "\"µࠒ𒀀\"",
        expect![[r#"
        [
            "Token { kind: String, location: 0..11 }",
        ]"#]],
    )
}

#[test]
fn backticks() {
    assert_tokens(
        "
            `sample`;
            1 `add` 2;
        ",
        expect![[r#"
            [
                "Token { kind: Backtick, location: 0..8 }",
                "Token { kind: Semicolon, location: 8..9 }",
                "Token { kind: Integer, location: 22..23 }",
                "Token { kind: Backtick, location: 24..29 }",
                "Token { kind: Integer, location: 30..31 }",
                "Token { kind: Semicolon, location: 31..32 }",
            ]"#]],
    );
}

#[test]
fn comments() {
    assert_tokens(
        "
            // full line comment
            1; // end of line comment
        //",
        expect![[r#"
            [
                "Token { kind: Comment, location: 0..20 }",
                "Token { kind: Integer, location: 33..34 }",
                "Token { kind: Semicolon, location: 34..35 }",
                "Token { kind: Comment, location: 36..58 }",
                "Token { kind: Comment, location: 67..69 }",
            ]"#]],
    );
}

#[test]
fn keywords() {
    assert_tokens(
        "
            mut
            match
            let
            if
            else
            return
            break
            nil
            true
            false
        ",
        expect![[r#"
            [
                "Token { kind: Mutable, location: 0..3 }",
                "Token { kind: Match, location: 16..21 }",
                "Token { kind: Let, location: 34..37 }",
                "Token { kind: If, location: 50..52 }",
                "Token { kind: Else, location: 65..69 }",
                "Token { kind: Return, location: 82..88 }",
                "Token { kind: Break, location: 101..106 }",
                "Token { kind: Nil, location: 119..122 }",
                "Token { kind: True, location: 135..139 }",
                "Token { kind: False, location: 152..157 }",
            ]"#]],
    );
}

#[test]
fn identifiers() {
    assert_tokens(
        "
            sample
            sample_id
            sample1234
            sample?
            SAMPLE
        ",
        expect![[r#"
            [
                "Token { kind: Identifier, location: 0..6 }",
                "Token { kind: Identifier, location: 19..28 }",
                "Token { kind: Identifier, location: 41..51 }",
                "Token { kind: Identifier, location: 64..71 }",
                "Token { kind: Identifier, location: 84..90 }",
            ]"#]],
    );
}

#[test]
fn list() {
    assert_tokens(
        r#"[1, 2.0, true, "hello"]"#,
        expect![[r#"
        [
            "Token { kind: LBracket, location: 0..1 }",
            "Token { kind: Integer, location: 1..2 }",
            "Token { kind: Comma, location: 2..3 }",
            "Token { kind: Decimal, location: 4..7 }",
            "Token { kind: Comma, location: 7..8 }",
            "Token { kind: True, location: 9..13 }",
            "Token { kind: Comma, location: 13..14 }",
            "Token { kind: String, location: 15..22 }",
            "Token { kind: RBracket, location: 22..23 }",
        ]"#]],
    );
}

#[test]
fn hash() {
    assert_tokens(
        r#"#{2.0: "hello", 1: true}"#,
        expect![[r#"
        [
            "Token { kind: HashLBrace, location: 0..2 }",
            "Token { kind: Decimal, location: 2..5 }",
            "Token { kind: Colon, location: 5..6 }",
            "Token { kind: String, location: 7..14 }",
            "Token { kind: Comma, location: 14..15 }",
            "Token { kind: Integer, location: 16..17 }",
            "Token { kind: Colon, location: 17..18 }",
            "Token { kind: True, location: 19..23 }",
            "Token { kind: RBrace, location: 23..24 }",
        ]"#]],
    );
}

#[test]
fn set() {
    assert_tokens(
        r#"{2.0, "hello", 1, true}"#,
        expect![[r#"
        [
            "Token { kind: LBrace, location: 0..1 }",
            "Token { kind: Decimal, location: 1..4 }",
            "Token { kind: Comma, location: 4..5 }",
            "Token { kind: String, location: 6..13 }",
            "Token { kind: Comma, location: 13..14 }",
            "Token { kind: Integer, location: 15..16 }",
            "Token { kind: Comma, location: 16..17 }",
            "Token { kind: True, location: 18..22 }",
            "Token { kind: RBrace, location: 22..23 }",
        ]"#]],
    );
}

#[test]
fn ranges() {
    assert_tokens(
        "1..10; 1..=10; 1..;",
        expect![[r#"
        [
            "Token { kind: Integer, location: 0..1 }",
            "Token { kind: DotDot, location: 1..3 }",
            "Token { kind: Integer, location: 3..5 }",
            "Token { kind: Semicolon, location: 5..6 }",
            "Token { kind: Integer, location: 7..8 }",
            "Token { kind: DotDotEqual, location: 8..11 }",
            "Token { kind: Integer, location: 11..13 }",
            "Token { kind: Semicolon, location: 13..14 }",
            "Token { kind: Integer, location: 15..16 }",
            "Token { kind: DotDot, location: 16..18 }",
            "Token { kind: Semicolon, location: 18..19 }",
        ]"#]],
    );
}

#[test]
fn symbols() {
    assert_tokens(
        "
            +-*/
            ! =
            == != > < >= <=
            && ||
            |> >>
            .. ..=
            _ () {} ;
        ",
        expect![[r#"
            [
                "Token { kind: Plus, location: 0..1 }",
                "Token { kind: Minus, location: 1..2 }",
                "Token { kind: Asterisk, location: 2..3 }",
                "Token { kind: Slash, location: 3..4 }",
                "Token { kind: Bang, location: 17..18 }",
                "Token { kind: Assign, location: 19..20 }",
                "Token { kind: Equal, location: 33..35 }",
                "Token { kind: NotEqual, location: 36..38 }",
                "Token { kind: GreaterThan, location: 39..40 }",
                "Token { kind: LessThan, location: 41..42 }",
                "Token { kind: GreaterThanEqual, location: 43..45 }",
                "Token { kind: LessThanEqual, location: 46..48 }",
                "Token { kind: AmpAmp, location: 61..63 }",
                "Token { kind: PipePipe, location: 64..66 }",
                "Token { kind: PipeGreater, location: 79..81 }",
                "Token { kind: GreaterGreater, location: 82..84 }",
                "Token { kind: DotDot, location: 97..99 }",
                "Token { kind: DotDotEqual, location: 100..103 }",
                "Token { kind: Underscore, location: 116..117 }",
                "Token { kind: LParen, location: 118..119 }",
                "Token { kind: RParen, location: 119..120 }",
                "Token { kind: LBrace, location: 121..122 }",
                "Token { kind: RBrace, location: 122..123 }",
                "Token { kind: Semicolon, location: 124..125 }",
            ]"#]],
    );
}

#[test]
fn script() {
    assert_tokens(
        "
            let fibonacci = |n| {
                let recur = |x, y, n| {
                    if n > 0 { return recur(y, x + y, n - 1) } else { x }
                };
                recur(0, 1, n);
            };
            fibonacci(90);
        ",
        expect![[r#"
            [
                "Token { kind: Let, location: 0..3 }",
                "Token { kind: Identifier, location: 4..13 }",
                "Token { kind: Assign, location: 14..15 }",
                "Token { kind: Pipe, location: 16..17 }",
                "Token { kind: Identifier, location: 17..18 }",
                "Token { kind: Pipe, location: 18..19 }",
                "Token { kind: LBrace, location: 20..21 }",
                "Token { kind: Let, location: 38..41 }",
                "Token { kind: Identifier, location: 42..47 }",
                "Token { kind: Assign, location: 48..49 }",
                "Token { kind: Pipe, location: 50..51 }",
                "Token { kind: Identifier, location: 51..52 }",
                "Token { kind: Comma, location: 52..53 }",
                "Token { kind: Identifier, location: 54..55 }",
                "Token { kind: Comma, location: 55..56 }",
                "Token { kind: Identifier, location: 57..58 }",
                "Token { kind: Pipe, location: 58..59 }",
                "Token { kind: LBrace, location: 60..61 }",
                "Token { kind: If, location: 82..84 }",
                "Token { kind: Identifier, location: 85..86 }",
                "Token { kind: GreaterThan, location: 87..88 }",
                "Token { kind: Integer, location: 89..90 }",
                "Token { kind: LBrace, location: 91..92 }",
                "Token { kind: Return, location: 93..99 }",
                "Token { kind: Identifier, location: 100..105 }",
                "Token { kind: LParen, location: 105..106 }",
                "Token { kind: Identifier, location: 106..107 }",
                "Token { kind: Comma, location: 107..108 }",
                "Token { kind: Identifier, location: 109..110 }",
                "Token { kind: Plus, location: 111..112 }",
                "Token { kind: Identifier, location: 113..114 }",
                "Token { kind: Comma, location: 114..115 }",
                "Token { kind: Identifier, location: 116..117 }",
                "Token { kind: Minus, location: 118..119 }",
                "Token { kind: Integer, location: 120..121 }",
                "Token { kind: RParen, location: 121..122 }",
                "Token { kind: RBrace, location: 123..124 }",
                "Token { kind: Else, location: 125..129 }",
                "Token { kind: LBrace, location: 130..131 }",
                "Token { kind: Identifier, location: 132..133 }",
                "Token { kind: RBrace, location: 134..135 }",
                "Token { kind: RBrace, location: 152..153 }",
                "Token { kind: Semicolon, location: 153..154 }",
                "Token { kind: Identifier, location: 171..176 }",
                "Token { kind: LParen, location: 176..177 }",
                "Token { kind: Integer, location: 177..178 }",
                "Token { kind: Comma, location: 178..179 }",
                "Token { kind: Integer, location: 180..181 }",
                "Token { kind: Comma, location: 181..182 }",
                "Token { kind: Identifier, location: 183..184 }",
                "Token { kind: RParen, location: 184..185 }",
                "Token { kind: Semicolon, location: 185..186 }",
                "Token { kind: RBrace, location: 199..200 }",
                "Token { kind: Semicolon, location: 200..201 }",
                "Token { kind: Identifier, location: 214..223 }",
                "Token { kind: LParen, location: 223..224 }",
                "Token { kind: Integer, location: 224..226 }",
                "Token { kind: RParen, location: 226..227 }",
                "Token { kind: Semicolon, location: 227..228 }",
            ]"#]],
    );
}

fn assert_tokens(input: &str, expected: Expect) {
    let tokens: Vec<String> = Lexer::new(input.trim()).map(|token| format!("{:?}", token)).collect();
    let actual = format!("{:#?}", tokens);
    expected.assert_eq(&actual)
}

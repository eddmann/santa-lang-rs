use super::*;

use expect_test::{expect, Expect};

#[test]
fn integer() {
    assert_tokens(
        "1",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..1 }",
            ]"#]],
    )
}

#[test]
fn integer_with_seperators() {
    assert_tokens(
        "1_000_000",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..9 }",
            ]"#]],
    )
}

#[test]
fn decimal() {
    assert_tokens(
        "1.5",
        expect![[r#"
            [
                "Token { kind: Decimal, source: 0..3 }",
            ]"#]],
    )
}

#[test]
fn decimal_with_seperators() {
    assert_tokens(
        "1_000_000.50",
        expect![[r#"
            [
                "Token { kind: Decimal, source: 0..12 }",
            ]"#]],
    )
}

#[test]
fn string() {
    assert_tokens(
        "\"Hello, world!\"",
        expect![[r#"
            [
                "Token { kind: String, source: 0..15 }",
            ]"#]],
    )
}

#[test]
fn string_with_escaped_characters() {
    assert_tokens(
        r#""\\r \\n \\t \\\\ \"""#,
        expect![[r#"
            [
                "Token { kind: String, source: 0..21 }",
            ]"#]],
    )
}

#[test]
fn string_with_multibyte_unicode_characters() {
    assert_tokens(
        "\"µࠒ𒀀\"",
        expect![[r#"
            [
                "Token { kind: String, source: 0..11 }",
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
                "Token { kind: Backtick, source: 0..8 }",
                "Token { kind: Semicolon, source: 8..9 }",
                "Token { kind: Integer, source: 22..23 }",
                "Token { kind: Backtick, source: 24..29 }",
                "Token { kind: Integer, source: 30..31 }",
                "Token { kind: Semicolon, source: 31..32 }",
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
                "Token { kind: Comment, source: 0..20 }",
                "Token { kind: Integer, source: 33..34 }",
                "Token { kind: Semicolon, source: 34..35 }",
                "Token { kind: Comment, source: 36..58 }",
                "Token { kind: Comment, source: 67..69 }",
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
                "Token { kind: Mutable, source: 0..3 }",
                "Token { kind: Match, source: 16..21 }",
                "Token { kind: Let, source: 34..37 }",
                "Token { kind: If, source: 50..52 }",
                "Token { kind: Else, source: 65..69 }",
                "Token { kind: Return, source: 82..88 }",
                "Token { kind: Break, source: 101..106 }",
                "Token { kind: Nil, source: 119..122 }",
                "Token { kind: True, source: 135..139 }",
                "Token { kind: False, source: 152..157 }",
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
                "Token { kind: Identifier, source: 0..6 }",
                "Token { kind: Identifier, source: 19..28 }",
                "Token { kind: Identifier, source: 41..51 }",
                "Token { kind: Identifier, source: 64..71 }",
                "Token { kind: Identifier, source: 84..90 }",
            ]"#]],
    );
}

#[test]
fn list() {
    assert_tokens(
        r#"[1, 2.0, true, "hello"]"#,
        expect![[r#"
            [
                "Token { kind: LBracket, source: 0..1 }",
                "Token { kind: Integer, source: 1..2 }",
                "Token { kind: Comma, source: 2..3 }",
                "Token { kind: Decimal, source: 4..7 }",
                "Token { kind: Comma, source: 7..8 }",
                "Token { kind: True, source: 9..13 }",
                "Token { kind: Comma, source: 13..14 }",
                "Token { kind: String, source: 15..22 }",
                "Token { kind: RBracket, source: 22..23 }",
            ]"#]],
    );
}

#[test]
fn hash() {
    assert_tokens(
        r#"#{2.0: "hello", 1: true}"#,
        expect![[r#"
            [
                "Token { kind: HashLBrace, source: 0..2 }",
                "Token { kind: Decimal, source: 2..5 }",
                "Token { kind: Colon, source: 5..6 }",
                "Token { kind: String, source: 7..14 }",
                "Token { kind: Comma, source: 14..15 }",
                "Token { kind: Integer, source: 16..17 }",
                "Token { kind: Colon, source: 17..18 }",
                "Token { kind: True, source: 19..23 }",
                "Token { kind: RBrace, source: 23..24 }",
            ]"#]],
    );
}

#[test]
fn set() {
    assert_tokens(
        r#"{2.0, "hello", 1, true}"#,
        expect![[r#"
            [
                "Token { kind: LBrace, source: 0..1 }",
                "Token { kind: Decimal, source: 1..4 }",
                "Token { kind: Comma, source: 4..5 }",
                "Token { kind: String, source: 6..13 }",
                "Token { kind: Comma, source: 13..14 }",
                "Token { kind: Integer, source: 15..16 }",
                "Token { kind: Comma, source: 16..17 }",
                "Token { kind: True, source: 18..22 }",
                "Token { kind: RBrace, source: 22..23 }",
            ]"#]],
    );
}

#[test]
fn ranges() {
    assert_tokens(
        "1..10; 1..=10; 1..;",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..1 }",
                "Token { kind: DotDot, source: 1..3 }",
                "Token { kind: Integer, source: 3..5 }",
                "Token { kind: Semicolon, source: 5..6 }",
                "Token { kind: Integer, source: 7..8 }",
                "Token { kind: DotDotEqual, source: 8..11 }",
                "Token { kind: Integer, source: 11..13 }",
                "Token { kind: Semicolon, source: 13..14 }",
                "Token { kind: Integer, source: 15..16 }",
                "Token { kind: DotDot, source: 16..18 }",
                "Token { kind: Semicolon, source: 18..19 }",
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
                "Token { kind: Plus, source: 0..1 }",
                "Token { kind: Minus, source: 1..2 }",
                "Token { kind: Asterisk, source: 2..3 }",
                "Token { kind: Slash, source: 3..4 }",
                "Token { kind: Bang, source: 17..18 }",
                "Token { kind: Assign, source: 19..20 }",
                "Token { kind: Equal, source: 33..35 }",
                "Token { kind: NotEqual, source: 36..38 }",
                "Token { kind: GreaterThan, source: 39..40 }",
                "Token { kind: LessThan, source: 41..42 }",
                "Token { kind: GreaterThanEqual, source: 43..45 }",
                "Token { kind: LessThanEqual, source: 46..48 }",
                "Token { kind: AmpAmp, source: 61..63 }",
                "Token { kind: PipePipe, source: 64..66 }",
                "Token { kind: PipeGreater, source: 79..81 }",
                "Token { kind: GreaterGreater, source: 82..84 }",
                "Token { kind: DotDot, source: 97..99 }",
                "Token { kind: DotDotEqual, source: 100..103 }",
                "Token { kind: Underscore, source: 116..117 }",
                "Token { kind: LParen, source: 118..119 }",
                "Token { kind: RParen, source: 119..120 }",
                "Token { kind: LBrace, source: 121..122 }",
                "Token { kind: RBrace, source: 122..123 }",
                "Token { kind: Semicolon, source: 124..125 }",
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
                "Token { kind: Let, source: 0..3 }",
                "Token { kind: Identifier, source: 4..13 }",
                "Token { kind: Assign, source: 14..15 }",
                "Token { kind: Pipe, source: 16..17 }",
                "Token { kind: Identifier, source: 17..18 }",
                "Token { kind: Pipe, source: 18..19 }",
                "Token { kind: LBrace, source: 20..21 }",
                "Token { kind: Let, source: 38..41 }",
                "Token { kind: Identifier, source: 42..47 }",
                "Token { kind: Assign, source: 48..49 }",
                "Token { kind: Pipe, source: 50..51 }",
                "Token { kind: Identifier, source: 51..52 }",
                "Token { kind: Comma, source: 52..53 }",
                "Token { kind: Identifier, source: 54..55 }",
                "Token { kind: Comma, source: 55..56 }",
                "Token { kind: Identifier, source: 57..58 }",
                "Token { kind: Pipe, source: 58..59 }",
                "Token { kind: LBrace, source: 60..61 }",
                "Token { kind: If, source: 82..84 }",
                "Token { kind: Identifier, source: 85..86 }",
                "Token { kind: GreaterThan, source: 87..88 }",
                "Token { kind: Integer, source: 89..90 }",
                "Token { kind: LBrace, source: 91..92 }",
                "Token { kind: Return, source: 93..99 }",
                "Token { kind: Identifier, source: 100..105 }",
                "Token { kind: LParen, source: 105..106 }",
                "Token { kind: Identifier, source: 106..107 }",
                "Token { kind: Comma, source: 107..108 }",
                "Token { kind: Identifier, source: 109..110 }",
                "Token { kind: Plus, source: 111..112 }",
                "Token { kind: Identifier, source: 113..114 }",
                "Token { kind: Comma, source: 114..115 }",
                "Token { kind: Identifier, source: 116..117 }",
                "Token { kind: Minus, source: 118..119 }",
                "Token { kind: Integer, source: 120..121 }",
                "Token { kind: RParen, source: 121..122 }",
                "Token { kind: RBrace, source: 123..124 }",
                "Token { kind: Else, source: 125..129 }",
                "Token { kind: LBrace, source: 130..131 }",
                "Token { kind: Identifier, source: 132..133 }",
                "Token { kind: RBrace, source: 134..135 }",
                "Token { kind: RBrace, source: 152..153 }",
                "Token { kind: Semicolon, source: 153..154 }",
                "Token { kind: Identifier, source: 171..176 }",
                "Token { kind: LParen, source: 176..177 }",
                "Token { kind: Integer, source: 177..178 }",
                "Token { kind: Comma, source: 178..179 }",
                "Token { kind: Integer, source: 180..181 }",
                "Token { kind: Comma, source: 181..182 }",
                "Token { kind: Identifier, source: 183..184 }",
                "Token { kind: RParen, source: 184..185 }",
                "Token { kind: Semicolon, source: 185..186 }",
                "Token { kind: RBrace, source: 199..200 }",
                "Token { kind: Semicolon, source: 200..201 }",
                "Token { kind: Identifier, source: 214..223 }",
                "Token { kind: LParen, source: 223..224 }",
                "Token { kind: Integer, source: 224..226 }",
                "Token { kind: RParen, source: 226..227 }",
                "Token { kind: Semicolon, source: 227..228 }",
            ]"#]],
    );
}

fn assert_tokens(input: &str, expected: Expect) {
    let tokens: Vec<String> = Lexer::new(input.trim()).map(|token| format!("{:?}", token)).collect();
    let actual = format!("{:#?}", tokens);
    expected.assert_eq(&actual)
}

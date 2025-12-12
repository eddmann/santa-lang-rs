use super::*;

use expect_test::{Expect, expect};

#[test]
fn integer() {
    assert_tokens(
        "1",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..1, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn integer_with_seperators() {
    assert_tokens(
        "1_000_000",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..9, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn decimal() {
    assert_tokens(
        "1.5",
        expect![[r#"
            [
                "Token { kind: Decimal, source: 0..3, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn decimal_with_seperators() {
    assert_tokens(
        "1_000_000.50",
        expect![[r#"
            [
                "Token { kind: Decimal, source: 0..12, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn string() {
    assert_tokens(
        "\"Hello, world!\"",
        expect![[r#"
            [
                "Token { kind: String, source: 0..15, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn string_with_escaped_characters() {
    assert_tokens(
        r#""\\r \\n \\t \\\\ \"""#,
        expect![[r#"
            [
                "Token { kind: String, source: 0..21, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    )
}

#[test]
fn string_with_multibyte_unicode_characters() {
    assert_tokens(
        "\"µࠒ𒀀\"",
        expect![[r#"
            [
                "Token { kind: String, source: 0..11, line: 1, preceded_by_blank_line: false }",
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
                "Token { kind: Backtick, source: 0..8, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 8..9, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 22..23, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Backtick, source: 24..29, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 30..31, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 31..32, line: 2, preceded_by_blank_line: false }",
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
                "Token { kind: Comment, source: 0..20, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 33..34, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 34..35, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Comment, source: 36..58, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Comment, source: 67..69, line: 3, preceded_by_blank_line: false }",
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
                "Token { kind: Mutable, source: 0..3, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Match, source: 16..21, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Let, source: 34..37, line: 3, preceded_by_blank_line: false }",
                "Token { kind: If, source: 50..52, line: 4, preceded_by_blank_line: false }",
                "Token { kind: Else, source: 65..69, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Return, source: 82..88, line: 6, preceded_by_blank_line: false }",
                "Token { kind: Break, source: 101..106, line: 7, preceded_by_blank_line: false }",
                "Token { kind: Nil, source: 119..122, line: 8, preceded_by_blank_line: false }",
                "Token { kind: True, source: 135..139, line: 9, preceded_by_blank_line: false }",
                "Token { kind: False, source: 152..157, line: 10, preceded_by_blank_line: false }",
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
                "Token { kind: Identifier, source: 0..6, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 19..28, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 41..51, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 64..71, line: 4, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 84..90, line: 5, preceded_by_blank_line: false }",
            ]"#]],
    );
}

#[test]
fn list() {
    assert_tokens(
        r#"[1, 2.0, true, "hello"]"#,
        expect![[r#"
            [
                "Token { kind: LBracket, source: 0..1, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 1..2, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 2..3, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Decimal, source: 4..7, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 7..8, line: 1, preceded_by_blank_line: false }",
                "Token { kind: True, source: 9..13, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 13..14, line: 1, preceded_by_blank_line: false }",
                "Token { kind: String, source: 15..22, line: 1, preceded_by_blank_line: false }",
                "Token { kind: RBracket, source: 22..23, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    );
}

#[test]
fn dictionary() {
    assert_tokens(
        r#"#{2.0: "hello", 1: true}"#,
        expect![[r#"
            [
                "Token { kind: HashLBrace, source: 0..2, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Decimal, source: 2..5, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Colon, source: 5..6, line: 1, preceded_by_blank_line: false }",
                "Token { kind: String, source: 7..14, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 14..15, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 16..17, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Colon, source: 17..18, line: 1, preceded_by_blank_line: false }",
                "Token { kind: True, source: 19..23, line: 1, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 23..24, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    );
}

#[test]
fn set() {
    assert_tokens(
        r#"{2.0, "hello", 1, true}"#,
        expect![[r#"
            [
                "Token { kind: LBrace, source: 0..1, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Decimal, source: 1..4, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 4..5, line: 1, preceded_by_blank_line: false }",
                "Token { kind: String, source: 6..13, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 13..14, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 15..16, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 16..17, line: 1, preceded_by_blank_line: false }",
                "Token { kind: True, source: 18..22, line: 1, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 22..23, line: 1, preceded_by_blank_line: false }",
            ]"#]],
    );
}

#[test]
fn ranges() {
    assert_tokens(
        "1..10; 1..=10; 1..;",
        expect![[r#"
            [
                "Token { kind: Integer, source: 0..1, line: 1, preceded_by_blank_line: false }",
                "Token { kind: DotDot, source: 1..3, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 3..5, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 5..6, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 7..8, line: 1, preceded_by_blank_line: false }",
                "Token { kind: DotDotEqual, source: 8..11, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 11..13, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 13..14, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 15..16, line: 1, preceded_by_blank_line: false }",
                "Token { kind: DotDot, source: 16..18, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 18..19, line: 1, preceded_by_blank_line: false }",
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
                "Token { kind: Plus, source: 0..1, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Minus, source: 1..2, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Asterisk, source: 2..3, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Slash, source: 3..4, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Bang, source: 17..18, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Assign, source: 19..20, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Equal, source: 33..35, line: 3, preceded_by_blank_line: false }",
                "Token { kind: NotEqual, source: 36..38, line: 3, preceded_by_blank_line: false }",
                "Token { kind: GreaterThan, source: 39..40, line: 3, preceded_by_blank_line: false }",
                "Token { kind: LessThan, source: 41..42, line: 3, preceded_by_blank_line: false }",
                "Token { kind: GreaterThanEqual, source: 43..45, line: 3, preceded_by_blank_line: false }",
                "Token { kind: LessThanEqual, source: 46..48, line: 3, preceded_by_blank_line: false }",
                "Token { kind: AmpAmp, source: 61..63, line: 4, preceded_by_blank_line: false }",
                "Token { kind: PipePipe, source: 64..66, line: 4, preceded_by_blank_line: false }",
                "Token { kind: PipeGreater, source: 79..81, line: 5, preceded_by_blank_line: false }",
                "Token { kind: GreaterGreater, source: 82..84, line: 5, preceded_by_blank_line: false }",
                "Token { kind: DotDot, source: 97..99, line: 6, preceded_by_blank_line: false }",
                "Token { kind: DotDotEqual, source: 100..103, line: 6, preceded_by_blank_line: false }",
                "Token { kind: Underscore, source: 116..117, line: 7, preceded_by_blank_line: false }",
                "Token { kind: LParen, source: 118..119, line: 7, preceded_by_blank_line: false }",
                "Token { kind: RParen, source: 119..120, line: 7, preceded_by_blank_line: false }",
                "Token { kind: LBrace, source: 121..122, line: 7, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 122..123, line: 7, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 124..125, line: 7, preceded_by_blank_line: false }",
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
                "Token { kind: Let, source: 0..3, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 4..13, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Assign, source: 14..15, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Pipe, source: 16..17, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 17..18, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Pipe, source: 18..19, line: 1, preceded_by_blank_line: false }",
                "Token { kind: LBrace, source: 20..21, line: 1, preceded_by_blank_line: false }",
                "Token { kind: Let, source: 38..41, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 42..47, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Assign, source: 48..49, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Pipe, source: 50..51, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 51..52, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 52..53, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 54..55, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 55..56, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 57..58, line: 2, preceded_by_blank_line: false }",
                "Token { kind: Pipe, source: 58..59, line: 2, preceded_by_blank_line: false }",
                "Token { kind: LBrace, source: 60..61, line: 2, preceded_by_blank_line: false }",
                "Token { kind: If, source: 82..84, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 85..86, line: 3, preceded_by_blank_line: false }",
                "Token { kind: GreaterThan, source: 87..88, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 89..90, line: 3, preceded_by_blank_line: false }",
                "Token { kind: LBrace, source: 91..92, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Return, source: 93..99, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 100..105, line: 3, preceded_by_blank_line: false }",
                "Token { kind: LParen, source: 105..106, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 106..107, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 107..108, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 109..110, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Plus, source: 111..112, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 113..114, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 114..115, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 116..117, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Minus, source: 118..119, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 120..121, line: 3, preceded_by_blank_line: false }",
                "Token { kind: RParen, source: 121..122, line: 3, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 123..124, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Else, source: 125..129, line: 3, preceded_by_blank_line: false }",
                "Token { kind: LBrace, source: 130..131, line: 3, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 132..133, line: 3, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 134..135, line: 3, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 152..153, line: 4, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 153..154, line: 4, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 171..176, line: 5, preceded_by_blank_line: false }",
                "Token { kind: LParen, source: 176..177, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 177..178, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 178..179, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 180..181, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Comma, source: 181..182, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 183..184, line: 5, preceded_by_blank_line: false }",
                "Token { kind: RParen, source: 184..185, line: 5, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 185..186, line: 5, preceded_by_blank_line: false }",
                "Token { kind: RBrace, source: 199..200, line: 6, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 200..201, line: 6, preceded_by_blank_line: false }",
                "Token { kind: Identifier, source: 214..223, line: 7, preceded_by_blank_line: false }",
                "Token { kind: LParen, source: 223..224, line: 7, preceded_by_blank_line: false }",
                "Token { kind: Integer, source: 224..226, line: 7, preceded_by_blank_line: false }",
                "Token { kind: RParen, source: 226..227, line: 7, preceded_by_blank_line: false }",
                "Token { kind: Semicolon, source: 227..228, line: 7, preceded_by_blank_line: false }",
            ]"#]],
    );
}

fn assert_tokens(input: &str, expected: Expect) {
    let tokens: Vec<String> = Lexer::new(input.trim()).map(|token| format!("{:?}", token)).collect();
    let actual = format!("{:#?}", tokens);
    expected.assert_eq(&actual)
}

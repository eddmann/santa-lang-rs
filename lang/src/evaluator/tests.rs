use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn integers() {
    #[rustfmt::skip]
    let cases = vec![
        ("5", "5"),
        ("125", "125"),
        ("10_000", "10000"),        
    ];
    test_cases_str(cases);
}

#[test]
fn decimals() {
    #[rustfmt::skip]
    let cases = vec![
        ("5.05", "5.05"),
        ("5.25", "5.25"),
        ("5.50", "5.5"),
        ("5.5", "5.5"),      
    ];
    test_cases_str(cases);
}

#[test]
fn booleans() {
    #[rustfmt::skip]
    let cases = vec![
        ("true", "true"),
        ("false", "false"),    
    ];
    test_cases_str(cases);
}

#[test]
fn strings() {
    #[rustfmt::skip]
    let cases = vec![
        (r#""Hello, world!""#, r#""Hello, world!""#),
        //(r#""\\ \" \n \t""#, r#""Hello, world!""#),
    ];
    test_cases_str(cases);
}

#[test]
fn nil() {
    #[rustfmt::skip]
    let cases = vec![
        ("nil", "nil"),
    ];
    test_cases_str(cases);
}

#[test]
fn lists() {
    #[rustfmt::skip]
    let cases = vec![
        ("[1, 2, 3]", "[1, 2, 3]"),
        ("[1, 2.25, \"3\", true, {1}, #{1: 2}, [1..3]]", "[1, 2.25, \"3\", true, {1}, #{1: 2}, [1..3]]"),
        ("[1, ..[2, 3], 4]", "[1, 2, 3, 4]")
    ];
    test_cases_str(cases);
}

#[test]
fn sets() {
    #[rustfmt::skip]
    let cases = vec![
        ("{1, 2, 3}", "{1, 2, 3}"),
        ("{[1..3], 1, \"3\", #{1: 2}, 2.25, {1}, true}", "{[1..3], 1, \"3\", #{1: 2}, 2.25, {1}, true}"),
        ("{1, ..[2, 3], 4}", "{1, 2, 4, 3}")
    ];
    test_cases_str(cases);
}

fn test_cases_str(cases: Vec<(&str, &str)>) {
    cases.iter().for_each(|(src, expected)| {
        let actual = evaluate(src);
        assert!(actual == *expected, "\nexpected: {}\ngot: {}", expected, actual);
    });
}

// #[test]
// fn test_integer() {
//     // let result = evaluate(
//     //     r#"
//     //     let a = |a| ax
//     //     let b = |b| a(b)
//     //     let c = |c| b(1)

//     //     c(1)
//     // "#,
//     // );
//     let result = evaluate(
//         r#"
//         [[1, 2], [3], [2,[3]]] |> flat_map(|a| a)
//         0.. |> skip(30) |> map(|a| a) |> skip(5) |> take(5) |> list
//         repeat(10) |> take(4)

//         let a = cycle([1, 2, 3]);
//         let b = a |> skip(2);
//         [a |> take(5), b |> take(5)]

//         iterate(|a| a, 1) |> take(5)

//         let f = |..a| a

//         f(..[1, 2, 3]);

//         zip(1.., 2.., [1, 2, 3], "abc", 5..0);

//         zip(1.., 2.., 3.., 4.., 5..) |> skip(2) |> take(5) |> list;

//         1..10 |> fold(0, -);

//         "123" |> map(_ * 2) |> set;

//         // let x = || 1; fix me

//         1 <= 2
//     "#,
//     );
//     //assert_eq!("2", result);
// }

fn evaluate(input: &str) -> String {
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate(&program) {
        Ok(value) => value.to_string(),
        Err(error) => format!("{:?}", error),
    }
}

// https://gitlab.com/findley/monkey-lang/-/blob/main/lib/src/eval/tests.rs

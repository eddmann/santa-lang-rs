use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_integer() {
    // let result = evaluate(
    //     r#"
    //     let a = |a| ax
    //     let b = |b| a(b)
    //     let c = |c| b(1)

    //     c(1)
    // "#,
    // );
    let result = evaluate(
        r#"
        [[1, 2], [3], [2,[3]]] |> flat_map(|a| a)
        0.. |> skip(30) |> map(|a| a) |> skip(5) |> take(5) |> list
        repeat(10) |> take(4)
    
        let a = cycle([1, 2, 3]);
        let b = a |> skip(2);
        [a |> take(5), b |> take(5)]

        iterate(|a| a, 1) |> take(5)
        

        let f = |..a| a

        f(..[1, 2, 3]);
        
        zip(1.., 2.., [1, 2, 3], "abc", 5..0);

        zip(1.., 2.., 3.., 4.., 5..) |> skip(2) |> take(5) |> list;
        
        1..10 |> fold(0, -)
    "#,
    );
    assert_eq!("2", result);
}

fn evaluate(input: &str) -> String {
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate(&program) {
        Ok(value) => value.to_string(),
        Err(value) => format!("{:?}", value),
    }
}

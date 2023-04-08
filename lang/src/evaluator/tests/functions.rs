use crate::evaluator::function::{Arguments, ExternalFnDef};
use crate::evaluator::Object;
use crate::parser::ast::ExpressionKind;
use std::rc::Rc;

test_eval! {
    suite functions;

    ("let fn = |x, y| { x + y }; fn(5, 2)", "7", function_literal_with_block),
    ("let fn = |x, y| x + y; fn(5, 2)", "7", function_literal_with_expression),
    ("let fn = |a, [b, ..c], ..d| [a, b, c, d]; fn(1, [2, 3, 4], 5, 6)", "[1, 2, [3, 4], [5, 6]]", function_literal_with_parameter_destructuring),
    ("let fn = |x, y| { x + y }; fn(4)(3)", "7", curried_call),
    (
        r#"
            let add = |x, y| { x + y };
            let inc = add(6);
            inc(1);
        "#,
        "7",
        paritial_application_using_call
    ),
    (
        r#"
            let inc = _ + 1;
            let dec = -1 + _;
            let add = _ + _;
            add(dec(6), inc(1));
        "#,
        "7",
        paritial_application_using_placeholders
    ),
    ("let fn = |x, y| { x + y }; fn(..[3, 4])", "7", spread_call),
    (
        r#"
            let repeat = |value, times| {
                if times > 1 {
                    [value] + repeat(value, times - 1)
                } else {
                    [value]
                }
            }
            repeat(1, 3);
        "#,
        "[1, 1, 1]",
        recursive_call
    ),
    ("let add = |x, y| { x + y }; 3 `add` 4", "7", infix_call),
    (
        r#"
            let add = |a, b| { a + b };
            let inc = add(1);
            3 |> add(1) |> |a| { a + 1 } |> inc |> _ + 1;
        "#,
        "7",
        function_threading
    ),
    (
        r#"
            let add = |a, b| { a + b };
            let inc = add(1);
            let add4 = add(1) >> |a| { a + 1 } >> inc >> _ + 1;
            add4(3);
        "#,
        "7",
        function_composition
    ),
    (
        r#"
            let mut sum = 0;
            [1, 2, 3] |> each |n| {
                sum = sum + n;
            }
            sum
        "#,
        "6",
        trailing_lambda_without_call_expression
    ),
    (
        r#"
            let fn = |greeting, fn| { fn(greeting) };
            fn("Hello") |greeting| {
                greeting + "!"
            };
        "#,
        "\"Hello!\"",
        trailing_lambda_with_call_expression
    ),
    (
        r#"
            let fibonacci = |n| {
                let recur = |x, y, n| {
                    if n > 0 { return recur(y, x + y, n - 1) } else { x }
                };
                recur(0, 1, n);
            };
            fibonacci(90);
        "#,
        "2880067194370816120",
        recursive_call_with_explicit_return
    ),
    (
        r#"
            let fibonacci = |n| {
                let recur = |x, y, n| {
                    if n > 0 { recur(y, x + y, n - 1) } else { x }
                };
                recur(0, 1, n);
            };
            fibonacci(90);
        "#,
        "2880067194370816120",
        recursive_call_with_implicit_return
    ),
    (
        r#"
            let counter = || {
                let mut total = 0;
                || total = total + 1;
            }();
            counter(); counter(); counter();
        "#,
        "3",
        enclosed_function_closure_state
    ),
    (
        r#"
            let mut x = 0;
            let f = memoize |n| x = x + n;
            f(1); f(2); f(1);
            x;
        "#,
        "3",
        memoization
    ),
    (
        r#"
            let fibonacci = memoize |n| if (n > 1) { fibonacci(n - 1) + fibonacci(n - 2) } else { n };
            fibonacci(30)
        "#,
        "832040",
        recursive_memoization
    )
}

#[test]
fn external_function() {
    let hello_template = String::from("Hello, {}!");
    let hello_function: ExternalFnDef = (
        "hello".to_owned(),
        vec![ExpressionKind::Identifier("name".to_owned())],
        Rc::new(move |arguments: Arguments| match &**arguments.get("name").unwrap() {
            Object::String(name) => Ok(Rc::new(Object::String(hello_template.replace("{}", name)))),
            _ => Ok(Rc::new(Object::Nil)),
        }),
    );

    let source = "hello(\"world\");";
    let mut parser = crate::parser::Parser::new(crate::lexer::Lexer::new(source));
    let program = parser.parse().unwrap();
    let mut evaluator = crate::evaluator::Evaluator::new_with_external_functions(vec![hello_function]);
    let actual = match evaluator.evaluate(&program) {
        Ok(value) => value.to_string(),
        Err(error) => error.message,
    };

    assert_eq!("\"Hello, world!\"", actual);
}

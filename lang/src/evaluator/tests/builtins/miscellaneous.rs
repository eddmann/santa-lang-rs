test_eval! {
    suite range;

    ("range(1, 10, 2) |> list", "[1, 3, 5, 7, 9]", positive_step),
    ("range(10, 1, -2) |> list", "[10, 8, 6, 4, 2]", negative_step),
    ("range(1, 10, 20) |> list", "[1]", step_larger_than_range)
}

test_eval! {
    suite r#type;

    ("type(1)", "\"Integer\"", integer),
    ("type(1.5)", "\"Decimal\"", decimal),
    ("type(\"\")", "\"String\"", string),
    ("type(1..10)", "\"BoundedRange\"", bounded_range),
    ("type(1..)", "\"UnboundedRange\"", unbounded_range),
    ("type(|| 1)", "\"Function\"", function)
}

test_eval! {
    suite id;

    ("id(nil)", "nil", nil),
    ("id(1)", "1", integer),
    ("id(1.5)", "1.5", decimal),
    ("id(\"\")", "\"\"", string)
}

test_eval! {
    suite evaluate;

    ("evaluate(\"\")", "nil", empty_source),
    ("evaluate(\"[1, 2, 3]\")", "[1, 2, 3]", collection),
    ("evaluate(\"[1, 2, 3] |> map(_ + 1)\")", "[2, 3, 4]", expression)
}

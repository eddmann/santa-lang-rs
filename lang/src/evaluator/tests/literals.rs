test_eval! {
    suite integer;

    ("5", "5", single_number),
    ("125", "125", multi_number),
    ("1_000_000", "1000000", with_underscore_seperators),
    ("-5", "-5", negative)
}

test_eval! {
    suite decimal;

    ("5.05", "5.05", with_single_fraction),
    ("5.25", "5.25", with_multi_number_fraction),
    ("5.50", "5.5", with_trailing_fraction_zero),
    ("5.5", "5.5", with_no_trailing_fraction_zero),
    ("-5.25", "-5.25", negative)
}

test_eval! {
    suite boolean;

    ("true", "true", true_value),
    ("false", "false", false_value)
}

test_eval! {
    suite string;

    (r#""Hello, world!""#, r#""Hello, world!""#, ascii),
    ("\"µࠒ𒀀\"", "\"µࠒ𒀀\"", unicode),
    (r#""\\r \\n \\t \\\\ \"""#, r#""\r \n \t \\ """#, escaped_characters)
}

test_eval! {
    suite nil;

    ("nil", "nil", literal)
}

test_eval! {
    suite list;

    ("[1, 2, 3]", "[1, 2, 3]", homogeneous),
    ("[1, 2.25, \"3\", true, nil, {1}, #{1: 2}, [1..3]]", "[1, 2.25, \"3\", true, nil, {1}, #{1: 2}, [1..3]]", heterogeneous),
    (
        r#"
            let x = [2, 3];
            [1, ..x, ..["a", "b", "c"]];
        "#,
        "[1, 2, 3, \"a\", \"b\", \"c\"]",
        spread
    ),
    ("let r = 1..4; [..r]", "[1, 2, 3]", spread_exclusive_range),
    ("let r = 1..=3; [..r]", "[1, 2, 3]", spread_inclusive_range),
    ("let r = 1..3; [0, ..r, 4]", "[0, 1, 2, 4]", spread_range_middle)
}

test_eval! {
    suite set;

    ("{1, 2, 3}", "{1, 2, 3}", homogeneous),
    ("{1, 2, 3, 1, 2, 4}", "{1, 2, 4, 3}", homogeneous_with_duplicates),
    ("{1, \"3\", 2.25, {1}, [2], true}", "{1, \"3\", 2.25, {1}, [2], true}", heterogeneous),
    ("{1, \"3\", 2.25, {1}, [2], true, 2.25, {1}, [2]}", "{1, \"3\", 2.25, {1}, [2], true}", heterogeneous_with_duplicates),
    ("{|a| a}", "Unable to include a Function within an Set", function_disallowed),
    ("{1..5}", "Unable to include a LazySequence within an Set", lazy_sequence_disallowed)
}

test_eval! {
    suite dictionary;

    ("#{\"1\": 1, \"2\": 2, \"3\": 3}", "#{\"3\": 3, \"1\": 1, \"2\": 2}", homogeneous),
    ("#{\"1\": 1, \"2\": 2, \"3\": 3, \"1\": 4}", "#{\"3\": 3, \"1\": 4, \"2\": 2}", homogeneous_with_duplicates),
    ("#{1: true, \"2\": {nil}, 3.0: [1..5], {1}: #{1: 2}}", "#{1: true, 3: [1..5], {1}: #{1: 2}, \"2\": {nil}}", heterogeneous),
    ("#{1: true, \"2\": {nil}, 3.0: [1..5], {1}: #{1: 2}, {1}: 2}", "#{1: true, 3: [1..5], {1}: 2, \"2\": {nil}}", heterogeneous_with_duplicates),
    ("#{(|a| a): 1}", "Unable to use a Function as a Dictionary key", function_key_disallowed),
    ("#{1..5: 1}", "Unable to use a LazySequence as a Dictionary key", lazy_sequence_key_disallowed)
}

test_eval! {
    suite range;

    ("1..5", "1..5", exclusive),
    ("1..=5", "1..=5", inclusive),
    ("1..", "1..", unbounded)
}

test_eval! {
    suite range_iteration;

    // Ascending non-inclusive
    ("1..3 |> list", "[1, 2]", ascending_non_inclusive),
    // Ascending inclusive
    ("1..=3 |> list", "[1, 2, 3]", ascending_inclusive),
    // Descending non-inclusive
    ("3..1 |> list", "[3, 2]", descending_non_inclusive),
    // Descending inclusive
    ("3..=1 |> list", "[3, 2, 1]", descending_inclusive),
    // Crossing zero descending non-inclusive
    ("1..-1 |> list", "[1, 0]", crossing_zero_descending_non_inclusive),
    // Crossing zero descending inclusive
    ("1..=-1 |> list", "[1, 0, -1]", crossing_zero_descending_inclusive),
    // Crossing zero ascending non-inclusive
    ("-1..1 |> list", "[-1, 0]", crossing_zero_ascending_non_inclusive),
    // Crossing zero ascending inclusive
    ("-1..=1 |> list", "[-1, 0, 1]", crossing_zero_ascending_inclusive),
    // Empty range (non-inclusive with equal bounds)
    ("3..3 |> list", "[]", empty_non_inclusive)
}

test_eval! {
    suite range_map_iteration;

    // Ascending non-inclusive with map
    ("1..3 |> map(|x| x * 2) |> list", "[2, 4]", ascending_non_inclusive),
    // Ascending inclusive with map
    ("1..=3 |> map(|x| x * 2) |> list", "[2, 4, 6]", ascending_inclusive),
    // Descending non-inclusive with map
    ("3..1 |> map(|x| x * 2) |> list", "[6, 4]", descending_non_inclusive),
    // Descending inclusive with map
    ("3..=1 |> map(|x| x * 2) |> list", "[6, 4, 2]", descending_inclusive),
    // Crossing zero with map
    ("1..-1 |> map(|x| x * 2) |> list", "[2, 0]", crossing_zero_descending),
    ("-1..1 |> map(|x| x * 2) |> list", "[-2, 0]", crossing_zero_ascending)
}

test_eval! {
    suite comments;

    (
        r#"
            // line comment
            1
        "#,
        "1",
        line
    ),
    ("1 // trailing comment", "1", trailing)
}

test_eval! {
    suite negation;

    ("!true", "false", true_boolean),
    ("!false", "true", false_boolean),
    ("!(\"abc\")", "false", truthy_string_value),
    ("!(\"\")", "true", falsey_string_value),
    ("!([1, 2, 3])", "false", truthy_list_value),
    ("!([])", "true", falsey_list_value),
    ("!({1, 2, 3})", "false", truthy_set_value),
    ("!({})", "true", falsey_set_value),
    ("!(#{1: 2})", "false", truthy_dictionary_value),
    ("!(#{})", "true", falsey_dictionary_value)
}

test_eval! {
    suite if_expression;

    ("if 1 == 1 { 1 }", "1", true_without_else),
    ("if 1 != 1 { 1 }", "nil", false_without_else),
    ("if 1 == 1 { 1 } else { 2 }", "1", true_with_else),
    ("if 1 != 1 { 1 } else { 2 }", "2", false_with_else),
    ("if let x = 1 { x }", "1", true_let_assignment),
    ("if let x = 0 { x }", "nil", false_let_assignment)
}

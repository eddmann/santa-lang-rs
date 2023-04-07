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
    ("[1, 2.25, \"3\", true, nil, {1}, #{1: 2}, [1..3]]", "[1, 2.25, \"3\", true, nil, {1}, #{1: 2}, [1..3]]", heterogeneous)
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
    suite hash;

    ("#{\"1\": 1, \"2\": 2, \"3\": 3}", "#{\"3\": 3, \"1\": 1, \"2\": 2}", homogeneous),
    ("#{\"1\": 1, \"2\": 2, \"3\": 3, \"1\": 4}", "#{\"3\": 3, \"1\": 4, \"2\": 2}", homogeneous_with_duplicates),
    ("#{1: true, \"2\": {nil}, 3.0: [1..5], {1}: #{1: 2}}", "#{1: true, 3: [1..5], {1}: #{1: 2}, \"2\": {nil}}", heterogeneous),
    ("#{1: true, \"2\": {nil}, 3.0: [1..5], {1}: #{1: 2}, {1}: 2}", "#{1: true, 3: [1..5], {1}: 2, \"2\": {nil}}", heterogeneous_with_duplicates),
    ("#{(|a| a): 1}", "Unable to use a Function as a Hash key", function_key_disallowed),
    ("#{1..5: 1}", "Unable to use a LazySequence as a Hash key", lazy_sequence_key_disallowed)
}

test_eval! {
    suite range;

    ("1..5", "1..5", exclusive),
    ("1..=5", "1..=5", inclusive),
    ("1..", "1..", unbounded)
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
    ("!(#{1: 2})", "false", truthy_hash_value),
    ("!(#{})", "true", falsey_hash_value)
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

// test_eval! {
//     suite match_expression;
//     (
//         r#"
//             let sut = |x| match x {
//                 "1" { "1" }
//                 2 { "2" }
//                 3.5 { "3" }
//                 x { ["4", x] }
//             };
//             return [sut("1"), sut(2), sut(3.5), sut([4])]
//         "#,
//         "",
//         basic
//     )
// }

test_eval! {
    suite abs;

    ("abs(1)", "1", positive_integer),
    ("abs(-1)", "1", negative_integer),
    ("abs(1.5)", "1.5", positive_decimal),
    ("abs(-1.5)", "1.5", negative_decimal)
}

test_eval! {
    suite vec_add;

    ("vec_add([], [])", "[]", empty_list),
    ("vec_add([1], [2])", "[3]", single_element_lists),
    ("vec_add([1, 2], [3, 4])", "[4, 6]", multi_element_list),
    ("vec_add([1, 2], [3])", "[4]", different_list_length)
}

test_eval! {
    suite signum;

    ("signum(0)", "0", zero_integer),
    ("signum(10)", "1", positive_integer),
    ("signum(-10)", "-1", negative_integer),
    ("signum(10.5)", "1", positive_decimal),
    ("signum(-10.5)", "-1", negative_decimal)
}

test_eval! {
    suite bit_operations;

    ("bit_and(3, 5)", "1", bit_and),
    ("bit_or(3, 5)", "7", bit_or),
    ("bit_xor(3, 5)", "6", bit_xor),
    ("bit_shift_left(1, 2)", "4", bit_shift_left),
    ("bit_shift_right(16, 1)", "8", bit_shift_right)
}

test_eval! {
    suite int;

    ("int(true)", "1", true_boolean),
    ("int(false)", "0", false_boolean),
    ("int(-1)", "-1", negative_integer),
    ("int(1)", "1", positive_integer),
    ("int(-1.5)", "-2", negative_decimal),
    ("int(1.5)", "2", positive_decimal),
    ("int(\"\")", "0", empty_string),
    ("int(\"-1\")", "-1", negative_integer_string),
    ("int(\"1\")", "1", positive_integer_string),
    ("int(\"-1.5\")", "-2", negative_decimal_string),
    ("int(\"1.5\")", "2", positive_decimal_string),
    ("int(\"abc\")", "0", invalid_string)
}

test_eval! {
    suite ints;

    ("ints(\"abc\")", "[]", no_ints),
    ("ints(\"1 2 3\")", "[1, 2, 3]", space_seperators),
    ("ints(\"1,2,3\")", "[1, 2, 3]", comma_seperators),
    ("ints(\"1a 2b 3c\")", "[1, 2, 3]", letter_seperators)
}

test_eval! {
    suite lines;

    ("lines(\"\")", "[]", empty_string),
    ("lines(\"abc\")", "[\"abc\"]", single_line),
    ("lines(\"abc\ndef\")", "[\"abc\", \"def\"]", multi_line)
}

test_eval! {
    suite split;

    ("split(\",\", \"\")", "[\"\"]", empty_string),
    ("split(\",\", \"abc\")", "[\"abc\"]", string_without_match),
    ("split(\",\", \"a,bc\")", "[\"a\", \"bc\"]", string_with_single_match),
    ("split(\",\", \"a,b,c\")", "[\"a\", \"b\", \"c\"]", string_with_multiple_matches)
}

test_eval! {
    suite regex_match;

    ("regex_match(\"([0-9]), ([0-9]{2}), ([0-9]+)\", \"1, 22, 333\")", "[\"1\", \"22\", \"333\"]", match_found),
    ("regex_match(\"([0-9]), ([0-9]{2}), ([0-9]+)\", \"1, 22\")", "[]", match_not_found),
    ("regex_match(\"[0-+a]\", \"\")", "Failed to compile regex pattern: [0-+a]", invalid_pattern)
}

test_eval! {
    suite regex_match_all;

    ("regex_match_all(\"([0-9]+)\", \"1, 22, 333\")", "[\"1\", \"22\", \"333\"]", match_found),
    ("regex_match_all(\"([0-9]+)\", \"abc\")", "[]", match_not_found),
    ("regex_match_all(\"[0-+a]\", \"\")", "Failed to compile regex pattern: [0-+a]", invalid_pattern)
}

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
    ("type(1..10)", "\"LazySequence\"", lazy_sequence),
    ("type(|| 1)", "\"Function\"", function)
}

test_eval! {
    suite id;

    ("id(nil)", "nil", nil),
    ("id(1)", "1", integer),
    ("id(1.5)", "1.5", decimal),
    ("id(\"\")", "\"\"", string)
}

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
    suite md5;

    ("md5(\"\")", "\"d41d8cd98f00b204e9800998ecf8427e\"", empty_string),
    ("md5(\"hello\")", "\"5d41402abc4b2a76b9719d911017c592\"", hello),
    ("md5(\"Hello, World!\")", "\"65a8e27d8879283831b664bd8b7f0ad4\"", hello_world)
}

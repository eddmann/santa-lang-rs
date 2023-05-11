test_eval! {
    suite plus;

    ("1 + 1", "2", integer_integer),
    ("1 + 1.5", "2", integer_decimal),
    ("1.5 + 1.5", "3", decimal_decimal),
    ("1.5 + 1", "2.5", decimal_integer),
    ("\"a\" + \"b\"", "\"ab\"", string_string),
    ("\"a\" + 1", "\"a1\"", string_integer),
    ("\"a\" + 1.5", "\"a1.5\"", string_decimal),
    ("[1, 2, 3] + [4, 5, 6]", "[1, 2, 3, 4, 5, 6]", list_list),
    ("[1, 2, 3] + {4}", "[1, 2, 3, 4]", list_set),
    ("#{1: 2} + #{2: 3}", "#{1: 2, 2: 3}", dictionary_dictionary),
    ("{1, 2, 3} + {2, 3, 4}", "{1, 2, 4, 3}", set_set),
    ("{1, 2, 3} + [2, 3, 4]", "{1, 2, 4, 3}", set_list),
    ("+(1, 2)", "3", function_call)
}

test_eval! {
    suite minus;

    ("2 - 1", "1", integer_integer),
    ("3 - 1.5", "2", integer_decimal),
    ("2.5 - 1.2", "1.3", decimal_decimal),
    ("1.5 - 1", "0.5", decimal_integer),
    ("[1, 2, 3] - [2]", "[1, 3]", list_list),
    ("[1, 2, 3] - {3}", "[1, 2]", list_set),
    ("{1, 2, 3} - {2, 3, 4}", "{1}", set_set),
    ("{1, 2, 3} - [2, 3, 4]", "{1}", set_list),
    ("let x = -; x(2, 1)", "1", function_call)
}

test_eval! {
    suite asterisk;

    ("2 * 2", "4", integer_integer),
    ("1 * 5.5", "5", integer_decimal),
    ("1.5 * 1.5", "2.25", decimal_decimal),
    ("1.5 * 3", "4.5", decimal_integer),
    ("\"a\" * 3", "\"aaa\"", string_integer),
    ("[1, 2] * 2", "[1, 2, 1, 2]", list_integer),
    ("*(2, 2)", "4", function_call)
}

test_eval! {
    suite slash;

    ("5 / 2", "2", integer_integer),
    ("6 / 3.2", "2", integer_decimal),
    ("5.4 / 3.2", "1.6875", decimal_decimal),
    ("4.5 / 2", "2.25", decimal_integer),
    ("/(4, 2)", "2", function_call)
}

test_eval! {
    suite modulo;

    ("5 % 2", "1", positive_positive),
    ("5 % -2", "-1", positive_negative),
    ("-5 % 4", "3", negative_positive),
    ("-5 % -4", "-1", negative_negative),
    ("%(5, 2)", "1", function_call)
}

test_eval! {
    suite equal;

    ("1 == 1", "true", true_integer_integer),
    ("1 == 2", "false", false_integer_integer),
    ("1 == 1.0", "false", false_integer_decimal),
    ("1.5 == 1.5", "true", true_decimal_decimal),
    ("1.5 == 2.4", "false", false_decimal_decimal),
    ("1.0 == 1", "false", false_decimal_integer),
    ("\"a\" == \"a\"", "true", true_string_string),
    ("\"a\" == \"b\"", "false", false_string_string),
    ("\"1\" == 1", "false", false_string_integer),
    ("\"1.5\" == 1.5", "false", false_string_decimal),
    ("[1, 2, 3] == [1, 2, 3]", "true", true_list_list),
    ("[1, 2, 3] == [2, 3, 4]", "false", false_list_list),
    ("[1, 2, 3] == {1, 2, 3}", "false", false_list_set),
    ("#{1: \"one\", 1.5: [1, 2, 3], true: #{1: 2}} == #{1: \"one\", 1.5: [1, 2, 3], true: #{1: 2}}", "true", true_dictionary_dictionary),
    ("#{1: 2} == #{2: 1}", "false", false_dictionary),
    ("{1, 2, 3} == {1, 2, 3}", "true", true_set_set),
    ("{1, 2, 3} == {1, 2, 3, 2}", "true", true_set_set_with_duplicates),
    ("{1, 2, 3} == {2, 3, 4}", "false", false_set_set),
    ("==(1, 1)", "true", function_call)
}

test_eval! {
    suite not_equal;

    ("1 != 1", "false", false_integer_integer),
    ("1 != 2", "true", true_integer_integer),
    ("1 != 1.0", "true", true_integer_decimal),
    ("1.5 != 1.5", "false", false_decimal_decimal),
    ("1.5 != 2.4", "true", true_decimal_decimal),
    ("1.0 != 1", "true", true_decimal_integer),
    ("\"a\" != \"a\"", "false", false_string_string),
    ("\"a\" != \"b\"", "true", true_string_string),
    ("\"1\" != 1", "true", true_string_integer),
    ("\"1.5\" != 1.5", "true", true_string_decimal),
    ("[1, 2, 3] != [1, 2, 3]", "false", false_list_list),
    ("[1, 2, 3] != [2, 3, 4]", "true", true_list_list),
    ("[1, 2, 3] != {1, 2, 3}", "true", true_list_set),
    ("#{1: \"one\", 1.5: [1, 2, 3], true: #{1: 2}} != #{1: \"one\", 1.5: [1, 2, 3], true: #{1: 2}}", "false", false_dictionary_dictionary),
    ("#{1: 2} != #{2: 1}", "true", true_dictionary),
    ("{1, 2, 3} != {1, 2, 3}", "false", false_set_set),
    ("{1, 2, 3} != {1, 2, 3, 2}", "false", false_set_set_with_duplicates),
    ("{1, 2, 3} != {2, 3, 4}", "true", true_set_set),
    ("!=(1, 2)", "true", function_call)
}

test_eval! {
    suite less_than;

    ("1 < 2", "true", true_integer_integer),
    ("2 < 1", "false", false_integer_integer),
    ("1.5 < 2.5", "true", true_decimal_decimal),
    ("2.5 < 1.5", "false", false_decimal_decimal),
    ("<(1, 2)", "true", function_call)
}

test_eval! {
    suite less_than_equal;

    ("1 <= 2", "true", less_integer_integer),
    ("1 <= 1", "true", equal_integer_integer),
    ("2 <= 1", "false", false_integer_integer),
    ("1.5 <= 2.5", "true", less_decimal_decimal),
    ("1.5 <= 1.5", "true", equal_decimal_decimal),
    ("2.5 <= 1.5", "false", false_decimal_decimal),
    ("<=(1, 2)", "true", function_call)
}

test_eval! {
    suite greater_than;

    ("2 > 1", "true", true_integer_integer),
    ("1 > 2", "false", false_integer_integer),
    ("2.5 > 1.5", "true", true_decimal_decimal),
    ("1.5 > 2.5", "false", false_decimal_decimal),
    (">(2, 1)", "true", function_call)
}

test_eval! {
    suite greater_than_equal;

    ("2 >= 1", "true", greater_integer_integer),
    ("1 >= 1", "true", equal_integer_integer),
    ("1 >= 2", "false", false_integer_integer),
    ("2.5 >= 1.5", "true", greater_decimal_decimal),
    ("1.5 >= 1.5", "true", equal_decimal_decimal),
    ("1.5 >= 2.5", "false", false_decimal_decimal),
    (">=(2, 1)", "true", function_call)
}

test_eval! {
    suite and;

    ("true && !false", "true", true_value),
    ("true && false", "false", false_value),
    ("let mut x = true; false && (|| x = false)(); x", "true", short_circuit_evaluation),
    ("and(true, !false)", "true", function_call)
}

test_eval! {
    suite or;

    ("true || !false", "true", true_value),
    ("!true || false", "false", false_value),
    ("let mut x = true; true || (|| x = false)(); x", "true", short_circuit_evaluation),
    ("or(true, false)", "true", function_call)
}

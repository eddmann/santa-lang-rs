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

test_eval! {
    suite lists;

    sut "let list = [1, 2, 3];";

    ("list[1]", "2", positive_integer),
    ("list[-1]", "3", negative_integer),
    ("list[3]", "nil", out_of_range_integer),
    ("list[1..2]", "[2]", exclusive_positive_range),
    ("list[1..=2]", "[2, 3]", inclusive_positive_range),
    ("list[1..-1]", "[2, 1]", exclusive_positive_range_with_negative_until),
    ("list[1..=-1]", "[2, 1, 3]", inclusive_positive_range_with_negative_to),
    ("list[1..]", "[2, 3]", unbounded_positive_range),
    ("list[-2..]", "[2, 3]", unbounded_negative_range)
}

test_eval! {
    suite sets;

    sut "let set = {1, 1.5, \"hello\", true, [1, 2, 3]};";

    ("set[1]", "1", integer),
    ("set[1.5]", "1.5", decimal),
    ("set[\"hello\"]", "\"hello\"", string),
    ("set[true]", "true", boolean),
    ("set[[1, 2, 3]]", "[1, 2, 3]", list),
    ("set[\"unknown\"]", "nil", unknown_value)
}

test_eval! {
    suite hashs;

    sut "let hash = #{1: \"integer\", 1.5: [1, 2, 3], \"hello\": \"world\", true: 1..5, [3, 2, 1]: true};";

    ("hash[1]", "\"integer\"", integer),
    ("hash[1.5]", "[1, 2, 3]", decimal),
    ("hash[\"hello\"]", "\"world\"", string),
    ("hash[true]", "1..5", boolean),
    ("hash[[3, 2, 1]]", "true", list),
    ("hash[\"unknown\"]", "nil", unknown_value)
}

test_eval! {
    suite lazy_sequences;

    sut "let sequence = 1..5;";

    ("sequence[1]", "2", positive_integer),
    ("sequence[5]", "nil", out_of_range_integer)
}

test_eval! {
    suite strings;

    sut "let string = \"hello\";";

    ("string[1]", "\"e\"", positive_integer),
    ("string[-1]", "\"o\"", negative_integer),
    ("string[5]", "nil", out_of_range_integer),
    ("string[1..2]", "\"e\"", exclusive_positive_range),
    ("string[1..=2]", "\"el\"", inclusive_positive_range),
    ("string[1..-1]", "\"eh\"", exclusive_positive_range_with_negative_until),
    ("string[1..=-1]", "\"eho\"", inclusive_positive_range_with_negative_to),
    ("string[1..]", "\"ello\"", unbounded_positive_range),
    ("string[-2..]", "\"lo\"", unbounded_negative_range)
}

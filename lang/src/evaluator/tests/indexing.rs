test_eval! {
    suite lists;

    sut "let list = [1, 2, 3];";

    ("list[1]", "2", positive_integer),
    ("list[-1]", "3", negative_integer),
    ("list[3]", "nil", out_of_range_integer),
    ("list[1..2]", "[2]", exclusive_positive_range),
    ("list[1..=2]", "[2, 3]", inclusive_positive_range),
    ("list[1..-1]", "[2]", exclusive_positive_range_with_negative_until),
    ("list[1..=-1]", "[2, 3]", inclusive_positive_range_with_negative_to),
    ("list[1..]", "[2, 3]", unbounded_positive_range),
    ("list[-2..]", "[2, 3]", unbounded_negative_range),
    // Additional tests for negative range indexing
    ("list[0..-1]", "[1, 2]", exclusive_drop_last),
    ("[1, 2, 3, 4, 5][0..-1]", "[1, 2, 3, 4]", exclusive_drop_last_longer),
    ("[1, 2, 3, 4, 5][-3..-1]", "[3, 4]", exclusive_negative_start_and_end)
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
    suite dictionaries;

    sut "let dictionary = #{1: \"integer\", 1.5: [1, 2, 3], \"hello\": \"world\", true: 1..5, [3, 2, 1]: true};";

    ("dictionary[1]", "\"integer\"", integer),
    ("dictionary[1.5]", "[1, 2, 3]", decimal),
    ("dictionary[\"hello\"]", "\"world\"", string),
    ("dictionary[true]", "1..5", boolean),
    ("dictionary[[3, 2, 1]]", "true", list),
    ("dictionary[\"unknown\"]", "nil", unknown_value)
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
    ("string[1..-1]", "\"ell\"", exclusive_positive_range_with_negative_until),
    ("string[1..=-1]", "\"ello\"", inclusive_positive_range_with_negative_to),
    ("string[1..]", "\"ello\"", unbounded_positive_range),
    ("string[-2..]", "\"lo\"", unbounded_negative_range),
    // Additional tests for negative range indexing
    ("string[0..-1]", "\"hell\"", exclusive_drop_last),
    ("\"ab\"[0..-1]", "\"a\"", exclusive_drop_last_short),
    ("\"a\"[0..-1]", "\"\"", exclusive_drop_last_single_char),
    ("string[-3..-1]", "\"ll\"", exclusive_negative_start_and_end)
}

test_eval! {
    suite string_graphemes;

    // Grapheme cluster indexing - complex emoji should be single graphemes
    ("\"👨‍👩‍👧‍👦\"[0]", "\"👨‍👩‍👧‍👦\"", family_emoji_index_zero),
    ("\"🇬🇧\"[0]", "\"🇬🇧\"", flag_emoji_index_zero),
    ("\"a👨‍👩‍👧‍👦b\"[1]", "\"👨‍👩‍👧‍👦\"", emoji_in_middle),
    ("\"a👨‍👩‍👧‍👦b\"[0]", "\"a\"", char_before_emoji),
    ("\"a👨‍👩‍👧‍👦b\"[2]", "\"b\"", char_after_emoji),
    ("\"a👨‍👩‍👧‍👦b\"[-1]", "\"b\"", negative_index_with_emoji),
    ("\"a👨‍👩‍👧‍👦b\"[-2]", "\"👨‍👩‍👧‍👦\"", negative_index_on_emoji),
    // Combining characters
    ("\"é\"[0]", "\"é\"", combining_accent_single_grapheme),
    ("\"café\"[3]", "\"é\"", combining_accent_in_word)
}

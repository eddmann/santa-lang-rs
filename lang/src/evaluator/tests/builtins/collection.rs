test_eval! {
    suite push;

    ("push(1, [])", "[1]", empty_list),
    ("push(3, [1, 2])", "[1, 2, 3]", list_with_elements),
    ("push(1, {})", "{1}", empty_set),
    ("push(3, {1, 2})", "{1, 2, 3}", set_with_elements)
}

test_eval! {
    suite size;

    ("size([])", "0", empty_list),
    ("size([1, 2])", "2", list_with_elements),
    ("size({})", "0", empty_set),
    ("size({1, 2})", "2", set_with_elements),
    ("size(#{})", "0", empty_hash),
    ("size(#{1: 2, 3: 4})", "2", hash_with_elements),
    ("size(\"\")", "0", empty_string),
    ("size(\"ab\")", "2", string_with_characters),
    ("size(0..0)", "0", empty_lazy_sequence),
    ("size(0..2)", "2", lazy_sequence_with_elements)
}

test_eval! {
    suite map;

    ("map(_ + 1, [])", "[]", empty_list),
    ("map(_ + 1, [1, 2])", "[2, 3]", list_with_elements),
    ("map(_ + 1, {})", "{}", empty_set),
    ("map(_ + 1, {1, 2})", "{2, 3}", set_with_elements),
    ("map(_ + 1, #{})", "#{}", empty_hash),
    ("map(_ + 1, #{1: 2, 3: 4})", "#{1: 3, 3: 5}", hash_with_elements),
    ("map(_ * 2, \"\")", "[]", empty_string),
    ("map(_ * 2, \"ab\")", "[\"aa\", \"bb\"]", string_with_characters),
    ("map(_ + 1, 0..0) |> list", "[]", empty_lazy_sequence),
    ("map(_ + 1, 0..2) |> list", "[1, 2]", lazy_sequence_with_elements)
}

test_eval! {
    suite filter;

    ("filter(_ == 1, [])", "[]", empty_list),
    ("filter(_ == 1, [1, 2])", "[1]", list_with_elements),
    ("filter(_ == 1, {})", "{}", empty_set),
    ("filter(_ == 1, {1, 2})", "{1}", set_with_elements),
    ("filter(_ == 1, #{})", "#{}", empty_hash),
    ("filter(_ == 2, #{1: 2, 3: 4})", "#{1: 2}", hash_with_elements),
    ("filter(_ == \"a\", \"\")", "[]", empty_string),
    ("filter(_ == \"a\", \"ab\")", "[\"a\"]", string_with_characters),
    ("filter(_ == 0, 0..0) |> list", "[]", empty_lazy_sequence),
    ("filter(_ == 0, 0..2) |> list", "[0]", lazy_sequence_with_elements)
}

test_eval! {
    suite fold;

    ("fold(0, +, [])", "0", empty_list),
    ("fold(0, +, [1, 2])", "3", list_with_elements),
    ("fold(0, +, {})", "0", empty_set),
    ("fold(0, +, {1, 2})", "3", set_with_elements),
    ("fold(0, +, #{})", "0", empty_hash),
    ("fold(0, +, #{1: 2, 3: 4})", "6", hash_with_elements),
    ("fold(0, _ + 1, \"\")", "0", empty_string),
    ("fold(0, _ + 1, \"ab\")", "2", string_with_characters),
    ("fold(0, +, 0..0)", "0", empty_lazy_sequence),
    ("fold(0, +, 0..2)", "1", lazy_sequence_with_elements),
    ("fold(0, |acc, value| if acc == 10 { break acc } else { acc + value }, 0..)", "10", early_break)
}

test_eval! {
    suite each;

    sut "let mut a = 0;";

    ("each(|v| a = a + v, []); a", "0", empty_list),
    ("each(|v| a = a + v, [1, 2]); a", "3", list_with_elements),
    ("each(|v| a = a + v, {}); a", "0", empty_set),
    ("each(|v| a = a + v, {1, 2}); a", "3", set_with_elements),
    ("each(|v| a = a + v, #{}); a", "0", empty_hash),
    ("each(|v| a = a + v, #{1: 2, 3: 4}); a", "6", hash_with_elements),
    ("each(|_| a = a + 1, \"\"); a", "0", empty_string),
    ("each(|_| a = a + 1, \"ab\"); a", "2", string_with_characters),
    ("each(|v| a = a + v, 0..0); a", "0", empty_lazy_sequence),
    ("each(|v| a = a + v, 0..2); a", "1", lazy_sequence_with_elements),
    ("each(|v| if v == 10 { break nil } else { a = a + v }, 0..); a", "45", early_break)
}

test_eval! {
    suite reduce;

    ("reduce(+, [])", "Unable to reduce an empty List", empty_list),
    ("reduce(+, [1, 2])", "3", list_with_elements),
    ("reduce(+, {})", "Unable to reduce an empty Set", empty_set),
    ("reduce(+, {1, 2})", "3", set_with_elements),
    ("reduce(+, #{})", "Unable to reduce an empty Hash", empty_hash),
    ("reduce(+, #{1: 2, 3: 4})", "6", hash_with_elements),
    ("reduce(+, \"\")", "Unable to reduce an empty String", empty_string),
    ("reduce(+, \"ab\")", "\"ab\"", string_with_characters),
    ("reduce(+, 0..0)", "Unable to reduce an empty LazySequence", empty_lazy_sequence),
    ("reduce(+, 0..2)", "1", lazy_sequence_with_elements),
    ("reduce(|acc, value| if acc == 10 { break acc } else { acc + value }, 0..)", "10", early_break)
}

test_eval! {
    suite flat_map;

    ("flat_map(+, [])", "[]", empty_list),
    ("flat_map(_ * 2, [[1, 2], [3, 4]])", "[1, 2, 1, 2, 3, 4, 3, 4]", list_with_elements)
}

test_eval! {
    suite find;

    ("find(_ == 1, [])", "nil", empty_list),
    ("find(_ == 1, [1, 2])", "1", list_with_elements),
    ("find(_ == 1, {})", "nil", empty_set),
    ("find(_ == 1, {1, 2})", "1", set_with_elements),
    ("find(_ == 1, #{})", "nil", empty_hash),
    ("find(_ == 2, #{1: 2, 3: 4})", "2", hash_with_elements),
    ("find(_ == \"a\", \"\")", "nil", empty_string),
    ("find(_ == \"a\", \"ab\")", "\"a\"", string_with_characters),
    ("find(_ == 1, 0..0)", "nil", empty_lazy_sequence),
    ("find(_ == 1, 0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite count;

    ("count(_ == 1, [])", "0", empty_list),
    ("count(_ == 1, [1, 2])", "1", list_with_elements),
    ("count(_ == 1, {})", "0", empty_set),
    ("count(_ == 1, {1, 2})", "1", set_with_elements),
    ("count(_ == 1, #{})", "0", empty_hash),
    ("count(_ == 2, #{1: 2, 3: 4})", "1", hash_with_elements),
    ("count(_ == \"a\", \"\")", "0", empty_string),
    ("count(_ == \"a\", \"ab\")", "1", string_with_characters),
    ("count(_ == 1, 0..0)", "0", empty_lazy_sequence),
    ("count(_ == 1, 0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite sum;

    ("sum([])", "0", empty_list),
    ("sum([1, 2])", "3", list_with_elements),
    ("sum({})", "0", empty_set),
    ("sum({1, 2})", "3", set_with_elements),
    ("sum(#{})", "0", empty_hash),
    ("sum(#{1: 2, 3: 4})", "6", hash_with_elements),
    ("sum(0..0)", "0", empty_lazy_sequence),
    ("sum(0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite max;

    ("max([])", "nil", empty_list),
    ("max([1, 2])", "2", list_with_elements),
    ("max({})", "nil", empty_set),
    ("max({1, 2})", "2", set_with_elements),
    ("max(#{})", "nil", empty_hash),
    ("max(#{1: 2, 3: 4})", "4", hash_with_elements),
    ("max(0..0)", "nil", empty_lazy_sequence),
    ("max(0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite min;

    ("min([])", "nil", empty_list),
    ("min([1, 2])", "1", list_with_elements),
    ("min({})", "nil", empty_set),
    ("min({1, 2})", "1", set_with_elements),
    ("min(#{})", "nil", empty_hash),
    ("min(#{1: 2, 3: 4})", "2", hash_with_elements),
    ("min(0..0)", "nil", empty_lazy_sequence),
    ("min(0..2)", "0", lazy_sequence_with_elements)
}

test_eval! {
    suite skip;

    ("skip(1, [])", "[]", empty_list),
    ("skip(1, [1, 2, 3])", "[2, 3]", list_with_elements),
    ("skip(1, 0..0) |> list", "[]", empty_lazy_sequence),
    ("skip(1, 1..=3) |> list", "[2, 3]", lazy_sequence_with_elements)
}

test_eval! {
    suite take;

    ("take(2, [])", "[]", empty_list),
    ("take(2, [1, 2, 3])", "[1, 2]", list_with_elements),
    ("take(2, 0..0) |> list", "[]", empty_lazy_sequence),
    ("take(2, 1..=3) |> list", "[1, 2]", lazy_sequence_with_elements)
}

test_eval! {
    suite list;

    ("list([])", "[]", empty_list),
    ("list([1, 2])", "[1, 2]", list_with_elements),
    ("list({})", "[]", empty_set),
    ("list({1, 2})", "[1, 2]", set_with_elements),
    ("list(#{})", "[]", empty_hash),
    ("list(#{1: 2, 3: 4})", "[[1, 2], [3, 4]]", hash_with_elements),
    ("list(\"\")", "[]", empty_string),
    ("list(\"ab\")", "[\"a\", \"b\"]", string_with_characters),
    ("list(0..0)", "[]", empty_lazy_sequence),
    ("list(0..2)", "[0, 1]", lazy_sequence_with_elements)
}

test_eval! {
    suite set;

    ("set([])", "{}", empty_list),
    ("set([1, 2])", "{1, 2}", list_with_elements),
    ("set({})", "{}", empty_set),
    ("set({1, 2})", "{1, 2}", set_with_elements),
    ("set(\"\")", "{}", empty_string),
    ("set(\"ab\")", "{\"a\", \"b\"}", string_with_characters),
    ("set(0..0)", "{}", empty_lazy_sequence),
    ("set(0..2)", "{1, 0}", lazy_sequence_with_elements)
}

test_eval! {
    suite hash;

    ("hash([])", "#{}", empty_list),
    ("hash([[1, 2], [3, 4]])", "#{1: 2, 3: 4}", list_with_elements),
    ("hash(#{})", "#{}", empty_hash),
    ("hash(#{1: 2, 3: 4})", "#{1: 2, 3: 4}", hash_with_elements),
    ("zip(0..0, 0..1) |> hash", "#{}", empty_lazy_sequence),
    ("zip(0..2, 1..3) |> hash", "#{1: 2, 0: 1}", lazy_sequence_with_elements)
}

test_eval! {
    suite repeat;

    ("repeat(1) |> take(3)", "[1, 1, 1]", take),
    ("repeat(1) |> skip(3) |> take(3)", "[1, 1, 1]", skip_take)
}

test_eval! {
    suite cycle;

    ("cycle([1, 2, 3]) |> take(4)", "[1, 2, 3, 1]", list_take),
    ("cycle([1, 2, 3]) |> skip(1) |> take(4)", "[2, 3, 1, 2]", list_skip_take),
    ("cycle(\"abc\") |> take(4)", "[\"a\", \"b\", \"c\", \"a\"]", string_take),
    ("cycle(\"abc\") |> skip(1) |> take(4)", "[\"b\", \"c\", \"a\", \"b\"]", string_skip_take)
}

test_eval! {
    suite iterate;

    ("iterate(|[a, b]| [b, a + b], [0, 1]) |> skip(9) |> take(1)", "[[34, 55]]", fibonacci),
    ("iterate(_ * 2, 1) |> take(5)", "[1, 2, 4, 8, 16]", doubles)
    // TODO break in iterate?
}

test_eval! {
    suite zip;

    ("zip(0.., \"abc\", [1.5, 2.5, 3.5])", "[[0, \"a\", 1.5], [1, \"b\", 2.5], [2, \"c\", 3.5]]", eager_zip_with_same_size_collections),
    ("zip(0.., \"abcdef\", [1.5, 2.5, 3.5])", "[[0, \"a\", 1.5], [1, \"b\", 2.5], [2, \"c\", 3.5]]", eager_zip_with_different_size_collections),
    ("zip(0.. |> filter(_ % 2), \"abc\")", "[[1, \"a\"], [3, \"b\"], [5, \"c\"]]", eager_zip_using_lazy_sequence_with_fns),
    ("zip(0.., 1..) |> take(3)", "[[0, 1], [1, 2], [2, 3]]", lazy_infinite_zip),
    ("zip(0.. |> filter(_ % 2), 1..) |> take(3)", "[[1, 1], [3, 2], [5, 3]]", lazy_infinite_zip_with_fns),
    ("zip(0.., 1..5) |> list", "[[0, 1], [1, 2], [2, 3], [3, 4]]", lazy_finite_zip),
    ("zip(0.. |> filter(_ % 2), 1..5) |> list", "[[1, 1], [3, 2], [5, 3], [7, 4]]", lazy_finite_zip_with_fns),
    ("let x = zip(0.., 1..); [x |> take(2), x |> skip(1) |> take(2)]", "[[[0, 1], [1, 2]], [[1, 2], [2, 3]]]", use_of_same_zip_source)
}

test_eval! {
    suite keys;

    ("keys(#{})", "[]", empty_hash),
    ("keys(#{1: 2, 3: 4})", "[1, 3]", hash_with_elements)
}

test_eval! {
    suite values;

    ("values(#{})", "[]", empty_hash),
    ("values(#{1: 2, 3: 4})", "[2, 4]", hash_with_elements)
}

test_eval! {
    suite first;

    ("first([])", "nil", empty_list),
    ("first([1, 2])", "1", list_with_elements),
    ("first({})", "nil", empty_set),
    ("first({1, 2})", "1", set_with_elements),
    ("first(\"\")", "nil", empty_string),
    ("first(\"ab\")", "\"a\"", string_with_characters),
    ("first(0..0)", "nil", empty_lazy_sequence),
    ("first(0..2)", "0", lazy_sequence_with_elements)
}

test_eval! {
    suite rest;

    ("rest([])", "[]", empty_list),
    ("rest([1, 2])", "[2]", list_with_elements),
    ("rest({})", "{}", empty_set),
    ("rest({1, 2})", "{2}", set_with_elements),
    ("rest(\"\")", "\"\"", empty_string),
    ("rest(\"ab\")", "\"b\"", string_with_characters),
    ("rest(0..0) |> list", "[]", empty_lazy_sequence),
    ("rest(0..2) |> list", "[1]", lazy_sequence_with_elements)
}

test_eval! {
    suite get;

    ("get(1, [])", "nil", empty_list),
    ("get(1, [1, 2])", "2", list_with_elements),
    ("get(1, {})", "nil", empty_set),
    ("get(1, {2, 1})", "1", set_with_elements),
    ("get(1, #{})", "nil", empty_hash),
    ("get(1, #{1: 2, 3: 4})", "2", hash_with_elements),
    ("get(1, \"\")", "nil", empty_string),
    ("get(1, \"ab\")", "\"b\"", string_with_characters),
    ("get(1, 0..0)", "nil", empty_lazy_sequence),
    ("get(1, 0..2)", "1", finite_lazy_sequence),
    ("get(1, 0..)", "1", infinite_lazy_sequence)
}

test_eval! {
    suite includes;

    ("includes?(1, [])", "false", empty_list),
    ("includes?(1, [1, 2])", "true", list_with_elements),
    ("includes?(1, {})", "false", empty_set),
    ("includes?(1, {2, 1})", "true", set_with_elements),
    ("includes?(1, #{})", "false", empty_hash),
    ("includes?(1, #{1: 2, 3: 4})", "true", hash_with_elements),
    ("includes?(\"a\", \"\")", "false", empty_string),
    ("includes?(\"a\", \"ab\")", "true", string_with_characters),
    ("includes?(1, 0..0)", "false", empty_lazy_sequence),
    ("includes?(1, 0..2)", "true", finite_lazy_sequence),
    ("includes?(1, 0..)", "true", infinite_lazy_sequence)
}

test_eval! {
    suite excludes;

    ("excludes?(1, [])", "true", empty_list),
    ("excludes?(1, [1, 2])", "false", list_with_elements),
    ("excludes?(1, {})", "true", empty_set),
    ("excludes?(1, {2, 1})", "false", set_with_elements),
    ("excludes?(1, #{})", "true", empty_hash),
    ("excludes?(1, #{1: 2, 3: 4})", "false", hash_with_elements),
    ("excludes?(\"a\", \"\")", "true", empty_string),
    ("excludes?(\"a\", \"ab\")", "false", string_with_characters),
    ("excludes?(1, 0..0)", "true", empty_lazy_sequence),
    ("excludes?(1, 0..2)", "false", finite_lazy_sequence)
}

test_eval! {
    suite any;

    ("any?(_ == 1, [])", "false", empty_list),
    ("any?(_ == 1, [1, 2])", "true", list_with_elements),
    ("any?(_ == 1, {})", "false", empty_set),
    ("any?(_ == 1, {2, 1})", "true", set_with_elements),
    ("any?(_ == 2, #{})", "false", empty_hash),
    ("any?(_ == 2, #{1: 2, 3: 4})", "true", hash_with_elements),
    ("any?(_ == \"a\", \"\")", "false", empty_string),
    ("any?(_ == \"a\", \"ab\")", "true", string_with_characters),
    ("any?(_ == 1, 0..0)", "false", empty_lazy_sequence),
    ("any?(_ == 1, 0..2)", "true", finite_lazy_sequence)
}

test_eval! {
    suite all;

    ("all?(_ > 0, [])", "true", empty_list),
    ("all?(_ > 0, [1, 2])", "true", list_with_elements),
    ("all?(_ > 0, {})", "true", empty_set),
    ("all?(_ > 0, {2, 1})", "true", set_with_elements),
    ("all?(_ > 0, #{})", "true", empty_hash),
    ("all?(_ > 0, #{1: 2, 3: 4})", "true", hash_with_elements),
    ("all?(_ != \"c\", \"\")", "true", empty_string),
    ("all?(_ != \"c\", \"ab\")", "true", string_with_characters),
    ("all?(_ >= 0, 0..0)", "true", empty_lazy_sequence),
    ("all?(_ >= 0, 0..2)", "true", finite_lazy_sequence)
}

test_eval! {
    suite sort;

    ("sort(>, [])", "[]", empty_list_using_predicate_comparison),
    ("sort(>, [3, 2, 1])", "[1, 2, 3]", unsorted_list_using_predicate_comparison),
    ("sort(>, [1, 2, 3])", "[1, 2, 3]", sorted_list_using_predicate_comparison),
    ("sort(>, [])", "[]", empty_list_using_integer_comparison),
    ("sort(-, [3, 2, 1])", "[1, 2, 3]", unsorted_list_using_integer_comparison),
    ("sort(-, [1, 2, 3])", "[1, 2, 3]", sorted_list_using_integer_comparison)
}

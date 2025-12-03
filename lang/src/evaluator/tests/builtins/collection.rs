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
    ("size(#{})", "0", empty_dictionary),
    ("size(#{1: 2, 3: 4})", "2", dictionary_with_elements),
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
    ("map(_ + 1, #{})", "#{}", empty_dictionary),
    ("map(_ + 1, #{1: 2, 3: 4})", "#{1: 3, 3: 5}", dictionary_with_elements),
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
    ("filter(_ == 1, #{})", "#{}", empty_dictionary),
    ("filter(_ == 2, #{1: 2, 3: 4})", "#{1: 2}", dictionary_with_elements),
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
    ("fold(0, +, #{})", "0", empty_dictionary),
    ("fold(0, +, #{1: 2, 3: 4})", "6", dictionary_with_elements),
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
    ("each(|v| a = a + v, #{}); a", "0", empty_dictionary),
    ("each(|v| a = a + v, #{1: 2, 3: 4}); a", "6", dictionary_with_elements),
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
    ("reduce(+, #{})", "Unable to reduce an empty Dictionary", empty_dictionary),
    ("reduce(+, #{1: 2, 3: 4})", "6", dictionary_with_elements),
    ("reduce(+, \"\")", "Unable to reduce an empty String", empty_string),
    ("reduce(+, \"ab\")", "\"ab\"", string_with_characters),
    ("reduce(+, 0..0)", "Unable to reduce an empty LazySequence", empty_lazy_sequence),
    ("reduce(+, 0..2)", "1", lazy_sequence_with_elements),
    ("reduce(|acc, value| if acc == 10 { break acc } else { acc + value }, 0..)", "10", early_break)
}

test_eval! {
    suite flat_map;

    ("flat_map(+, [])", "[]", empty_list),
    ("flat_map(_ * 2, [[1, 2], [3, 4]])", "[1, 2, 1, 2, 3, 4, 3, 4]", list_with_elements),
    ("0..3 |> flat_map(|x| [x, x])", "[0, 0, 1, 1, 2, 2]", lazy_sequence),
    ("1..=3 |> flat_map(|x| [])", "[]", lazy_sequence_empty_result),
    ("zip(1..3, 4..6) |> flat_map(|[a, b]| [a, b])", "[1, 4, 2, 5]", lazy_sequence_zipped)
}

test_eval! {
    suite find;

    ("find(_ == 1, [])", "nil", empty_list),
    ("find(_ == 1, [1, 2])", "1", list_with_elements),
    ("find(_ == 1, {})", "nil", empty_set),
    ("find(_ == 1, {1, 2})", "1", set_with_elements),
    ("find(_ == 1, #{})", "nil", empty_dictionary),
    ("find(_ == 2, #{1: 2, 3: 4})", "2", dictionary_with_elements),
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
    ("count(_ == 1, #{})", "0", empty_dictionary),
    ("count(_ == 2, #{1: 2, 3: 4})", "1", dictionary_with_elements),
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
    ("sum(#{})", "0", empty_dictionary),
    ("sum(#{1: 2, 3: 4})", "6", dictionary_with_elements),
    ("sum(0..0)", "0", empty_lazy_sequence),
    ("sum(0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite max;

    ("max([])", "nil", empty_list),
    ("max([1, 2])", "2", list_with_elements),
    ("max({})", "nil", empty_set),
    ("max({1, 2})", "2", set_with_elements),
    ("max(#{})", "nil", empty_dictionary),
    ("max(#{1: 2, 3: 4})", "4", dictionary_with_elements),
    ("max(0..0)", "nil", empty_lazy_sequence),
    ("max(0..2)", "1", lazy_sequence_with_elements),
    ("max(1, 2)", "2", multi_argument)
}

test_eval! {
    suite min;

    ("min([])", "nil", empty_list),
    ("min([1, 2])", "1", list_with_elements),
    ("min({})", "nil", empty_set),
    ("min({1, 2})", "1", set_with_elements),
    ("min(#{})", "nil", empty_dictionary),
    ("min(#{1: 2, 3: 4})", "2", dictionary_with_elements),
    ("min(0..0)", "nil", empty_lazy_sequence),
    ("min(0..2)", "0", lazy_sequence_with_elements),
    ("min(1, 2)", "1", multi_argument)
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
    ("list(#{})", "[]", empty_dictionary),
    ("list(#{1: 2, 3: 4})", "[[1, 2], [3, 4]]", dictionary_with_elements),
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
    suite dict;

    ("dict([])", "#{}", empty_list),
    ("dict([[1, 2], [3, 4]])", "#{1: 2, 3: 4}", list_with_elements),
    ("dict(#{})", "#{}", empty_dictionary),
    ("dict(#{1: 2, 3: 4})", "#{1: 2, 3: 4}", dictionary_with_elements),
    ("zip(0..0, 0..1) |> dict", "#{}", empty_lazy_sequence),
    ("zip(0..2, 1..3) |> dict", "#{1: 2, 0: 1}", lazy_sequence_with_elements)
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

    ("keys(#{})", "[]", empty_dictionary),
    ("keys(#{1: 2, 3: 4})", "[1, 3]", dictionary_with_elements)
}

test_eval! {
    suite values;

    ("values(#{})", "[]", empty_dictionary),
    ("values(#{1: 2, 3: 4})", "[2, 4]", dictionary_with_elements)
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
    suite second;

    ("second([])", "nil", empty_list),
    ("second([1, 2])", "2", list_with_elements),
    ("second({})", "nil", empty_set),
    ("second({1, 2})", "2", set_with_elements),
    ("second(\"\")", "nil", empty_string),
    ("second(\"ab\")", "\"b\"", string_with_characters),
    ("second(0..0)", "nil", empty_lazy_sequence),
    ("second(0..2)", "1", lazy_sequence_with_elements)
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
    ("get(1, #{})", "nil", empty_dictionary),
    ("get(1, #{1: 2, 3: 4})", "2", dictionary_with_elements),
    ("get(1, \"\")", "nil", empty_string),
    ("get(1, \"ab\")", "\"b\"", string_with_characters),
    ("get(1, 0..0)", "nil", empty_lazy_sequence),
    ("get(1, 0..2)", "1", finite_lazy_sequence),
    ("get(1, 0..)", "1", infinite_lazy_sequence)
}

test_eval! {
    suite includes;

    ("includes?([], 1)", "false", empty_list),
    ("includes?([1, 2], 1)", "true", list_with_elements),
    ("includes?({}, 1)", "false", empty_set),
    ("includes?({2, 1}, 1)", "true", set_with_elements),
    ("includes?(#{}, 1)", "false", empty_dictionary),
    ("includes?(#{1: 2, 3: 4}, 1)", "true", dictionary_with_elements),
    ("includes?(\"\", \"a\")", "false", empty_string),
    ("includes?(\"ab\", \"a\")", "true", string_with_characters),
    ("includes?(0..0, 1)", "false", empty_lazy_sequence),
    ("includes?(0..2, 1)", "true", finite_lazy_sequence),
    ("includes?(0.., 1)", "true", infinite_lazy_sequence)
}

test_eval! {
    suite excludes;

    ("excludes?([], 1)", "true", empty_list),
    ("excludes?([1, 2], 1)", "false", list_with_elements),
    ("excludes?({}, 1)", "true", empty_set),
    ("excludes?({2, 1}, 1)", "false", set_with_elements),
    ("excludes?(#{}, 1)", "true", empty_dictionary),
    ("excludes?(#{1: 2, 3: 4}, 1)", "false", dictionary_with_elements),
    ("excludes?(\"\", \"a\")", "true", empty_string),
    ("excludes?(\"ab\", \"a\")", "false", string_with_characters),
    ("excludes?(0..0, 1)", "true", empty_lazy_sequence),
    ("excludes?(0..2, 1)", "false", finite_lazy_sequence)
}

test_eval! {
    suite any;

    ("any?(_ == 1, [])", "false", empty_list),
    ("any?(_ == 1, [1, 2])", "true", list_with_elements),
    ("any?(_ == 1, {})", "false", empty_set),
    ("any?(_ == 1, {2, 1})", "true", set_with_elements),
    ("any?(_ == 2, #{})", "false", empty_dictionary),
    ("any?(_ == 2, #{1: 2, 3: 4})", "true", dictionary_with_elements),
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
    ("all?(_ > 0, #{})", "true", empty_dictionary),
    ("all?(_ > 0, #{1: 2, 3: 4})", "true", dictionary_with_elements),
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

test_eval! {
    suite union;

    ("union([{1, 2}, {2, 3}])", "{1, 2, 3}", single_argument_sets),
    ("union([{1, 2}, [2, 3]])", "{1, 2, 3}", single_argument_set_and_list),
    ("union([{1, 2}, 2..=3])", "{1, 2, 3}", single_argument_set_and_lazy_sequence),
    ("union({1, 2}, {2, 3})", "{1, 2, 3}", multi_argument_sets),
    ("union({1, 2}, [2, 3])", "{1, 2, 3}", multi_argument_set_and_list),
    ("union({1, 2}, 2..=3)", "{1, 2, 3}", multi_argument_set_and_lazy_sequence),
    ("union(true, {1, 2})", "Unable to convert a Boolean into an Set", fails_to_convert_argument_into_set),
    ("union({1, 2}, [|| 1])", "Unable to include a Function within an Set", fails_to_convert_unhashable_list_element_into_set)
}

test_eval! {
    suite intersection;

    ("intersection([{1, 2}, {2, 3}])", "{2}", single_argument_sets),
    ("intersection([{1, 2}, [2, 3]])", "{2}", single_argument_set_and_list),
    ("intersection([{1, 2}, 2..=3])", "{2}", single_argument_set_and_lazy_sequence),
    ("intersection({1, 2}, {2, 3})", "{2}", multi_argument_sets),
    ("intersection({1, 2}, [2, 3])", "{2}", multi_argument_set_and_list),
    ("intersection({1, 2}, 2..=3)", "{2}", multi_argument_set_and_lazy_sequence),
    ("intersection(true, {1, 2})", "Unable to convert a Boolean into an Set", fails_to_convert_argument_into_set),
    ("intersection({1, 2}, [|| 1])", "Unable to include a Function within an Set", fails_to_convert_unhashable_list_element_into_set)
}

test_eval! {
    suite scan;

    ("scan(0, +, [])", "[0]", empty_list),
    ("scan(0, +, [1, 2])", "[0, 1, 3]", list_with_elements),
    ("scan(0, +, {})", "[0]", empty_set),
    ("scan(0, +, {1, 2})", "[0, 1, 3]", set_with_elements),
    ("scan(0, +, #{})", "[0]", empty_dictionary),
    ("scan(0, +, #{1: 2, 3: 4})", "[0, 2, 6]", dictionary_with_elements),
    ("scan(\"\", +, \"\")", "[\"\"]", empty_string),
    ("scan(\"\", +, \"ab\")", "[\"\", \"a\", \"ab\"]", string_with_characters),
    ("scan(0, +, 0..0)", "[0]", empty_lazy_sequence),
    ("scan(0, +, 0..2)", "[0, 0, 1]", finite_lazy_sequence)
}

test_eval! {
    suite reverse;

    ("reverse([])", "[]", empty_list),
    ("reverse([1, 2])", "[2, 1]", list_with_elements),
    ("reverse(\"\")", "\"\"", empty_string),
    ("reverse(\"ab\")", "\"ba\"", string_with_characters),
    ("reverse(0..0)", "[]", empty_lazy_sequence),
    ("reverse(0..2)", "[1, 0]", finite_lazy_sequence)
}

test_eval! {
    suite filter_map;

    ("filter_map(|a| if a != 1 { a + 1 }, [])", "[]", empty_list),
    ("filter_map(|a| if a != 1 { a + 1 }, [1, 2])", "[3]", list_with_elements),
    ("filter_map(|a| if a != 1 { a + 1 }, {})", "{}", empty_set),
    ("filter_map(|a| if a != 1 { a + 1 }, {1, 2})", "{3}", set_with_elements),
    ("filter_map(|a| if a != 1 { a + 1 }, #{})", "#{}", empty_dictionary),
    ("filter_map(|a| if a != 2 { a + 1 }, #{1: 2, 3: 4})", "#{3: 5}", dictionary_with_elements),
    ("filter_map(|a| if a != \"a\" { a * 2 }, \"\")", "[]", empty_string),
    ("filter_map(|a| if a != \"a\" { a * 2 }, \"ab\")", "[\"bb\"]", string_with_characters),
    ("filter_map(|a| if a != 1 { a + 1 }, 0..0) |> list", "[]", empty_lazy_sequence),
    ("filter_map(|a| if a != 1 { a + 1 }, 0..2) |> list", "[1]", bounded_lazy_sequence_with_elements),
    ("filter_map(|a| if a != 1 { a + 1 }, 0..) |> take(3)", "[1, 3, 4]", unbounded_lazy_sequence_with_elements)
}

test_eval! {
    suite find_map;

    ("find_map(|a| if a != 1 { a + 1 }, [])", "nil", empty_list),
    ("find_map(|a| if a != 1 { a + 1 }, [1, 2])", "3", list_with_elements),
    ("find_map(|a| if a != 1 { a + 1 }, {})", "nil", empty_set),
    ("find_map(|a| if a != 1 { a + 1 }, {1, 2})", "3", set_with_elements),
    ("find_map(|a| if a != 1 { a + 1 }, #{})", "nil", empty_dictionary),
    ("find_map(|a| if a != 2 { a + 1 }, #{1: 2, 3: 4})", "5", dictionary_with_elements),
    ("find_map(|a| if a != \"a\" { a * 2 }, \"\")", "nil", empty_string),
    ("find_map(|a| if a != \"a\" { a * 2 }, \"ab\")", "\"bb\"", string_with_characters),
    ("find_map(|a| if a != 1 { a + 1 }, 0..0)", "nil", empty_lazy_sequence),
    ("find_map(|a| if a != 1 { a + 1 }, 0..2)", "1", lazy_sequence_with_elements)
}

test_eval! {
    suite assoc;

    ("assoc(0, 1, [])", "[1]", empty_list),
    ("assoc(1, 1, [])", "[nil, 1]", empty_list_second_index),
    ("assoc(0, 3, [1, 2])", "[3, 2]", list_with_existing_element),
    ("assoc(0, 1, #{})", "#{0: 1}", empty_dictionary),
    ("assoc(1, 1, #{1: 2, 3: 4})", "#{1: 1, 3: 4}", dictionary_with_existing_entry),
    ("assoc(0, 1, #{1: 2, 3: 4})", "#{1: 2, 3: 4, 0: 1}", dictionary_with_new_entry)
}

test_eval! {
    suite update;

    ("update(0, || 1, [])", "[1]", empty_list),
    ("update(1, || 1, [])", "[nil, 1]", empty_list_non_zero_index),
    ("update(0, _ + 1, [1, 2])", "[2, 2]", list_with_existing_element),
    ("update(0, || 1, #{})", "#{0: 1}", empty_dictionary),
    ("update(1, _ + 1, #{1: 2, 3: 4})", "#{1: 3, 3: 4}", dictionary_with_existing_entry)
}

test_eval! {
    suite update_d;

    ("update_d(0, 0, _ + 1, [])", "[1]", empty_list),
    ("update_d(1, 0, _ + 1, [])", "[nil, 1]", empty_list_non_zero_index),
    ("update_d(0, 0, _ + 1, [1, 2])", "[2, 2]", list_with_existing_element),
    ("update_d(0, 0, _ + 1, #{})", "#{0: 1}", empty_dictionary),
    ("update_d(1, 0, _ + 1, #{1: 2, 3: 4})", "#{1: 3, 3: 4}", dictionary_with_existing_entry)
}

test_eval! {
    suite fold_s;

    sut "let folder = |[acc, prev], val| [acc + prev * val, val]";

    ("fold_s([0, 1], folder, [])", "0", empty_list),
    ("fold_s([0, 1], folder, [1, 4, 3, 2])", "23", list_with_elements),
    ("fold_s([0, 1], folder, {})", "0", empty_set),
    ("fold_s([0, 1], folder, {1, 4, 3, 2})", "23", set_with_elements),
    ("fold_s([0, 1], folder, #{})", "0", empty_dictionary),
    ("fold_s([0, 1], folder, #{1: 2, 3: 4})", "10", dictionary_with_elements),
    ("fold_s([\"\", \"\"], |[acc, prev], val| [acc + prev + val, val], \"\")", "\"\"", empty_string),
    ("fold_s([\"\", \"a\"], |[acc, prev], val| [acc + prev + val, val], \"ab\")", "\"aaab\"", string_with_characters),
    ("fold_s([0, 1], folder, 0..0)", "0", empty_lazy_sequence),
    ("fold_s([0, 1], folder, 0..=10)", "330", lazy_sequence_with_elements),
    ("fold_s([0, 0, 0], |[acc, x, y], val| [acc + x * y * val, val, val / 2], 1..=10)", "1060", multi_state)
}

test_eval! {
    suite rotate;

    ("rotate(2, [1, 2, 3])", "[2, 3, 1]", positive_step_in_bounds),
    ("rotate(6, [1, 2, 3])", "[1, 2, 3]", positive_step_out_of_bounds),
    ("rotate(-2, [1, 2, 3])", "[3, 1, 2]", negative_step_in_bounds),
    ("rotate(-6, [1, 2, 3])", "[1, 2, 3]", negative_step_out_of_bounds)
}

test_eval! {
    suite chunk;

    ("chunk(2, [])", "[]", empty_list),
    ("chunk(2, [1, 2, 3])", "[[1, 2], [3]]", list_with_odd_amount_of_times),
    ("chunk(2, [1, 2, 3, 4])", "[[1, 2], [3, 4]]", list_with_even_amount_of_times),
    ("chunk(5, [1, 2, 3, 4])", "[[1, 2, 3, 4]]", list_with_less_items_than_the_chunk),
    ("chunk(2, \"hello\")", "[[\"h\", \"e\"], [\"l\", \"l\"], [\"o\"]]", string)
}

test_eval! {
    suite combinations;

    ("combinations(1, []) |> list", "[]", empty_list),
    ("combinations(2, []) |> list", "[]", empty_list_size_two),
    ("combinations(1, [1, 2, 3, 4, 5]) |> list", "[[1], [2], [3], [4], [5]]", one_element),
    ("combinations(2, [1, 2, 3, 4, 5]) |> list", "[[1, 2], [1, 3], [1, 4], [1, 5], [2, 3], [2, 4], [2, 5], [3, 4], [3, 5], [4, 5]]", two_elements),
    ("combinations(5, [1, 2, 3, 4, 5]) |> list", "[[1, 2, 3, 4, 5]]", one_combination),
    ("combinations(6, [1, 2, 3, 4, 5]) |> list", "[]", exhausted_elements),
    // Large list tests - verify no overflow (C(20,2) = 190 combinations)
    ("combinations(2, 1..=20 |> list) |> size", "190", large_list_size_two),
    // C(15,3) = 455 combinations
    ("combinations(3, 1..=15 |> list) |> size", "455", large_list_size_three),
    // Verify first and last combination of large list
    ("combinations(2, 1..=20 |> list) |> first", "[1, 2]", large_list_first_combination),
    ("combinations(2, 1..=20 |> list) |> list |> |l| l[-1]", "[19, 20]", large_list_last_combination)
}

test_eval! {
    suite primitives;

    sut r#"
        let sut = |x| match x {
            "1" { "1" },
            2 { "2" },
            3.5 { "3" },
            x { ["4", x] }
        };
    "#;

    ("sut(\"1\")", "\"1\"", string),
    ("sut(2)", "\"2\"", integer),
    ("sut(3.5)", "\"3\"", decimal),
    ("sut([1, 2, 3])", "[\"4\", [1, 2, 3]]", list)
}

test_eval! {
    suite lists;

    sut r#"
        let sut = |x| match x {
            [] { ["1"] }
            [[1], [x], [y]] { ["2", 1, [x], [y]] }
            [[x], [y], [z]] { ["3", [x], [y], [z]] }
            [1] { ["4", 1] }
            [x] { ["5", x] }
            [1, x] { ["6", 1, x] }
            [x, y] { ["7", x, y] }
            [x, 1, ..y] { ["8", x, 1, y] }
            [x, y, ..z] { ["9", x, y, z] }
            _ { "10" }
        };
    "#;

    ("sut([])", "[\"1\"]", empty),
    ("sut([[1], [2], [3]])", "[\"2\", 1, [2], [3]]", multi_level_with_literal),
    ("sut([[2], [3], [4]])", "[\"3\", [2], [3], [4]]", multi_level_with_identifiers),
    ("sut([1])", "[\"4\", 1]", single_element_with_literal),
    ("sut([2])", "[\"5\", 2]", single_element_with_identifier),
    ("sut([1, 2])", "[\"6\", 1, 2]", multi_element_with_literal),
    ("sut([2, 3])", "[\"7\", 2, 3]", multi_element_with_identifiers),
    ("sut([2, 1, 3, 4])", "[\"8\", 2, 1, [3, 4]]", multi_element_with_literal_and_rest),
    ("sut([1, 2, 3, 4])", "[\"9\", 1, 2, [3, 4]]", multi_element_with_rest),
    ("sut(\"\")", "\"10\"", catch_all)
}

test_eval! {
    suite ranges;

    sut r#"
        let sut = |x| match x {
            [2..4, 1] { "1" },
            [2.., -1] { "2" },
            2..4 { "3" }
            2.. { "4" }
        };
    "#;

    ("sut([3, 1])", "\"1\"", list_with_exclusive_range),
    ("sut([5, -1])", "\"2\"", list_with_unbounded_range),
    ("sut(2)", "\"3\"", exclusive_range),
    ("sut(5)", "\"4\"", unbounded_range)
}

test_eval! {
    suite guards;

    sut r#"
        let sut = |x| match x {
            [x] if x == 10 { "1" }
            [x, y] if x < y { ["2", x, y] }
            [x, y, ..z] if size(z) == 2 { ["3", x, y, z] }
            x if x > 10 { ["4", x] }
            _ { ["5"] }
        };
    "#;

    ("sut([10])", "\"1\"", single_element_list),
    ("sut([1, 2])", "[\"2\", 1, 2]", multi_element_list),
    ("sut([1, 2, 3, 4])", "[\"3\", 1, 2, [3, 4]]", function_call),
    ("sut(15)", "[\"4\", 15]", identifier),
    ("sut(5)", "[\"5\"]", catch_all)
}

test_eval! {
    suite prefix_rest;

    sut r#"
        let sut = |x| match x {
            [..init, last] { ["prefix", init, last] }
            _ { "no match" }
        };
    "#;

    ("sut([1, 2, 3, 4])", "[\"prefix\", [1, 2, 3], 4]", four_elements),
    ("sut([1, 2])", "[\"prefix\", [1], 2]", two_elements),
    ("sut([1])", "[\"prefix\", [], 1]", one_element),
    ("sut([])", "\"no match\"", empty_list)
}

test_eval! {
    suite prefix_rest_with_nested_destructure;

    sut r#"
        let sut = |x| match x {
            [..init, [a, b]] { ["nested", init, a, b] }
            _ { "no match" }
        };
    "#;

    ("sut([[1, 2], [3, 4], [5, 6]])", "[\"nested\", [[1, 2], [3, 4]], 5, 6]", three_pairs),
    ("sut([[1, 2]])", "[\"nested\", [], 1, 2]", single_pair),
    ("sut([[1, 2, 3]])", "\"no match\"", wrong_nested_size),
    ("sut([1, 2, 3])", "\"no match\"", non_list_last_element)
}

test_eval! {
    suite prefix_rest_with_guard;

    sut r#"
        let sut = |x| match x {
            [..init, [a, b]] if a < b { ["ascending", init, a, b] }
            [..init, [a, b]] if a > b { ["descending", init, a, b] }
            [..init, last] { ["other", init, last] }
            _ { "no match" }
        };
    "#;

    ("sut([[1, 2], [3, 5]])", "[\"ascending\", [[1, 2]], 3, 5]", ascending_guard),
    ("sut([[1, 2], [5, 3]])", "[\"descending\", [[1, 2]], 5, 3]", descending_guard),
    ("sut([[1, 2], [3, 3]])", "[\"other\", [[1, 2]], [3, 3]]", equal_falls_through),
    ("sut([1, 2, 3])", "[\"other\", [1, 2], 3]", simple_last)
}

test_eval! {
    suite middle_rest;

    sut r#"
        let sut = |x| match x {
            [first, ..middle, last] { ["middle", first, middle, last] }
            _ { "no match" }
        };
    "#;

    ("sut([1, 2, 3, 4, 5])", "[\"middle\", 1, [2, 3, 4], 5]", five_elements),
    ("sut([1, 2, 3])", "[\"middle\", 1, [2], 3]", three_elements),
    ("sut([1, 2])", "[\"middle\", 1, [], 2]", two_elements),
    ("sut([1])", "\"no match\"", one_element),
    ("sut([])", "\"no match\"", empty_list)
}

test_eval! {
    suite middle_rest_multiple_after;

    sut r#"
        let sut = |x| match x {
            [first, ..middle, second_last, last] { ["multi", first, middle, second_last, last] }
            _ { "no match" }
        };
    "#;

    ("sut([1, 2, 3, 4, 5])", "[\"multi\", 1, [2, 3], 4, 5]", five_elements),
    ("sut([1, 2, 3])", "[\"multi\", 1, [], 2, 3]", three_elements),
    ("sut([1, 2])", "\"no match\"", two_elements),
    ("sut([1])", "\"no match\"", one_element)
}

test_eval! {
    suite edge_cases;

    (
        r#"
            match "unknown" {
                "hello" { "1" }
                1 { "2" }
                2.0 { "3" }
            };
        "#,
        "nil",
        unexhaustive_match_returns_nil
    ),
    (
        r#"
            match -1 {
                -1 { "-" }
                1 { "+" }
            };
        "#,
        "\"-\"",
        negative_value
    )
}

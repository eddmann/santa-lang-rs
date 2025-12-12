test_eval! {
    suite assigment;

    ("let x = 1; x", "1", let_assignment),
    ("let mut x = 1; x = 2; x", "2", mutable_let_assignment),
    ("let x = 1; let x = 2;", "Variable 'x' has already been declared", unable_to_declare_variable_more_than_once),
    ("let x = 1; x = 2;", "Variable 'x' is not mutable", unable_to_assign_variable_which_is_not_mutable),
    ("x = 2", "Variable 'x' has not been declared", unable_to_assign_variable_which_has_not_been_declared),
    (
        r#"
            let [a, b, [c, d, [e]], ..f] = [1, 2, [3, 4, [5]], 6, 7];
            [a, b, c, d, e, f]
        "#,
        "[1, 2, 3, 4, 5, [6, 7]]",
        let_list_destructing
    ),
    (
        r#"
            let mut [x, y, ..z] = [1, 2, 3, 4];
            x = 100;
            [x, y, z]
        "#,
        "[100, 2, [3, 4]]",
        mutable_let_list_destructing
    ),
    (
        r#"
            let [_, _, value] = [1, 2, 3];
            value
        "#,
        "3",
        ignored_placeholders_within_let_list_destructing
    ),
    (
        r#"
            let total = 1;
            let fn = || { let total = 2; total; };
            [total, fn()];
        "#,
        "[1, 2]",
        scoped_let_redeclaration
    ),
    (
        r#"
            let mut total = 1;
            let fn = || { total = total + 1; };
            fn();
            total;
        "#,
        "2",
        mutable_parent_variable_is_modified_from_within_function
    )
}

test_eval! {
    suite dictionary_destructuring;

    // Basic shorthand destructuring
    (
        r#"
            let #{name, age} = #{"name": "Alice", "age": 30};
            [name, age]
        "#,
        "[\"Alice\", 30]",
        shorthand_destructuring
    ),
    // Explicit key binding
    (
        r#"
            let #{"name": n, "age": a} = #{"name": "Bob", "age": 25};
            [n, a]
        "#,
        "[\"Bob\", 25]",
        explicit_key_binding
    ),
    // Rest pattern captures remaining keys (test key existence, not order)
    (
        r#"
            let #{name, ..rest} = #{"name": "Charlie", "age": 35, "city": "NYC"};
            [name, rest["age"], rest["city"]]
        "#,
        "[\"Charlie\", 35, \"NYC\"]",
        rest_pattern
    ),
    // Subset matching - extra keys allowed
    (
        r#"
            let #{name} = #{"name": "Diana", "age": 40, "extra": "value"};
            name
        "#,
        "\"Diana\"",
        subset_matching_allows_extra_keys
    ),
    // Placeholder ignores value
    (
        r#"
            let #{name, "age": _} = #{"name": "Eve", "age": 50};
            name
        "#,
        "\"Eve\"",
        placeholder_ignores_value
    ),
    // Nested list destructuring inside dictionary
    (
        r#"
            let #{"coords": [x, y]} = #{"coords": [10, 20]};
            [x, y]
        "#,
        "[10, 20]",
        nested_list_destructuring
    ),
    // Nested dictionary destructuring
    (
        r#"
            let #{"person": #{name, age}} = #{"person": #{"name": "Frank", "age": 60}};
            [name, age]
        "#,
        "[\"Frank\", 60]",
        nested_dictionary_destructuring
    ),
    // Mutable dictionary destructuring
    (
        r#"
            let mut #{x, y} = #{"x": 1, "y": 2};
            x = 100;
            [x, y]
        "#,
        "[100, 2]",
        mutable_dictionary_destructuring
    ),
    // Missing key returns nil (subset matching semantics)
    (
        r#"
            let #{missing} = #{"name": "Ghost"};
            missing
        "#,
        "nil",
        missing_key_returns_nil
    ),
    // Error: non-dictionary subject
    (
        r#"
            let #{name} = [1, 2, 3];
            name
        "#,
        "Expected a Dictionary to destructure, found: List",
        non_dictionary_error
    ),
    // Rest with explicit keys combined
    (
        r#"
            let #{"a": x, ..rest} = #{"a": 1, "b": 2, "c": 3};
            [x, rest["b"], rest["c"]]
        "#,
        "[1, 2, 3]",
        rest_with_explicit_keys
    ),
    // Deeply nested (3 levels)
    (
        r#"
            let #{outer: #{middle: #{inner}}} = #{"outer": #{"middle": #{"inner": 42}}};
            inner
        "#,
        "42",
        deeply_nested_three_levels
    )
}

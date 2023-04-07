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
    suite spread;

    (
        r#"
            let x = [2, 3];
            [1, ..x, ..["a", "b", "c"]];
        "#,
        "[1, 2, 3, \"a\", \"b\", \"c\"]",
        list
    ),
    (
        r#"
            let add = |x, y| { x + y };
            let xs = [3, 4]
            add(..xs);
        "#,
        "7",
        call
    )
}

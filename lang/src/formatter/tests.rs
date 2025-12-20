use super::format;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn format_integer() {
    assert_eq!(format("42").unwrap(), "42\n");
}

#[test]
fn format_integer_with_underscores() {
    assert_eq!(format("1_000_000").unwrap(), "1_000_000\n");
}

#[test]
fn format_decimal() {
    assert_eq!(format("3.14").unwrap(), "3.14\n");
}

#[test]
fn format_decimal_with_underscores() {
    assert_eq!(format("1_000.50").unwrap(), "1_000.50\n");
}

#[test]
fn format_string_simple() {
    assert_eq!(format(r#""hello""#).unwrap(), "\"hello\"\n");
}

#[test]
fn format_string_escapes_newlines() {
    assert_eq!(format(r#""a\nb""#).unwrap(), "\"a\\nb\"\n");
}

#[test]
fn format_string_with_tab() {
    assert_eq!(format(r#""a\tb""#).unwrap(), "\"a\\tb\"\n");
}

#[test]
fn format_string_with_quote() {
    assert_eq!(format(r#""a\"b""#).unwrap(), "\"a\\\"b\"\n");
}

#[test]
fn format_string_with_backslash() {
    assert_eq!(format(r#""a\\b""#).unwrap(), "\"a\\\\b\"\n");
}

#[test]
fn format_string_with_carriage_return() {
    assert_eq!(format(r#""a\rb""#).unwrap(), "\"a\\rb\"\n");
}

#[test]
fn format_string_with_backspace() {
    assert_eq!(format("\"a\x08b\"").unwrap(), "\"a\\bb\"\n");
}

#[test]
fn format_string_with_form_feed() {
    assert_eq!(format("\"a\x0Cb\"").unwrap(), "\"a\\fb\"\n");
}

#[test]
fn format_string_short_escapes_newlines() {
    assert_eq!(format("\"line1\nline2\"").unwrap(), "\"line1\\nline2\"\n");
}

#[test]
fn format_string_two_newlines_still_escapes() {
    assert_eq!(format("\"a\nb\nc\"").unwrap(), "\"a\\nb\\nc\"\n");
}

#[test]
fn format_string_three_newlines_short_escapes() {
    // 3 newlines is NOT > 3, so escapes (protects patterns like "#\n#\n#\n#")
    let result = format("\"a\nb\nc\nd\"").unwrap();
    assert_eq!(result, "\"a\\nb\\nc\\nd\"\n");
}

#[test]
fn format_string_four_newlines_short_preserves_literal() {
    // 4 newlines IS > 3, so preserves literal newlines even for short strings
    let result = format("\"a\nb\nc\nd\ne\"").unwrap();
    assert!(result.matches('\n').count() > 1);
}

#[test]
fn format_string_long_few_newlines_preserves_literal() {
    // >50 chars triggers literal newlines regardless of newline count
    let long_string = format!("\"{}\\n{}\"", "x".repeat(30), "y".repeat(25));
    let result = format(&long_string).unwrap();
    assert!(result.matches('\n').count() > 1);
}

#[test]
fn format_string_short_few_newlines_escapes() {
    // <50 chars AND <=3 newlines should escape
    assert_eq!(format("\"hello\\nworld\"").unwrap(), "\"hello\\nworld\"\n");
}

#[test]
fn format_boolean_true() {
    assert_eq!(format("true").unwrap(), "true\n");
}

#[test]
fn format_boolean_false() {
    assert_eq!(format("false").unwrap(), "false\n");
}

#[test]
fn format_nil() {
    assert_eq!(format("nil").unwrap(), "nil\n");
}

#[test]
fn format_placeholder() {
    assert_eq!(format("_").unwrap(), "_\n");
}

#[test]
fn format_infix_plus() {
    assert_eq!(format("1+2").unwrap(), "1 + 2\n");
}

#[test]
fn format_infix_minus() {
    assert_eq!(format("1-2").unwrap(), "1 - 2\n");
}

#[test]
fn format_infix_multiply() {
    assert_eq!(format("1*2").unwrap(), "1 * 2\n");
}

#[test]
fn format_infix_divide() {
    assert_eq!(format("1/2").unwrap(), "1 / 2\n");
}

#[test]
fn format_infix_modulo() {
    assert_eq!(format("1%2").unwrap(), "1 % 2\n");
}

#[test]
fn format_infix_equal() {
    assert_eq!(format("1==2").unwrap(), "1 == 2\n");
}

#[test]
fn format_infix_not_equal() {
    assert_eq!(format("1!=2").unwrap(), "1 != 2\n");
}

#[test]
fn format_infix_less_than() {
    assert_eq!(format("1<2").unwrap(), "1 < 2\n");
}

#[test]
fn format_infix_less_than_equal() {
    assert_eq!(format("1<=2").unwrap(), "1 <= 2\n");
}

#[test]
fn format_infix_greater_than() {
    assert_eq!(format("1>2").unwrap(), "1 > 2\n");
}

#[test]
fn format_infix_greater_than_equal() {
    assert_eq!(format("1>=2").unwrap(), "1 >= 2\n");
}

#[test]
fn format_infix_and() {
    assert_eq!(format("true&&false").unwrap(), "true && false\n");
}

#[test]
fn format_infix_or() {
    assert_eq!(format("true||false").unwrap(), "true || false\n");
}

#[test]
fn format_prefix_bang() {
    assert_eq!(format("!true").unwrap(), "!true\n");
}

#[test]
fn format_prefix_bang_removes_space() {
    assert_eq!(format("! true").unwrap(), "!true\n");
}

#[test]
fn format_prefix_minus() {
    assert_eq!(format("-42").unwrap(), "-42\n");
}

#[test]
fn format_prefix_minus_with_infix_preserves_parens() {
    // Critical: -(a + b) must NOT become -a + b (different semantics)
    assert_eq!(format("-(a + b)").unwrap(), "-(a + b)\n");
}

#[test]
fn format_prefix_minus_without_parens_stays_flat() {
    // -a + b should stay as -a + b (no unnecessary parens added)
    assert_eq!(format("-a + b").unwrap(), "-a + b\n");
}

#[test]
fn format_prefix_bang_with_infix_preserves_parens() {
    // !(a && b) must preserve parens
    assert_eq!(format("!(a && b)").unwrap(), "!(a && b)\n");
}

#[test]
fn format_prefix_with_function_thread_preserves_parens() {
    // -(a |> b) must preserve parens
    assert_eq!(format("-(a |> b)").unwrap(), "-(a |> b)\n");
}

#[test]
fn format_chained_operators() {
    assert_eq!(format("1+2*3").unwrap(), "1 + 2 * 3\n");
}

#[test]
fn format_backtick_operator() {
    assert_eq!(format("1 `add` 2").unwrap(), "1 `add` 2\n");
}

#[test]
fn format_empty_list() {
    assert_eq!(format("[]").unwrap(), "[]\n");
}

#[test]
fn format_list_short() {
    assert_eq!(format("[1,2,3]").unwrap(), "[1, 2, 3]\n");
}

#[test]
fn format_empty_set() {
    assert_eq!(format("{}").unwrap(), "{}\n");
}

#[test]
fn format_set_short() {
    assert_eq!(format("{1,2,3}").unwrap(), "{1, 2, 3}\n");
}

#[test]
fn format_empty_dict() {
    assert_eq!(format("#{}").unwrap(), "#{}\n");
}

#[test]
fn format_dict_short() {
    assert_eq!(format("#{a:1,b:2}").unwrap(), "#{a: 1, b: 2}\n");
}

#[test]
fn format_nested_collections() {
    assert_eq!(format("[[1,2],[3,4]]").unwrap(), "[[1, 2], [3, 4]]\n");
}

#[test]
fn format_list_long_formats_correctly() {
    assert_eq!(
        format("[very_long_name_one, very_long_name_two, very_long_name_three]").unwrap(),
        "[very_long_name_one, very_long_name_two, very_long_name_three]\n"
    );
}

#[test]
fn format_list_exceeding_line_width_wraps() {
    // This list exceeds 100 chars so it should wrap
    let input =
        "[very_long_name_one, very_long_name_two, very_long_name_three, very_long_name_four, very_long_name_five]";
    let output = format(input).unwrap();
    // Should wrap to multiple lines
    assert!(output.contains('\n'), "Long list should wrap to multiple lines");
    // Should have trailing comma when multiline
    assert!(output.contains(",\n"), "Should have comma before newline");
}

#[test]
fn format_call_exceeding_line_width_wraps() {
    // Long function call should wrap
    let input = "very_long_function_name(argument_one, argument_two, argument_three, argument_four, argument_five)";
    let output = format(input).unwrap();
    // Should wrap to multiple lines
    assert!(output.contains('\n'), "Long function call should wrap");
}

#[test]
fn format_wrapped_list_has_trailing_comma() {
    // Short list should stay inline (only has trailing newline, not internal newlines)
    let short_output = format("[1, 2, 3]").unwrap();
    let short_lines: Vec<&str> = short_output.trim().lines().collect();
    assert_eq!(
        short_lines.len(),
        1,
        "Short list should be single line: {:?}",
        short_lines
    );

    // Very long list should wrap
    let long_items: Vec<String> = (0..20).map(|i| format!("element_{}", i)).collect();
    let long_input = format!("[{}]", long_items.join(", "));
    let long_output = format(&long_input).unwrap();

    assert_eq!(
        long_output,
        "[\n  element_0,\n  element_1,\n  element_2,\n  element_3,\n  element_4,\n  element_5,\n  element_6,\n  element_7,\n  element_8,\n  element_9,\n  element_10,\n  element_11,\n  element_12,\n  element_13,\n  element_14,\n  element_15,\n  element_16,\n  element_17,\n  element_18,\n  element_19\n]\n"
    );
}

#[test]
fn format_dict_with_string_keys() {
    assert_eq!(format(r#"#{"a":1,"b":2}"#).unwrap(), "#{\"a\": 1, \"b\": 2}\n");
}

#[test]
fn format_range_exclusive() {
    assert_eq!(format("1..10").unwrap(), "1..10\n");
}

#[test]
fn format_range_inclusive() {
    assert_eq!(format("1..=10").unwrap(), "1..=10\n");
}

#[test]
fn format_range_unbounded() {
    assert_eq!(format("1..").unwrap(), "1..\n");
}

#[test]
fn format_lambda_no_params() {
    assert_eq!(format("|| 42").unwrap(), "|| 42\n");
}

#[test]
fn format_lambda_single_param() {
    assert_eq!(format("|x|x+1").unwrap(), "|x| x + 1\n");
}

#[test]
fn format_lambda_multi_param() {
    assert_eq!(format("|x,y|x+y").unwrap(), "|x, y| x + y\n");
}

#[test]
fn format_lambda_with_block() {
    assert_eq!(
        format("|x| { let y = x + 1; y }").unwrap(),
        "|x| {\n  let y = x + 1;\n\n  y\n}\n"
    );
}

#[test]
fn format_call_no_args() {
    assert_eq!(format("f()").unwrap(), "f()\n");
}

#[test]
fn format_call_single_arg() {
    assert_eq!(format("f(1)").unwrap(), "f(1)\n");
}

#[test]
fn format_call_multi_args() {
    assert_eq!(format("f(1,2,3)").unwrap(), "f(1, 2, 3)\n");
}

#[test]
fn format_call_nested() {
    assert_eq!(format("f(g(x))").unwrap(), "f(g(x))\n");
}

#[test]
fn format_trailing_closure_preserved() {
    assert_eq!(
        format("each |x| { let y = x + 1\nputs(y) }").unwrap(),
        "each |x| {\n  let y = x + 1;\n\n  puts(y)\n}\n"
    );
}

#[test]
fn format_trailing_closure_in_pipe() {
    assert_eq!(
        format("[1, 2] |> each |x| { let y = x\nputs(y) }").unwrap(),
        "[1, 2] |> each |x| {\n  let y = x;\n\n  puts(y)\n}\n"
    );
}

#[test]
fn format_single_statement_lambda_inside_parens() {
    assert_eq!(format("map(|x| x + 1)").unwrap(), "map(|x| x + 1)\n");
}

#[test]
fn format_single_statement_lambda_trailing_when_long() {
    let input =
        "map(|some_very_long_parameter_name| some_very_long_parameter_name + another_very_long_expression_here)";
    let expected = "map |some_very_long_parameter_name| {\n  some_very_long_parameter_name + another_very_long_expression_here\n}\n";
    assert_eq!(format(input).unwrap(), expected);
}

#[test]
fn format_lambda_with_other_args_inline_when_short() {
    assert_eq!(
        format("fold(0, |acc, x| acc + x)").unwrap(),
        "fold(0, |acc, x| acc + x)\n"
    );
}

#[test]
fn format_lambda_with_other_args_trailing_when_long() {
    let input =
        "fold_s([[], []], |[prefixes, prefix], key| [prefixes + [[..prefix, key]], [..prefix, key, extra, more]])";
    let expected = "fold_s([[], []]) |[prefixes, prefix], key| {\n  [prefixes + [[..prefix, key]], [..prefix, key, extra, more]]\n}\n";
    assert_eq!(format(input).unwrap(), expected);
}

#[test]
fn format_if_only_inline() {
    let input = "if x { 1 }";
    let output = format(input).unwrap();
    // Simple if should be inline
    assert_eq!(output, "if x { 1 }\n");
}

#[test]
fn format_if_else_inline() {
    let input = "if x { 1 } else { 2 }";
    let output = format(input).unwrap();
    // Simple if-else should be inline
    assert_eq!(output, "if x { 1 } else { 2 }\n");
}

#[test]
fn format_if_else_inline_in_lambda() {
    assert_eq!(
        format(r#"let f = |c| if c == "(" { 1 } else { -1 }"#).unwrap(),
        "let f = |c| if c == \"(\" { 1 } else { -1 }\n"
    );
}

#[test]
fn format_if_else_multiline_when_body_complex() {
    assert_eq!(
        format("if x { let y = 1\ny } else { 2 }").unwrap(),
        "if x {\n  let y = 1;\n\n  y\n} else {\n  2\n}\n"
    );
}

#[test]
fn format_match_inline_cases() {
    assert_eq!(
        format(r#"match x { 0 { "zero" } _ { "other" } }"#).unwrap(),
        "match x {\n  0 { \"zero\" }\n  _ { \"other\" }\n}\n"
    );
}

#[test]
fn format_match_with_guard_inline() {
    assert_eq!(
        format("match x { n if n > 0 { n } _ { 0 } }").unwrap(),
        "match x {\n  n if n > 0 { n }\n  _ { 0 }\n}\n"
    );
}

#[test]
fn format_match_multiline_when_complex() {
    assert_eq!(
        format("match x { 1 { let y = 2\ny } }").unwrap(),
        "match x {\n  1 {\n    let y = 2;\n\n    y\n  }\n}\n"
    );
}

#[test]
fn format_match_preserves_trailing_comment_on_case() {
    assert_eq!(
        format("match x { 1 { a } // comment\n2 { b } }").unwrap(),
        "match x {\n  1 { a } // comment\n  2 { b }\n}\n"
    );
}

#[test]
fn format_pipe_two_elements_inline() {
    // Two elements (one pipe) stays on one line
    let output = format("[1, 2] |> sum").unwrap();
    assert_eq!(output, "[1, 2] |> sum\n");
}

#[test]
fn format_pipe_three_or_more_elements_multiline() {
    // More than two elements forces multi-line
    let input = "input |> lines |> filter(is_nice?) |> size";
    let output = format(input).unwrap();
    assert_eq!(output, "input\n  |> lines\n  |> filter(is_nice?)\n  |> size\n");
}

#[test]
fn format_pipe_chain_multiline() {
    assert_eq!(
        format("[1, 2, 3] |> map(f) |> filter(g) |> sum").unwrap(),
        "[1, 2, 3]\n  |> map(f)\n  |> filter(g)\n  |> sum\n"
    );
}

#[test]
fn format_composition_two_functions_inline() {
    // Two functions stay on one line (like pipes with one function)
    assert_eq!(format("f >> g").unwrap(), "f >> g\n");
}

#[test]
fn format_composition_three_functions_inline_when_fits() {
    // Composition uses line-width based formatting (stays inline if it fits)
    assert_eq!(format("f >> g >> h").unwrap(), "f >> g >> h\n");
}

#[test]
fn format_composition_wraps_at_line_width() {
    // Composition wraps when exceeding line width (100 chars)
    assert_eq!(
        format("very_long_function_name_one >> very_long_function_name_two >> very_long_function_name_three >> very_long_function_name_four").unwrap(),
        "very_long_function_name_one\n  >> very_long_function_name_two\n  >> very_long_function_name_three\n  >> very_long_function_name_four\n"
    );
}

#[test]
fn format_composition_many_short_functions_inline() {
    // Many short functions stay inline if they fit within line width
    assert_eq!(
        format("a >> b >> c >> d >> e >> f").unwrap(),
        "a >> b >> c >> d >> e >> f\n"
    );
}

#[test]
fn format_let() {
    assert_eq!(format("let x=1").unwrap(), "let x = 1\n");
}

#[test]
fn format_let_mut() {
    assert_eq!(format("let mut x=1").unwrap(), "let mut x = 1\n");
}

#[test]
fn format_let_with_expression() {
    assert_eq!(format("let x=1+2").unwrap(), "let x = 1 + 2\n");
}

#[test]
fn format_assign() {
    assert_eq!(format("x=1").unwrap(), "x = 1\n");
}

#[test]
fn format_destructure_list() {
    assert_eq!(format("let [x, y] = list").unwrap(), "let [x, y] = list\n");
}

#[test]
fn format_preserves_comment() {
    assert_eq!(format("// hello\n1").unwrap(), "// hello\n\n1\n");
}

#[test]
fn format_multiple_comments() {
    assert_eq!(
        format("// line 1\n// line 2\n1").unwrap(),
        "// line 1\n\n// line 2\n\n1\n"
    );
}

#[test]
fn format_comments_between_statements() {
    assert_eq!(
        format("let x = 1\n// comment\nlet y = 2").unwrap(),
        "let x = 1\n\n// comment\n\nlet y = 2\n"
    );
}

#[test]
fn format_section_single_expression_inline() {
    // Single-expression sections should be inline without braces
    let input = "input: { read(\"aoc://2022/1\") }";
    let output = format(input).unwrap();
    assert_eq!(output, "input: read(\"aoc://2022/1\")\n");
}

#[test]
fn format_section_multi_statement_keeps_braces() {
    assert_eq!(
        format("part_one: { let x = 1\nx + 2 }").unwrap(),
        "part_one: {\n  let x = 1;\n\n  x + 2\n}\n"
    );
}

#[test]
fn format_section_with_attribute() {
    assert_eq!(format("@slow\ntest: { 1 }").unwrap(), "@slow\ntest: 1\n");
}

#[test]
fn format_section_with_multiple_attributes() {
    assert_eq!(
        format("@slow\n@memoize\npart_one: { 1 }").unwrap(),
        "@slow\n@memoize\npart_one: {\n  1\n}\n"
    );
}

#[test]
fn format_sections_have_blank_lines_between() {
    assert_eq!(
        format("input: 1\npart_one: 2\npart_two: 3").unwrap(),
        "input: 1\n\npart_one: {\n  2\n}\n\npart_two: {\n  3\n}\n"
    );
}

#[test]
fn format_nested_test_sections() {
    assert_eq!(
        format("test: { input: \"hello\"\npart_one: 5 }").unwrap(),
        "test: {\n  input: \"hello\"\n  part_one: 5\n}\n"
    );
}

#[test]
fn format_return() {
    assert_eq!(format("return 42;").unwrap(), "return 42\n");
}

#[test]
fn format_break() {
    assert_eq!(format("break 42;").unwrap(), "break 42\n");
}

#[test]
fn format_index() {
    assert_eq!(format("arr[0]").unwrap(), "arr[0]\n");
}

#[test]
fn format_index_with_expression() {
    assert_eq!(format("arr[i+1]").unwrap(), "arr[i + 1]\n");
}

#[test]
fn format_spread() {
    assert_eq!(format("[..xs]").unwrap(), "[..xs]\n");
}

#[test]
fn format_rest_identifier() {
    // In a pattern context
    assert_eq!(
        format("let [first, ..rest] = list").unwrap(),
        "let [first, ..rest] = list\n"
    );
}

fn assert_idempotent(source: &str) {
    let first = format(source).unwrap();
    let second = format(&first).unwrap();
    assert_eq!(first, second, "Formatting should be idempotent for: {}", source);
}

#[test]
fn idempotent_simple_expression() {
    assert_idempotent("1+2");
}

#[test]
fn idempotent_list() {
    assert_idempotent("[1,2,3]");
}

#[test]
fn idempotent_function() {
    assert_idempotent("|x,y| x + y");
}

#[test]
fn idempotent_pipe_chain() {
    assert_idempotent("[1, 2, 3] |> sum");
}

#[test]
fn idempotent_match() {
    assert_idempotent("match x { 0 { 1 } _ { 2 } }");
}

#[test]
fn idempotent_if_else() {
    assert_idempotent("if x { 1 } else { 2 }");
}

#[test]
fn idempotent_multiline() {
    assert_idempotent("let x = 1;\nlet y = 2;\nx + y");
}

#[test]
fn idempotent_complex() {
    let source = "let data = [1, 2, 3, 4, 5]; data |> sum";
    assert_idempotent(source);
}

fn assert_round_trip(source: &str) {
    let formatted = format(source).unwrap();

    // Both should parse successfully
    let lexer1 = Lexer::new(source);
    let mut parser1 = Parser::new(lexer1);
    let ast1 = parser1.parse().expect("Original should parse");

    let lexer2 = Lexer::new(&formatted);
    let mut parser2 = Parser::new(lexer2);
    let ast2 = parser2.parse().expect("Formatted should parse");

    // Compare statement counts (ignoring source locations)
    assert_eq!(
        ast1.statements.len(),
        ast2.statements.len(),
        "AST should have same number of statements"
    );
}

#[test]
fn round_trip_expression() {
    assert_round_trip("1 + 2 * 3");
}

#[test]
fn round_trip_function() {
    assert_round_trip("|x| x + 1");
}

#[test]
fn round_trip_match() {
    assert_round_trip("match x { 0 { 1 } _ { 2 } }");
}

#[test]
fn round_trip_if_else() {
    assert_round_trip("if x { 1 } else { 2 }");
}

#[test]
fn round_trip_list() {
    assert_round_trip("[1, 2, 3]");
}

#[test]
fn round_trip_pipe() {
    assert_round_trip("[1, 2, 3] |> sum");
}

#[test]
fn format_empty_program() {
    assert_eq!(format("").unwrap(), "");
}

#[test]
fn format_deeply_nested() {
    assert_eq!(format("[[[[1]]]]").unwrap(), "[[[[1]]]]\n");
}

#[test]
fn format_unicode_string() {
    assert_eq!(format(r#""héllo 世界""#).unwrap(), "\"héllo 世界\"\n");
}

#[test]
fn format_unicode_in_strings() {
    assert_eq!(format(r#""café""#).unwrap(), "\"café\"\n");
}

#[test]
fn format_invalid_syntax_returns_error() {
    assert!(format("let = ").is_err());
}

#[test]
fn format_unclosed_bracket_returns_error() {
    assert!(format("[1, 2, 3").is_err());
}

#[test]
fn format_unclosed_string_returns_error() {
    assert!(format(r#""unclosed"#).is_err());
}

#[test]
fn is_formatted_returns_true_for_formatted() {
    use super::is_formatted;
    assert!(is_formatted("1 + 2\n").unwrap());
}

#[test]
fn is_formatted_returns_false_for_unformatted() {
    use super::is_formatted;
    assert!(!is_formatted("1+2").unwrap());
}

#[test]
fn format_top_level_lets_have_blank_lines() {
    assert_eq!(
        format("let a = 1\nlet b = 2\nlet c = 3").unwrap(),
        "let a = 1\n\nlet b = 2\n\nlet c = 3\n"
    );
}

#[test]
fn format_block_final_expression_has_blank_line() {
    assert_eq!(
        format("|x| { let a = 1\nlet b = 2\na + b }").unwrap(),
        "|x| {\n  let a = 1\n  let b = 2;\n\n  a + b\n}\n"
    );
}

#[test]
fn format_block_single_expression_no_blank_line() {
    assert_eq!(format("|x| { x + 1 }").unwrap(), "|x| x + 1\n");
}

#[test]
fn format_block_two_lets_no_blank_line() {
    assert_eq!(
        format("|x| { let a = 1\nlet b = 2 }").unwrap(),
        "|x| {\n  let a = 1\n  let b = 2\n}\n"
    );
}

#[test]
fn format_multiline_return_has_blank_line() {
    assert_eq!(
        format("|x| { let v = process(x)\nreturn v |> map(f) |> filter(g) |> sum }").unwrap(),
        "|x| {\n  let v = process(x)\n\n  return v\n    |> map(f)\n    |> filter(g)\n    |> sum\n}\n"
    );
}

#[test]
fn format_single_line_return_no_blank_line() {
    assert_eq!(
        format("|x| { let r = compute(x)\nreturn r }").unwrap(),
        "|x| {\n  let r = compute(x)\n  return r\n}\n"
    );
}

#[test]
fn format_semicolon_before_implicit_return_skips_comments() {
    assert_eq!(
        format("|x| { let a = 1\n// comment\na + 1 }").unwrap(),
        "|x| {\n  let a = 1;\n  // comment\n\n  a + 1\n}\n"
    );
}

#[test]
fn format_semicolon_with_multiple_comments_before_return() {
    assert_eq!(
        format("|x| { let a = 1\n// comment 1\n// comment 2\na }").unwrap(),
        "|x| {\n  let a = 1;\n  // comment 1\n  // comment 2\n\n  a\n}\n"
    );
}

#[test]
fn format_preserves_parens_for_and_or_mixed() {
    assert_eq!(format("a && b || (c && d)").unwrap(), "a && b || (c && d)\n");
}

#[test]
fn format_preserves_parens_for_or_and_mixed() {
    assert_eq!(format("a || b && (c || d)").unwrap(), "a || b && (c || d)\n");
}

#[test]
fn format_removes_unnecessary_left_parens_and_or() {
    assert_eq!(format("(a && b) || c").unwrap(), "a && b || c\n");
}

#[test]
fn format_preserves_parens_for_pipe_in_addition() {
    assert_eq!(format("a + (b |> f)").unwrap(), "a + (b |> f)\n");
}

#[test]
fn format_preserves_parens_for_pipe_in_subtraction() {
    assert_eq!(format("a - (b |> f |> g)").unwrap(), "a - (b\n  |> f\n  |> g)\n");
}

#[test]
fn format_preserves_parens_for_subtraction_right_associativity() {
    assert_eq!(format("a - (b - c)").unwrap(), "a - (b - c)\n");
}

#[test]
fn format_preserves_parens_for_division_right_associativity() {
    assert_eq!(format("a / (b / c)").unwrap(), "a / (b / c)\n");
}

#[test]
fn format_preserves_parens_for_modulo_right_associativity() {
    assert_eq!(format("a % (b % c)").unwrap(), "a % (b % c)\n");
}

#[test]
fn format_preserves_parens_for_addition_right_associativity() {
    // Parens must be preserved because `a + (b + c)` has different semantics
    // than `(a + b) + c` when string concatenation is involved
    assert_eq!(format("a + (b + c)").unwrap(), "a + (b + c)\n");
}

#[test]
fn format_preserves_parens_for_multiplication_right_associativity() {
    // Preserve original grouping for consistency with addition
    assert_eq!(format("a * (b * c)").unwrap(), "a * (b * c)\n");
}

#[test]
fn format_lambda_preserves_braces_for_set_body() {
    assert_eq!(format("|x| { {a, b, c} }").unwrap(), "|x| {\n  {a, b, c}\n}\n");
}

#[test]
fn format_lambda_preserves_braces_for_dict_body() {
    assert_eq!(format("|x| { #{a: 1, b: 2} }").unwrap(), "|x| {\n  #{a: 1, b: 2}\n}\n");
}

#[test]
fn format_lambda_preserves_braces_for_pipe_body() {
    assert_eq!(
        format("|x| { [1, 2, 3] |> map(f) |> sum }").unwrap(),
        "|x| {\n  [1, 2, 3]\n    |> map(f)\n    |> sum\n}\n"
    );
}

#[test]
fn format_lambda_preserves_braces_for_composition_body() {
    // Lambda body with composition needs braces to avoid binding issues
    assert_eq!(format("|x| { f >> g >> h }").unwrap(), "|x| {\n  f >> g >> h\n}\n");
}

#[test]
fn format_lambda_unwraps_match_with_list_subject() {
    assert_eq!(
        format("|x| { match [a, b] { [1, _] { true } _ { false } } }").unwrap(),
        "|x| match [a, b] {\n  [1, _] { true }\n  _ { false }\n}\n"
    );
}

#[test]
fn format_lambda_unwraps_simple_expression() {
    assert_eq!(format("|x| { x + 1 }").unwrap(), "|x| x + 1\n");
}

#[test]
fn format_lambda_unwraps_match_with_identifier_subject() {
    assert_eq!(
        format("|x| { match x { 1 { true } _ { false } } }").unwrap(),
        "|x| match x {\n  1 { true }\n  _ { false }\n}\n"
    );
}

#[test]
fn format_list_no_trailing_comma_inline() {
    assert_eq!(format("[1, 2, 3]").unwrap(), "[1, 2, 3]\n");
}

#[test]
fn format_set_no_trailing_comma_inline() {
    assert_eq!(format("{1, 2, 3}").unwrap(), "{1, 2, 3}\n");
}

#[test]
fn format_dict_no_trailing_comma_inline() {
    assert_eq!(format("#{a: 1, b: 2}").unwrap(), "#{a: 1, b: 2}\n");
}

#[test]
fn round_trip_mixed_and_or_operators() {
    let input = "(a && b) || (c && d)";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn round_trip_pipe_in_arithmetic() {
    let input = "rest(queue) + (items |> map(f))";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn round_trip_lambda_with_set() {
    let input = "|x| { {a, b, c} }";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn round_trip_lambda_with_match_list_subject() {
    let input = "|j| { match [j >= len, j < len] { [true, _] { j } _ { j + 1 } } }";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn format_dict_shorthand_preserved() {
    assert_eq!(format("#{foo}").unwrap(), "#{foo}\n");
    assert_eq!(format("#{foo, bar, baz}").unwrap(), "#{foo, bar, baz}\n");
}

#[test]
fn format_dict_explicit_to_shorthand() {
    assert_eq!(format("#{\"foo\": foo}").unwrap(), "#{foo}\n");
    assert_eq!(format("#{\"foo\": foo, \"bar\": bar}").unwrap(), "#{foo, bar}\n");
}

#[test]
fn format_dict_shorthand_mixed() {
    assert_eq!(format("#{foo, \"bar\": baz}").unwrap(), "#{foo, \"bar\": baz}\n");
    assert_eq!(format("#{\"key\": value}").unwrap(), "#{\"key\": value}\n");
}

#[test]
fn round_trip_dict_shorthand() {
    let input = "#{a, b, c, \"key\": value}";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

// Trailing comment tests

#[test]
fn format_preserves_trailing_comment_on_let() {
    assert_eq!(format("let x = 1  // comment").unwrap(), "let x = 1 // comment\n");
}

#[test]
fn format_preserves_trailing_comment_on_expression() {
    assert_eq!(format("foo(bar)  // inline note").unwrap(), "foo(bar) // inline note\n");
}

#[test]
fn format_preserves_trailing_comment_after_semicolon() {
    // Note: Formatter removes unnecessary semicolons but preserves the trailing comment
    assert_eq!(format("let x = 1;  // comment").unwrap(), "let x = 1 // comment\n");
}

#[test]
fn format_trailing_comment_not_attached_from_next_line() {
    assert_eq!(
        format("let x = 1\n// standalone").unwrap(),
        "let x = 1\n\n// standalone\n"
    );
}

#[test]
fn format_trailing_comment_in_section() {
    // Trailing comments work in sections (wrapped in braces due to trailing content)
    assert_eq!(
        format("part_one: let x = 1  // inline").unwrap(),
        "part_one: {\n  let x = 1 // inline\n}\n"
    );
}

// Blank line preservation tests

#[test]
fn format_preserves_blank_line_between_statements_in_block() {
    assert_eq!(
        format("|x| { let a = 1\n\nlet b = 2\na + b }").unwrap(),
        "|x| {\n  let a = 1\n\n  let b = 2;\n\n  a + b\n}\n"
    );
}

#[test]
fn format_single_newline_no_blank_in_block() {
    assert_eq!(
        format("|x| { let a = 1\nlet b = 2\na + b }").unwrap(),
        "|x| {\n  let a = 1\n  let b = 2;\n\n  a + b\n}\n"
    );
}

#[test]
fn format_preserves_trailing_comment_on_return() {
    assert_eq!(format("return x // done").unwrap(), "return x // done\n");
}

#[test]
fn format_preserves_trailing_comment_on_break() {
    assert_eq!(format("break x // early exit").unwrap(), "break x // early exit\n");
}

#[test]
fn round_trip_trailing_comments() {
    let input = "let x = 1 // comment\nlet y = 2 // another";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn round_trip_blank_lines() {
    let input = "|x| {\n  let a = 1\n\n  let b = 2\n\n  a + b\n}";
    let formatted = format(input).unwrap();
    let reformatted = format(&formatted).unwrap();
    assert_eq!(formatted, reformatted);
}

#[test]
fn format_let_dictionary_pattern_shorthand() {
    assert_eq!(format("let #{name}=x").unwrap(), "let #{name} = x\n");
}

#[test]
fn format_let_dictionary_pattern_multiple() {
    assert_eq!(format("let #{name,age}=x").unwrap(), "let #{name, age} = x\n");
}

#[test]
fn format_let_dictionary_pattern_explicit_key() {
    assert_eq!(
        format(r#"let #{"key":value}=x"#).unwrap(),
        "let #{\"key\": value} = x\n"
    );
}

#[test]
fn format_let_dictionary_pattern_rest() {
    assert_eq!(format("let #{name,..rest}=x").unwrap(), "let #{name, ..rest} = x\n");
}

#[test]
fn format_function_dictionary_parameter() {
    assert_eq!(format("|#{x,y}|x+y").unwrap(), "|#{x, y}| x + y\n");
}

#[test]
fn format_function_dictionary_parameter_explicit_key() {
    assert_eq!(
        format(r#"|#{"a":x,"b":y}|x+y"#).unwrap(),
        "|#{\"a\": x, \"b\": y}| x + y\n"
    );
}

#[test]
fn format_match_dictionary_pattern() {
    assert_eq!(
        format("match d { #{name} { name } }").unwrap(),
        "match d {\n  #{name} { name }\n}\n"
    );
}

#[test]
fn idempotent_dictionary_pattern_let() {
    assert_idempotent("let #{name, age} = x");
}

#[test]
fn idempotent_dictionary_pattern_rest() {
    assert_idempotent("let #{name, ..rest} = x");
}

#[test]
fn idempotent_dictionary_parameter() {
    assert_idempotent("|#{x, y}| x + y");
}

#[test]
fn round_trip_dictionary_pattern_let() {
    assert_round_trip("let #{name, age} = dict");
}

#[test]
fn round_trip_dictionary_pattern_rest() {
    assert_round_trip("let #{name, ..rest} = dict");
}

#[test]
fn round_trip_dictionary_parameter() {
    assert_round_trip("|#{x, y}| x + y");
}

#[test]
fn round_trip_match_dictionary() {
    assert_round_trip("match d { #{name} { name } }");
}

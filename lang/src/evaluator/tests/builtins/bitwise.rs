test_eval! {
    suite bit_operations;

    ("bit_and(3, 5)", "1", bit_and),
    ("bit_or(3, 5)", "7", bit_or),
    ("bit_xor(3, 5)", "6", bit_xor),
    ("bit_shift_left(1, 2)", "4", bit_shift_left),
    ("bit_shift_right(16, 1)", "8", bit_shift_right)
}

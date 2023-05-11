# Language

- Everything is an expression.
- Everything is an function (mostly).
- Semicolons are optional.
- Block expressions implicitly return the last statement as their result, unless an explicit `return` is used.

There is a [write-up](https://eddmann.com/posts/designing-santa-lang-a-language-for-solving-advent-of-code-puzzles/) detailing the design decisions that went into creating the langauge.

## Types

### Integer

Represents Integer values, stored as a 64-bit signed number.

```santa
let int = 1;
let int_with_underscores = 1_000_000;
```

### Decimal

Represents Decimal values, stored as a 64-bit floating point number (the _binary64_ type defined in [IEEE 754-2008](https://en.wikipedia.org/wiki/IEEE_754)).

```santa
let dec = 1.5;
let dec_with_underscores = 1_000_000.50;
```

### String

Represents UTF-8 encoded character sequences, with the ability to escape newlines `\n`, tabs `\t` and quotes `\"`.

```santa
let str = "Hello, world!";
let escaped_str = "\"Hello, world!\"\n";
let str_with_unicode = "❤🍕";
```

### Range

#### Exclusive Range

Lazily evaluates the Integer range from a given start value until (but not including) the end value.
If the start value is greater than the end value then a _-1_ step is applied each iteration.

```santa
let asc_exc_range = 1..5;
let desc_exc_range = 2..-2;
```

```santa
let until = 5;
let exc_range_using_expr = (0 + 1)..until;
```

#### Inclusive Range

Lazily evaluates the Integer range from a given start value until (and including) the end value.
If the start value is greater than the end value then a _-1_ step is applied each iteration.

```santa
let asc_inc_range = 1..=5;
let desc_inc_range = 2..=-2;
```

```santa
let to = 5;
let inc_range_using_expr = (0 + 1)..=to;
```

#### Unbounded Range

Lazily evaluates an infinite Integer range from a given start value, using _+1_ step each iteration.

```santa
let inf_range = 1..;
let neg_inf_range = -5..;
```

### Collection

Collections are [persistent data-structures](https://en.wikipedia.org/wiki/Persistent_data_structure), yielding new values upon mutation.

#### List

Represents a sequence of heterogeneous elements in insertion order.

```santa
let homogeneous_list = [1, 2, 3];
let heterogeneous_list = ["4", 5.0];
```

```santa
let list = [1, 2, 3];
list |> push(4); // [1, 2, 3, 4]
list; // [1, 2, 3]
```

#### Set

Represents an unordered collection of unique heterogeneous elements.

```santa
let homogeneous_set = {1, 2, 2, 3}; // {1, 2, 3}
let heterogeneous_set = {"4", 5.0, "4"}; // {"4", 5.0}
```

```santa
let set = {1, 2, 3};
set |> push(4); // {1, 2, 3, 4};
set; // {1, 2, 3}
```

Most types can be stored within a Set, except for Lazy Sequences and Functions.

```santa
let set = {1, || 1}; // Error
```

#### Dictionary

Represents an unordered association between arbitrary keys and values.

```santa
let homogeneous_dict = #{"a": 1, "b": 2};
let heterogeneous_dict = #{[1]: 1.5, 2: true, homogeneous_dict};
```

```santa
let dictionary = #{"a": 1};
dictionary |> assoc("a", 2); // #{"a": 2}
dictionary; // #{"a": 1}
```

Most types can be used as Dictionary keys, except for Lazy Sequences and Functions.

```santa
let attempted_key = || 1;
let dictionary = #{attempted_key: "one"}; // Error
```

### Lazy Sequence

The language supports the concept of Lazy Sequences, which among other benefits, unlock the ability to produce infinite sequences.
Both bounded (inclusive, exclusive) and unbounded (infinite) [Ranges](#range) are examples of an Lazy Sequence.
Functions (such as `filter` and `map`) are applied **only** when required, in the example below when we invoke `take`.

```santa
1.. |> filter(|n| n % 2 == 0) |> take(5);
```

The sequence is an immutable definition of desired computation and can be shared.

```santa
let lazy_seq = zip(1.., 2..) |> map(|[x, y]| x * y);
[
  lazy_seq |> skip(5) |> first,
  lazy_seq |> first
];
```

Their are several other means of generating a Lazy Sequence:

Iterate takes a pure function and applies the previous result (starting with an initial value) upon each iteration.

```santa
iterate(|[a, b]| [b, a + b], [0, 1]) |> find(|[a]| a > 10);
```

Cycle iterates through a List indefinitely, looping back to the start once exhausted.

```santa
cycle([1, 2, 3]) |> skip(1) |> take(3);
```

Repeat iterates over the same value indefinitely.

```santa
repeat("a") |> take(3);
```

## Truthy Semantics

Values can be evaluated to a Boolean within predicate expressions using the _truthy_ semantic rules below:

| Type         | True      |
| ------------ | --------- |
| Integer      | Not 0     |
| Decimal      | Not 0.0   |
| String       | Not empty |
| List         | Not empty |
| Set          | Not empty |
| Dictionary   | Not empty |
| LazySequence | Always    |
| Function     | Always    |

## Variables

Variables are declared using let-binding syntax, with names conforming to `[a-Z][a-Z0-9_?]+`.
Bindings are immutable by-default, and can not be reassigned after declaration.

```santa
let x = 1;
x = 2; // Variable 'x' is not mutable
```

Variables can be made mutable using the `mut` keyword, allowing for the binding to be reassigned after declaration.

```santa
let mut x = 1;
x = 2;
```

### Destructing

List collection values can be destructed into desired let-bindings.
The `_` placeholder symbol is used to denote an ignored positional binding.
The `..` rest symbol is used to collect all the remaining elements into a single _List_ let-binding.

```santa
let [x, y, _, ..z] = [1, 2, 3, 4, 5];
[x, y, z];
```

Similar to previous let-bindings, these are immutable by-default.
Destructed bindings can be made mutable using the `mut` keyword.

```santa
let mut [x, y] = [1, 2];
x = 2;
```

## Operators

Expected arthritic operations on Integer and Decimal values are available, along with intuitive behaviour on other types.

```santa
1 + 1; // 2
1 + 2.5; // 3
1.5 + 3.25; // 4.75
1.5 + 1; // 2.5
"a" + "b"; // "ab"
[1] + [2, 3]; // [1, 2, 3]
{1} + {1, 2}; // {1, 2}
#{1: "one"} + #{2: "two"}; // #{1: "one", 2: "two}
```

```santa
2 - 1; // 1
2 - 1.5; // 1
1.5 - 1.25; // 0.25
1.5 - 1; // 0.50
{1, 2} - {1}; // {2}
```

```santa
2 * 2; // 4
2.2 * 2; // 4.4
"a" * 3; // "aaa"
["a"] * 2; // ["a", "a"]
```

```santa
5 / 2; // 2
5 / 2.25; // 2
5.0 / 2; // 2.5
5.0 / 2.25; // 2.22
```

Logical _OR_ and _AND_ operations are supported for all values.
Non-Boolean values are evaluated based on _truthy_ value semantics.

```santa
true || false;
1 || 0;
```

```santa
true && true;
[1] && {1};
```

Intuitive equality operations are support for all values.

```santa
1 == 1;
1.5 == 1.5;
"a" == "a";
true == true;
[1, 2, 3] == [1, 2, 3];
{1, 2, 3} == {1, 2, 3};
#{"a": 1} == #{"a": 1};
```

```santa
1 != 2;
1.5 != 2.0;
"a" != "b";
true != false;
[1, 2, 3] != [1, 2, 3, 4]
{1, 2, 3} != {1, 2, 3, 4};
#{"a": 1} != #{"b": 2};
```

## Indexing

### List

List indexing is zero-based, with the ability to index from the start (positive index) and end (negative index) of a sequence.
If an element is not found at the given index `nil` is returned.

```santa
let list = [1, 2, 3, 4];

list[0]; // 1
list[-1]; // 4
list[4]; // nil
list[-5]; // nil
```

List slices can be achieved by-way of inclusive/exclusive range indexing.

```santa
let list = [1, 2, 3, 4];

list[1..2]; // [2]
list[1..=2]; // [2, 3]
list[1..=-1]; // [2, 1, 4]
```

### Dictionary

Dictionary values can be found via their associated key.
If a Dictionary key is not present within the collection `nil` is returned.

```santa
let dictionary = #{"a": 1, "b": 2};

dictionary["a"]; // 1
dictionary["c"]; // nil
```

### String

String indexing follows much of the same semantics as List indexing, with elements instead being UTF-8 characters.
The returned element is the single UTF-8 character as represented as a String.

```santa
let str = "hello";

str[0]; // "h"
str[-1]; // "o"
str[5]; // nil
str[-6]; // nil
```

String slices can be achieved by-way of inclusive/exclusive range indexing.

```santa
let str = "hello";

str[1..2]; // "e"
str[1..=2]; // "el"
str[1..=-1]; // "eho"
```

## Control Structures

### If

This expression provides a means of performing conditional logic.
The supplied predicate expressions is evaluated using _truthy_ value semantics.
The expression always returns a value; with `nil` being returned in the case of the alternative branch not being specified and condition not passing.
Following block expression semantics found elsewhere in the language, unless explicitly stated (via the `return` keyword) the last statement is implicitly returned as the result.

Expression without an alternative `else` branch.

```santa
if 5 < 10 { 1 } // 1
if 10 < 5 { 1 } // nil
```

Expression with both a consequence and alternative `else` branch.

```santa
if 10 < 5 { 1 } else { 2 }
```

Let-bindings can be declared within the predicate expressions.
If the binding is _truthy_ then the variables is bound and available with the consequence branch.

```santa
if let x = 10 { x } else { 20 }
```

### Return

If you wish to return early from a block expression this can be achieved using the `return` keyword.

```santa
let ten = |x| {
  if x > 10 {
    return "> 10"
  }
  return "< 10"
}
ten(5);
```

### Break

If you wish to break early from a [builtin](builtins.md) looping construct (i.e `fold`, `reduce`, `each`) this can be achieved with the `break` keyword.
In the example below the reduction will be terminated prematurely and the _break_ value will be returned.

```santa
0.. |> reduce |acc, value| {
  if value == 10 {
    break acc
  } else {
    acc + value
  }
};
```

### Match

The match expression allows you to perform pattern matching on a given subject.
If no match is found `nil` is returned as the expression result.

Primitive type values can be matched based on equality rules.

```santa
let fibonacci = |n| match n {
  0 { 0 }
  1 { 1 }
  n { fibonacci(n - 1) + fibonacci(n - 2) }
};
fibonacci(10);
```

List patterns can be matched upon and destructed into let-bindings.

```santa
let map = |fn, list| match list {
  [] { [] }
  [head] { [fn(head)] }
  [head, ..tail] { [fn(head), ..map(fn, tail)] }
};
map(_ + 1, [1, 2, 3]);
```

Values within Integer ranges can be matched upon.

```santa
let number = |n| match n {
  0..5 { "< 5" },
  5..=6 { "5 or 6" }
  7.. { ">= 7" }
};
number(5);
```

An optional predicate expression _guard_ can be defined based on a matched pattern.

```santa
let filter = |fn, list| match list {
  [] { [] }
  [head] if fn(head) { [head] }
  [head, ..tail] if fn(head) { [head, ..filter(fn, tail)] }
  [_, ..tail] { filter(fn, tail) }
};
filter(_ != 2, [1, 2, 3]);
```

## Functions

Defining functions (closures) within the language is intentionally very easy and syntactically cheap to do.

The most basic means of creating a function is to define one using the _pipe_ syntax.

```santa
let inc = |x| { x + 1 };
inc(1);
```

In the event of a single-line block expression, the brackets can be omitted.

```santa
let inc = |x| x + 1;
inc(1);
```

Additionally you can partially apply an existing function, which in-turn will create a new function with the remaining parameter arity.

```santa
let inc = +(1);
inc(1);
```

> This also highlights use of operators being passed around as first-class functions.

The placeholder `_` symbol can also be used to positional omit parameters in which you wish to leave open for the newly created function.

```santa
let minus = -;
let dec = minus(_, 1);
dec(2);
```

Alternatively binary functions (functions which take two arguments) can be more succinctly written using the following placeholder syntax, borrowed from Scala.

```santa
let inc = 1 + _;
let dec = _ - 1;
inc(1) == dec(3);
```

All functions which are declared are Closures, and have access to their outer scope variables.

```santa
let fibonacci_seq = || {
  let mut [a, b] = [0, 1];
  || {
    let aa = a;
    a = b; b = aa + b;
    a;
  };
}();
fibonacci_seq();
fibonacci_seq();
```

Function arguments can be _spread_ from a List, as well as parameters be collected _rest_ into a List.

```santa
let max = |..xs| xs |> sort(<) |> first;
max(..[1, 2, 3]);
```

### Recursion

Recursive function invocation is supported.

```santa
let factorial = |n| if n == 0 { 1 } else { n * factorial(n - 1) };
factorial(10);
```

Along with _Tail-call optimization_.
To avoid exhausting the call stack, the above `factorial` function can be rewritten in a tail-recursive form.
In this case the runtime will reuse the function call stack frame upon each iteration.

```santa
let factorial = |n| {
  let recur = |acc, n| {
    if n == 0 { acc } else { recur(acc * n, n - 1) }
  };
  recur(1, n);
};
factorial(10);
```

### Composition

Functions can be [composed](https://en.wikipedia.org/wiki/Function_composition) together using the `>>` syntax.

```santa
let inc_dbl = _ + 1 >> |x| x * x;
inc_dbl(15);
```

```santa
let parse = lines >> map(split(",") >> map(int));
parse("1,2\n3,4\n5,6");
```

This is syntactic sugar on top of the following expression.

```santa
let parse = |x| {
  map(|line| map(int, split(",", line)), lines(x));
};
parse("1,2\n3,4\n5,6");
```

### Threading

The language leans heavily on functions, and so as to improve readability invocation can be threaded using the `|>` syntax.

```santa
1..5 |> map(_ + 1) |> reduce(+);
```

This is syntactic sugar on top of the following expression.

```santa
reduce(+, map(_ + 1, 1..5));
```

### Trailing Lambda

If the last parameter of a function is a function, then a lambda expression passed as the corresponding argument can be placed outside the parentheses.
Inspired by [Kotlin](https://kotlinlang.org/docs/lambdas.html#passing-trailing-lambdas), this improves readability and enables rich DSLs to be built on-top of language constructs.

```santa
let mut acc = 1;
[1, 2, 3] |> each |x| {
  acc = acc + x * x;
}
acc;
```

```santa
[1, 2, 3] |> fold(1) |acc, x| {
  acc + x * x;
};
```

This is syntactic sugar on top of the following expressions.

```santa
let mut acc = 1;
each(|x| { acc = acc + x * x; }, [1, 2, 3]);
acc;
```

```santa
fold(1, |acc, x| acc + x * x, [1, 2, 3]);
```

### Infix invocation

Functions which accept two arguments (binary) can be called within the infix position like so:

```santa
[1, 2, 3] `includes?` 3;
```

This is syntactic sugar on top of the following expression.

```santa
includes?([1, 2, 3], 3);
```

### Memoization

Referential transparent function calls can be memoized using the built-in higher-order function.

```santa
let fibonacci = memoize |n| {
  if (n > 1) {
    fibonacci(n - 1) + fibonacci(n - 2)
  } else {
    n
  }
}
fibonacci(50);
```

> This highlights the use of the _trailing lambda_ syntax to produce a rich DSL which looks like a language construct.

### Builtins

There is a suite of [builtin functions](builtins.md) which help solve many different class of problem.

# Builtin Functions

The language trys to follow Python's _batteries included_ motto, where-by all desired functionality is included out of the box.
This is achieved by the inclusion of many different builtin functions, allowing you tackle many general-purpose and Advent of Code specific problems.
The following builtin functions are available in **all** runtimes:

## Collection

### list

```
list(value)
```

Return the List representation of the given value.

=== "List"

    ```santa
    list([1, 2, 3])
    ```

=== "Set"

    ```santa
    list({1, 2, 3})
    ```

=== "Dictionary"

    Ouput is a List of List tuples `[key, value]`.

    ```santa
    list(#{1: 2, 3: 4})
    ```

=== "String"

    ```santa
    list("ab")
    ```

=== "Exclusive Range"

    ```santa
    list(1..5)
    ```

=== "Inclusive Range"

    ```santa
    list(1..=5)
    ```

### set

```
set(value)
```

Return the Set representation of the given value.

=== "List"

    ```santa
    set([1, 2, 3])
    ```

=== "Set"

    ```santa
    set({1, 2, 3})
    ```

=== "String"

    ```santa
    set("ab")
    ```

=== "Exclusive Range"

    ```santa
    set(1..5)
    ```

=== "Inclusive Range"

    ```santa
    set(1..=5)
    ```

### dict

```
dict(value)
```

Return the Dictionary representation of the given value.

=== "List"

    Input is a List of List tuples `[key, value]`.

    ```santa
    dict([[1, 2], [3, 4]])
    ```

=== "Dictionary"

    ```santa
    dict(#{1: 2, 3: 4})
    ```

### get

```
get(index, collection)
```

Get an element within a collection following the [indexing](language.md#indexing) rules.
If an element can not be found at that index then `nil` is returned.

=== "List"

    ```santa
    get(1, [1, 2])
    ```

=== "Set"

    ```santa
    get(1, {1, 2})
    ```

=== "Dictionary"

    ```santa
    get(1, #{1: 2, 3: 4})
    ```

=== "String"

    ```santa
    get(1, "ab")
    ```

=== "Exclusive Range"

    ```santa
    get(1, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    get(1, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    get(1, 0..)
    ```

### size

```
size(collection)
```

Get the size of a collection.

=== "List"

    ```santa
    size([1, 2])
    ```

=== "Set"

    ```santa
    size({1, 2})
    ```

=== "Dictionary"

    ```santa
    size(#{1: 2, 3: 4})
    ```

=== "String"

    ```santa
    size("ab")
    ```

=== "Exclusive Range"

    ```santa
    size(1..5)
    ```

=== "Inclusive Range"

    ```santa
    size(1..=5)
    ```

### push

```
push(value, collection)
```

Add a new value to a collection.

=== "List"

    The value is appended to the end of the List.

    ```santa
    push(3, [1, 2])
    ```

=== "Set"

    ```santa
    push(3, {1, 2})
    ```

### assoc

```
assoc(key, value, collection)
```

Associate the provided key/index with the given value in a collection.

=== "List"

    ```santa
    assoc(0, 3, [1, 2])
    ```

    If the index is not already present `nil` values are inserted up to the given index.

    ```santa
    assoc(1, 1, [])
    ```

=== "Dictionary"

    ```santa
    assoc(1, 1, #{1: 2, 3: 4})
    ```

    ```santa
    assoc(0, 1, #{1: 2, 3: 4})
    ```

### update

```
update(key, updater, collection)
```

Update the given index/key of a collection using the supplied pure `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present `nil` is supplied.

=== "List"

    ```santa
    update(0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted up to the given index.

    ```santa
    update(1, || 1, [])
    ```

=== "Dictionary"

    ```santa
    update(0, || 1, #{})
    ```

    ```santa
    update(1, _ + 1, #{1: 2, 3: 4})
    ```

### update_d

```
update_d(key, default, updater, collection)
```

Update the given index/key of a collection using the supplied pure `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present the _default_ value is supplied.

=== "List"

    ```santa
    update_d(0, 0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted up to the given index.

    ```santa
    update_d(1, 0, _ + 1, [])
    ```

=== "Dictionary"

    ```santa
    update_d(0, 0, _ + 1, #{})
    ```

    ```santa
    update_d(1, 0, _ + 1, #{1: 2, 3: 4})
    ```

### map

```
map(mapper, collection)
```

Return a collection with a pure `mapper` function applied over each element within the given collection.

=== "List"

    ```santa
    map(_ + 1, [1, 2])
    ```

=== "Set"

    ```santa
    map(_ + 1, {1, 2})
    ```

=== "Dictionary"

    ```santa
    map(_ + 1, #{1: 2, 3: 4})
    ```

    The `mapper` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    map(|v, k| "" + k + ": " + v, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```santa
    map(_ * 2, "ab")
    ```

Lazy Sequences return another Lazy Sequence, which when resolved will lazily apply the required mapping.

=== "Exclusive Range"

    ```santa
    map(_ + 1, 1..5) |> list
    ```

=== "Inclusive Range"

    ```santa
    map(_ + 1, 1..=5) |> list
    ```

=== "Unbounded Range"

    ```santa
    map(_ + 1, 0..) |> take(3)
    ```

=== "Lazy Sequence"

    ```santa
    map(_ + 1, repeat(1)) |> take(3)
    ```

### filter

```
filter(predicate, collection)
```

Return a collection based on a pure `predicate` function holding [truthy](language.md#truthy-semantics) for the given element in a collection.

=== "List"

    ```santa
    filter(_ == 1, [1, 2])
    ```

=== "Set"

    ```santa
    filter(_ == 1, {1, 2})
    ```

=== "Dictionary"

    ```santa
    filter(_ == 2, #{1: 2, 3: 4})
    ```

    The `predicate` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    map(|_, k| k == 3, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the predicate.
    The returned collection is a List.

    ```santa
    filter(_ == "a", "ab")
    ```

Lazy Sequences return another Lazy Sequence, which when resolved will lazily apply the required filter.

=== "Exclusive Range"

    ```santa
    filter(_ % 2, 1..5) |> list
    ```

=== "Inclusive Range"

    ```santa
    filter(_ % 2, 1..=5) |> list
    ```

=== "Unbounded Range"

    ```santa
    filter(_ % 2, 0..) |> take(3)
    ```

=== "Lazy Sequence"

    ```santa
    filter(_ != 2, cycle([1, 2, 3])) |> take(3)
    ```

### each

```
each(side_effect, collection)
```

Apply a side-effecting function over each element in the given collection.

=== "List"

    ```santa
    let mut acc = 0;
    each(|v| acc = acc + v, [1, 2]);
    acc;
    ```

=== "Set"

    ```santa
    let mut acc = 0;
    each(|v| acc = acc + v, {1, 2});
    acc;
    ```

=== "Dictionary"

    ```santa
    let mut acc = 0;
    each(|v| acc = acc + v, #{1: 2, 3: 4});
    acc;
    ```

    The `predicate` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    let mut acc = 0;
    each(|_, k| acc = ac + k, #{1: 2, 3: 4});
    acc;
    ```

=== "String"

    Each character is considered an element within the iteration.
    The returned collection is a List.

    ```santa
    let mut acc = 0;
    each(|_| acc = acc + 1, "ab");
    acc;
    ```

The function can `break` which will terminate the collection iteration early.

=== "Exclusive Range"

    ```santa
    let mut acc = 0;
    each(|v| acc = acc + v, 1..5);
    acc;
    ```

=== "Inclusive Range"

    ```santa
    let mut acc = 0;
    each(|v| acc = acc + v, 1..=5);
    acc;
    ```

=== "Unbounded Range"

    ```santa
    let mut acc = 0;
    0.. |> each |v| {
      if v == 10 { break nil } else { acc = acc + v }
    };
    acc;
    ```

=== "Lazy Sequence"

    ```santa
    let mut acc = 0;
    iterate(_ + 1, 1) |> each |v| {
      if v == 10 { break nil } else { acc = acc + v }
    };
    acc;
    ```

### reduce

```
reduce(reducer, collection)
```

Apply a pure `reducer` function over a given collection.
The initial accumulator value supplied upon first iteration is the first element in the collection.
If the collection is empty then an error is thrown.

=== "List"

    ```santa
    reduce(+, [1, 2])
    ```

=== "Set"

    ```santa
    reduce(+, {1, 2})
    ```

=== "Dictionary"

    ```santa
    reduce(+, #{1: 2, 3: 4})
    ```

    The `reducer` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    reduce(|acc, _, k| acc + k, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the reduction.
    The returned collection is a List.

    ```santa
    reduce(|acc, ch| ch + acc, "ab")
    ```

The function can `break` which will terminate the collection iteration early.

=== "Exclusive Range"

    ```santa
    reduce(+, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    reduce(+, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    0.. |> reduce |acc, v| {
      if acc == 10 { break acc } else { acc + v }
    }
    ```

=== "Lazy Sequence"

    ```santa
    iterate(_ + 1, 1) |> reduce |acc, v| {
      if acc == 10 { break acc } else { acc + v }
    }
    ```

### fold

```
fold(initial, folder, collection)
```

Apply a pure `folder` function over a given collection.
The initial fold receives the first element and the _initial value_ supplied.
If the collection is empty then the _initial value_ is returned.

=== "List"

    ```santa
    fold(0, +, [1, 2])
    ```

=== "Set"

    ```santa
    fold(0, +, {1, 2})
    ```

=== "Dictionary"

    ```santa
    fold(0, +, #{1: 2, 3: 4})
    ```

    The `folder` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    fold(0, |acc, _, k| acc + k, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the fold.
    The returned collection is a List.

    ```santa
    fold(0, _ + 1, "ab")
    ```

The function can `break` which will terminate the collection iteration early.

=== "Exclusive Range"

    ```santa
    fold(0, +, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    fold(0, +, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    0.. |> fold |acc, v| {
      if acc == 10 { break acc } else { acc + v }
    }
    ```

=== "Lazy Sequence"

    ```santa
    iterate(_ + 1, 1) |> fold |acc, v| {
      if acc == 10 { break acc } else { acc + v }
    }
    ```

### fold_s

```
fold_s(initial, folder, collection)
```

Apply a pure `folder` function over a given collection, with optional state which is passed along throughout the fold.
The accumulated value is a List comprising of the first element being the resulting _folded_ value, and other elements being _state_ you wish to pass on to the next iteration.
Upon completion, the extra state is discarded and the folded value is returned.
If the collection is empty then the _initial value_ is returned.

```santa
50..100 |> fold_s(
  [0, 0, 0],
  |[acc, x, y], val| [acc + x * y * val, val, val / 2]
)
```

### find

```
find(predicate, collection)
```

Apply a pure `predicate` function over a given collection, returning the first element where the predicate holds [truthy](language.md#truthy-semantics).

=== "List"

    ```santa
    find(_ % 2, [1, 2])
    ```

=== "Set"

    ```santa
    find(_ % 2, {1, 2})
    ```

=== "Dictionary"

    ```santa
    find(_ % 2, #{1: 2, 3: 4})
    ```

    The `predicate` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    find(|_, k| k % 2, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the predicate.
    The returned collection is a List.

    ```santa
    find(_ == "b", "ab")
    ```

=== "Exclusive Range"

    ```santa
    find(_ % 2, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    find(_ % 2, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    find(_ % 2, 0..)
    ```

=== "Lazy Sequence"

    ```santa
    find(_ % 2, iterate(_ + 1, 1))
    ```

### scan

```
scan(initial, folder, collection)
```

Return a collection which includes the result of each iteration of folding a pure `folder` function over each element within the given collection.

=== "List"

    ```santa
    scan(0, +, [1, 2])
    ```

=== "Set"

    ```santa
    scan(0, +, {1, 2})
    ```

=== "Dictionary"

    ```santa
    scan(0, +, #{1: 2, 3: 4})
    ```

    The `folder` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    scan(0, |acc, _, k| acc + k, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the fold.
    The returned collection is a List.

    ```santa
    scan("", +, "ab")
    ```

=== "Exclusive Range"

    ```santa
    scan(0, +, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    scan(0, +, 1..=5)
    ```

### flat_map

```
flat_map(mapper, collection)
```

Apply a pure `mapper` function over a given collection with the resulting _mapped_ List results being flattened into a single List.

```santa
flat_map(_ * 2, [[1, 2], [3, 4]])
```

### filter_map

```
filter_map(mapper, collection)
```

Apply a pure `mapper` function over a given collection and filter out the mapped values based on them being [truthy](language.md#truthy-semantics).
This is a convenience function (inspired by [Rust](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)) for the common place `map(..) |> filter(..)` pattern.

=== "List"

    ```santa
    [1, 2] |> filter_map(|v| if v != 1 { v + 1 })
    ```

=== "Set"

    ```santa
    {1, 2} |> filter_map(|v| if v != 1 { v + 1 })
    ```

=== "Dictionary"

    ```santa
    #{1: 2, 3: 4} |> filter_map(|v| if v != 2 { v + 1 })
    ```

    The `mapper` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    #{1: 2, 3: 4} |> filter_map(|_, k| if k != 1 { k + 1 })
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```santa
    "ab" |> filter_map(|v| if v != "a" { v * 2 })
    ```

=== "Exclusive Range"

    ```santa
    1..5 |> filter_map(|v| if v != 1 { v + 1 })
    ```

=== "Inclusive Range"

    ```santa
    1..=5 |> filter_map(|v| if v != 1 { v + 1 })
    ```

=== "Unbounded Range"

    ```santa
    1..
      |> filter_map(|v| if v != 1 { v + 1 })
      |> take(3)
    ```

=== "Lazy Sequence"

    ```santa
    iterate(_ + 1, 1)
      |> filter_map(|v| if v != 1 { v + 1 })
      |> take(3)
    ```

### find_map

```
find_map(mapper, collection)
```

Apply a pure `mapper` function over a given collection and find the first mapped element where the value returned is [truthy](language.md#truthy-semantics).
This is a convenience function (inspired by [Rust](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)) for the common place `map(..) |> find(..)` pattern.

=== "List"

    ```santa
    [1, 2] |> find_map(|v| if v != 1 { v + 1 })
    ```

=== "Set"

    ```santa
    {1, 2} |> find_map(|v| if v != 1 { v + 1 })
    ```

=== "Dictionary"

    ```santa
    #{1: 2, 3: 4} |> find_map(|v| if v != 2 { v + 1 })
    ```

    The `mapper` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    #{1: 2, 3: 4} |> find_map(|_, k| if k != 1 { k + 1 })
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```santa
    "ab" |> find_map(|v| if v != "a" { v * 2 })
    ```

=== "Exclusive Range"

    ```santa
    1..5 |> find_map(|v| if v != 1 { v + 1 })
    ```

=== "Inclusive Range"

    ```santa
    1..=5 |> find_map(|v| if v != 1 { v + 1 })
    ```

=== "Unbounded Range"

    ```santa
    1.. |> find_map(|v| if v != 1 { v + 1 })
    ```

=== "Lazy Sequence"

    ```santa
    iterate(_ + 1, 1)
      |> find_map(|v| if v != 1 { v + 1 })
    ```

### count

```
count(predicate, collection)
```

Count the total number of elements where the pure `predicate` function holds [truthy](language.md#truthy-semantics).

=== "List"

    ```santa
    count(_ % 2, [1, 2, 3, 4])
    ```

=== "Set"

    ```santa
    count(_ % 2, {1, 2, 3, 4})
    ```

=== "Dictionary"

    ```santa
    count(_ % 2, #{1: 2, 3: 4})
    ```

    The `predicate` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    count(|_, k| k % 2, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the predicate.
    The returned collection is a List.

    ```santa
    count(_ == "a", "ab")
    ```

=== "Exclusive Range"

    ```santa
    count(_ % 2, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    count(_ % 2, 1..=5)
    ```

### zip

```
zip(collection, ..collections)
```

Takes any number of iterables as an argument and aggregates them together producing a List/Lazy Sequence of List tuples.
Each List tuple contains elements of all iterables occurring at the same position, stopping when the shortest iterables is exhausted.

```santa
zip(0.., "abc", [1.5, 2.5, 3.5])
```

```santa
zip(0.., "abcdef", [1.5, 2.5, 3.5])
```

If any of the iterables have a finite size then a List is returned, else a Lazy Sequence is produced.

```santa
zip(0.., 1..) |> take(3)
```

### sum

```
sum(collection)
```

Sum all the Integer elements within a collection.

=== "List"

    ```santa
    sum([1, 2])
    ```

=== "Set"

    ```santa
    sum({1, 2})
    ```

=== "Dictionary"

    ```santa
    sum(#{1: 2, 3: 4})
    ```

=== "Exclusive Range"

    ```santa
    sum(1..5)
    ```

=== "Inclusive Range"

    ```santa
    sum(1..=5)
    ```

### max

```
max(..values)
```

Find the largest (maximum) element within a collection.
The collections can be supplied as a single argument List (containing multiple collections), or as a multi-arity function call.

```santa
max(1, 2) == max([1, 2])
```

=== "List"

    ```santa
    max([1, 2])
    ```

=== "Set"

    ```santa
    max({1, 2})
    ```

=== "Dictionary"

    ```santa
    max(#{1: 2, 3: 4})
    ```

=== "Exclusive Range"

    ```santa
    max(1..5)
    ```

=== "Inclusive Range"

    ```santa
    max(1..=5)
    ```

### min

```
min(..values)
```

Find the smallest (minimum) element within a collection.
The collections can be supplied as a single argument List (containing multiple collections), or as a multi-arity function call.

```santa
min(1, 2) == min([1, 2])
```

=== "List"

    ```santa
    min([1, 2])
    ```

=== "Set"

    ```santa
    min({1, 2})
    ```

=== "Dictionary"

    ```santa
    min(#{1: 2, 3: 4})
    ```

=== "Exclusive Range"

    ```santa
    min(1..5)
    ```

=== "Inclusive Range"

    ```santa
    min(1..=5)
    ```

### skip

```
skip(total, collection)
```

Skip a number of elements within a collection.
If the collection is a Lazy Sequence the skip is applied when the collection is lazily resolved.

=== "List"

    ```santa
    skip(1, [1, 2, 3])
    ```

=== "Set"

    ```santa
    skip(1, {1, 2, 3})
    ```

=== "Exclusive Range"

    ```santa
    skip(2, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    skip(2, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    skip(2, 1..) |> take(3)
    ```

=== "Lazy Sequence"

    ```santa
    skip(2, iterate(_ + 1, 1)) |> take(3)
    ```

### take

```
take(total, collection)
```

Take a number of elements from a collection.
If the collection is a Lazy Sequence then the collection is resolved with any outstanding operations (`map`, `skip` etc.) being applied.

=== "List"

    ```santa
    take(2, [1, 2, 3])
    ```

=== "Set"

    ```santa
    take(2, {1, 2, 3})
    ```

=== "Exclusive Range"

    ```santa
    take(2, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    take(2, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    take(2, 1..)
    ```

=== "Lazy Sequence"

    ```santa
    take(2, iterate(_ + 1, 1))
    ```

### sort

```
sort(comparator, collection)
```

Sort the collection based on a supplied pure `comparator` function.
The comparator function accepts two values (a, b) from the collection and can either return:

An Boolean value, with `false` signifying _a < b_ and `true` signifying _a > b_.

```santa
sort(>, [3, 2, 1])
```

An Integer value, with a negative value signifying _a < b_, zero signifying _a == b_, and a positive value signifying _a > b_.

```santa
sort(-, [3, 2, 1])
```

### reverse

```
reverse(collection)
```

Reverse the order of a given List collection.

=== "List"

    ```santa
    reverse([1, 2, 3])
    ```

=== "String"

    ```santa
    reverse("abc")
    ```

=== "Exclusive Range"

    ```santa
    reverse(1..5)
    ```

=== "Inclusive Range"

    ```santa
    reverse(1..=5)
    ```

### repeat

```
repeat(value)
```

Generate a Lazy Sequence which repeats the provided value indefinitely.

```santa
repeat(1) |> take(3)
```

### cycle

```
cycle(list)
```

Generate a Lazy Sequence which cycles through each element in a List indefinitely, looping back to the start once exhausted.

=== "List"

    ```santa
    cycle([1, 2, 3]) |> take(4)
    ```

=== "String"

    Each character is considered an element within the Lazy Sequence.

    ```santa
    cycle("abc") |> take(4)
    ```

### iterate

```
iterate(generator, initial)
```

Generate a Lazy Sequence which supplies a provided pure `generator` function with the previous result (starting with an initial value) to produce the next value in the sequence.

```santa
iterate(|[a, b]| [b, a + b], [0, 1])
  |> skip(9)
  |> take(1)
```

```santa
iterate(_ * 2, 1) |> take(5)
```

### keys

```
keys(dictionary)
```

Return the keys in a given Dictionary as a List.

```santa
keys(#{1: 2, 3: 4})
```

### values

```
values(dictionary)
```

Return the values in a given Dictionary as a List.

```santa
values(#{1: 2, 3: 4})
```

### first

```
first(collection)
```

Return the first element within the collection (aka _head_).
If the collection is empty then `nil` is returned.

=== "List"

    ```santa
    first([1, 2])
    ```

=== "Set"

    ```santa
    first({1, 2})
    ```

=== "String"

    Each character is considered an element.

    ```santa
    first("ab")
    ```

=== "Exclusive Range"

    ```santa
    first(1..5)
    ```

=== "Inclusive Range"

    ```santa
    first(1..=5)
    ```

=== "Unbounded Range"

    ```santa
    first(1..)
    ```

=== "Lazy Sequence"

    ```santa
    first(iterate(_ + 1, 1))
    ```

### second

```
second(collection)
```

Return the second element within the collection.
If the collection does not contain a second element then `nil` is returned.

=== "List"

    ```santa
    second([1, 2])
    ```

=== "Set"

    ```santa
    second({1, 2})
    ```

=== "String"

    Each character is considered an element.

    ```santa
    second("ab")
    ```

=== "Exclusive Range"

    ```santa
    second(1..5)
    ```

=== "Inclusive Range"

    ```santa
    second(1..=5)
    ```

=== "Unbounded Range"

    ```santa
    second(1..)
    ```

=== "Lazy Sequence"

    ```santa
    second(iterate(_ + 1, 1))
    ```

### rest

```
rest(collection)
```

Return the collection with the first element omitted (aka _tail_).
If the collection does not have more than one element then an empty List is returned.

=== "List"

    ```santa
    rest([1, 2])
    ```

=== "Set"

    ```santa
    rest({1, 2})
    ```

=== "String"

    Each character is considered an element.

    ```santa
    rest("ab")
    ```

=== "Exclusive Range"

    ```santa
    rest(1..5)
    ```

=== "Inclusive Range"

    ```santa
    rest(1..=5)
    ```

=== "Unbounded Range"

    ```santa
    rest(1..)
    ```

=== "Lazy Sequence"

    ```santa
    rest(iterate(_ + 1, 1))
    ```

### union

```
union(..values)
```

Return the elements (as a Set) which are found in _any_ of the provided collections.
The collections can be supplied as a single argument List (containing multiple collections), or as a multi-arity function call.

```santa
union([{1, 2}, [2, 3], 1..4, "abc"])
```

```santa
union({1, 2}, [2, 3], 1..4, "abc")
```

### intersection

```
intersection(..values)
```

Return the elements (as a Set) which are found in _all_ the provided collections.
The collections can be supplied as a single argument List (containing multiple collections), or as a multi-arity function call.

```santa
intersection([{1, 2}, [2, 3], 1..4])
```

```santa
intersection({1, 2}, [2, 3], 1..4)
```

### rotate

```
rotate(steps, collection)
```

Rotate a given List a number of steps.
If the step number is _positive_ the rotation proceed forward, with the last item moving to the start of the List.
If the step number is _negative_ the rotation will go backwards, with the first item moving to the end of the List.

```santa
rotate(5, [1, 2, 3])
```

```
rotate(-5, [1, 2, 3])
```

### chunk

```
chunk(size, collection)
```

Split a List into chunks based on a given size.
If the List size is not divisible by the chunk size then the last chunk will contain fewer than the desired elements.

```santa
chunk(2, [1, 2, 3])
```

```santa
chunk(2, [1, 2, 3, 4])
```

### combinations

```
combinations(size, collection)
```

Generate a Lazy Sequence which produces all the possible combinations of a desired number of elements from within a List.

```santa
combinations(2, [1, 2, 3, 4, 5]) |> list
```

```santa
combinations(3, [1, 2, 3, 4, 5]) |> find(|x| sum(x) == 10)
```

### includes?

```
includes?(collection, value)
```

Predicate to assert if a value is present within a given collection, based on [equality](language.md#operators) rules.

=== "List"

    ```santa
    includes?([1, 2], 1)
    ```

=== "Set"

    ```santa
    includes?({1, 2}, 1)
    ```

=== "String"

    Each character is considered an element.

    ```santa
    includes?("ab", "a")
    ```

=== "Exclusive Range"

    ```santa
    includes?(1..5, 1)
    ```

=== "Inclusive Range"

    ```santa
    includes?(1..=5, 1)
    ```

=== "Unbounded Range"

    ```santa
    includes?(1.., 5)
    ```

=== "Lazy Sequence"

    ```santa
    includes?(iterate(_ + 1, 1), 5)
    ```

### excludes?

```
excludes?(collection, value)
```

Predicate to assert if a value is not present within a given collection, based on [equality](language.md#operators) rules.

=== "List"

    ```santa
    excludes?([1, 2], 3)
    ```

=== "Set"

    ```santa
    excludes?({1, 2}, 3)
    ```

=== "String"

    Each character is considered an element.

    ```santa
    excludes?("ab", "c")
    ```

=== "Exclusive Range"

    ```santa
    excludes?(1..5, 6)
    ```

=== "Inclusive Range"

    ```santa
    excludes?(1..=5, 6)
    ```

### any?

```
any?(predicate, collection)
```

Predicate to assert if any value within the collection holds [truthy](language.md#truthy-semantics) based on the supplied pure `predicate` function.

=== "List"

    ```santa
    any?(_ == 1, [1, 2])
    ```

=== "Set"

    ```santa
    any?(_ == 1, {1, 2})
    ```

=== "String"

    Each character is considered an element.

    ```santa
    any?(_ == "a", "ab")
    ```

=== "Exclusive Range"

    ```santa
    any?(_ == 1, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    any?(_ == 1, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    any?(_ == 1, 1..)
    ```

=== "Lazy Sequence"

    ```santa
    any?(_ == 1, iterate(_ + 1, 1))
    ```

### all?

```
all?(predicate, collection)
```

Predicate to assert if all values within the collection hold [truthy](language.md#truthy-semantics) based on the supplied pure `predicate` function.

=== "List"

    ```santa
    all?(_ > 0, [1, 2])
    ```

=== "Set"

    ```santa
    all?(_ > 0, {1, 2})
    ```

=== "String"

    Each character is considered an element.

    ```santa
    all?(_ != "c", "ab")
    ```

=== "Exclusive Range"

    ```santa
    all?(_ > 0, 1..5)
    ```

=== "Inclusive Range"

    ```santa
    all?(_ > 0, 1..=5)
    ```

## Math

### abs

```
abs(value)
```

Return the absolute value of a number.

=== "Integer"

    ```santa
    abs(-1)
    ```

=== "Decimal"

    ```santa
    abs(-1.5)
    ```

### vec_add

```
vec_add(a, b)
```

Sum two Lists together using Vector addition.
The resulting List will contain results up to the shortest List's size.

```santa
vec_add([1, 2], [3, 4])
```

```santa
vec_add([1, 2, 3], [4, 5, 6])
```

### signum

```
signum(value)
```

Return the sign (`-1, 0, 1`) for the given number.

=== "Integer"

    ```santa
    signum(5)
    ```

=== "Decimal"

    ```santa
    signum(-5.5)
    ```

## Bitwise

### bit_and

```
bit_and(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of both operands are 1.

```santa
bit_and(9, 11) // 1001 & 1011
```

### bit_or

```
bit_or(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of either or both operands are 1.

```santa
bit_or(9, 11) // 1001 | 1011
```

### bit_xor

```
bit_xor(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of either but not both operands are 1.

```santa
bit_or(9, 11) // 1001 ^ 1011
```

### bit_shift_left

```
bit_shift_left(value, shift)
```

Return an Integer whose binary representation is the first operand shifted by the specified number of bits to the left.

```santa
bit_shift_left(10, 3)
```

### bit_shift_right

```
bit_shift_right(value, shift)
```

Return an Integer whose binary representation is the first operand shifted by the specified number of bits to the right.

```santa
bit_shift_left(10, 3)
```

## String

### int

```
int(value)
```

Attempt to parse the provided value into an Integer representation.

=== "Integer"

    ```santa
    int(5)
    ```

=== "Decimal"

    ```santa
    int(-5.5)
    ```

=== "String"

    ```santa
    int("5")
    ```

=== "Boolean"

    ```santa
    int(true)
    ```

Upon failure to parse the value `nil` is returned.

```santa
int("invalid")
```

### ints

```
ints(value)
```

Return all parseable Integer values (as per [`int`](#int)) from a String value as a List.
If no Integers are found and empty List is returned.

```santa
ints("1,2,3")
```

```santa
ints("15a20b35")
```

### lines

```
lines(value)
```

Split a given String into a List of Strings, seperated on new lines `\n`.

```santa
lines("a\nb\nc")
```

### split

```
split(seperator, value)
```

Split a given String into a List of Strings, seperated based on the provided value.

```santa
split("-", "a-b-c")
```

### regex_match

```
regex_match(pattern, value)
```

Match and capture values from a subject String based on a provided Regular Expression.
Captured values are returned as a List of Strings.
If no match/capture can be found an empty List is returned.

```santa
regex_match("name: (\\w+), age: (\\d+)", "name: Bob, age: 30")
```

### regex_match_all

```
regex_match_all(pattern, value)
```

Match and capture _all_ occurrences from a subject String based on a provided Regular Expression.
Captured values are returned as a List of Strings.
If no match/capture can be found an empty List is returned.

```santa
regex_match_all("\\w+: \\w+", "name: Bob, age: 30")
```

## Miscellaneous

### range

```
range(from, to, step)
```

Generate an [Inclusive Range](language.md#inclusive-range) using a custom step value (not the default +1, -1).

```santa
range(1, 10, 2) |> list
```

### id

```
id(value)
```

Return the value passed in as an argument.

```santa
id(5)
```

### memoize

```
memoize(function)
```

Return a function which wraps a given pure function [memoizing](language.md#memoization) invocation calls for performance.
This is a trade-off between space and time complexity.

```santa
let fibonacci = memoize |n| {
  if (n > 1) {
    fibonacci(n - 1) + fibonacci(n - 2)
  } else {
    n
  }
};
fibonacci(30)
```

### evaluate

```
evaluate(source)
```

Evaluates the provided String expression within a sandbox _santa-lang_ interpreter.

```santa
evaluate("1.. |> filter(_ % 2) |> take(3)")
```

### type

```
type(value)
```

Return the type of the given value as a String.

```santa
type(1)
```

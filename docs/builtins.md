# Builtin Functions

The language trys to follow Python's _batteries included_ motto, where-by all desired functionality is included out of the box.
This is achieved by the inclusion of many different builtin functions, allowing you tackle many general-purpose and Advent of Code specific problems.
The following builtin functions are available in **all** runtimes:

## Collection

### list

Returns the List represenation of the given value.

```
list(value)
```

=== "List"

    ```santa
    list([1, 2, 3])
    ```

=== "Set"

    ```santa
    list({1, 2, 3})
    ```

=== "Dictionary"

    Ouput is a List of List tuples (key, value).

    ```santa
    list(#{1: 2, 3: 4})
    ```

=== "String"

    ```santa
    list("ab")
    ```

=== "Inclusive Range"

    ```santa
    list(1..5)
    ```

=== "Exclusive Range"

    ```santa
    list(1..=5)
    ```

### set

Returns the Set represenation of the given value.

```
set(value)
```

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

=== "Inclusive Range"

    ```santa
    set(1..5)
    ```

=== "Exclusive Range"

    ```santa
    set(1..=5)
    ```

### dict

Returns the Dictionary represenation of the given value.

```
dict(value)
```

=== "List"

    Input is a List of List tuples (key, value).

    ```santa
    dict([[1, 2], [3, 4]])
    ```

=== "Dictionary"

    ```santa
    dict(#{1: 2, 3: 4})
    ```

### get

Get an element within a collection, following the rules laid out in [Indexing](language.md#indexing).
If an element can not be found at that index then `nil` is returned.

```
get(index, collection)
```

=== "List"

    ```santa
    get(1, [1, 2])
    ```

=== "Set"

    ```santa
    get(1, {2, 1})
    ```

=== "Dictionary"

    ```santa
    get(1, #{1: 2, 3: 4})
    ```

=== "String"

    ```santa
    get(1, "ab")
    ```

=== "Inclusive Range"

    ```santa
    get(1, 1..5)
    ```

=== "Exclusive Range"

    ```santa
    get(1, 1..=5)
    ```

=== "Unbounded Range"

    ```santa
    get(1, 0..)
    ```

### size

Get the size of a collection.

```
size(collection)
```

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

=== "Inclusive Range"

    ```santa
    size(1..5)
    ```

=== "Exclusive Range"

    ```santa
    size(1..=5)
    ```

### push

Add a new value to a collection.

```
push(value, collection)
```

=== "List"

    The value is added to the end of the List.

    ```santa
    push(3, [1, 2])
    ```

=== "Set"

    ```santa
    push(3, {1, 2})
    ```

### assoc

Associate the given key/index with the given value in a collection.

```
assoc(key, value, collection)
```

=== "List"

    ```santa
    assoc(0, 3, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

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

Update the given index/key of a collection using the supplied `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present `nil` is supplied.

```
update(key, updater, collection)
```

=== "List"

    ```santa
    update(0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

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

Update the given index/key of a collection using the supplied `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present the _default_ value is supplied.

```
update_d(key, default, updater, collection)
```

=== "List"

    ```santa
    update_d(0, 0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

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

Apply a pure function over each element within the given collection.

```
map(mapper, collection)
```

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

Lazy Sequences return another Lazy Sequence which when resolved will apply the required mapping.

=== "Inclusive Range"

    ```santa
    map(_ + 1, 1..5) |> list
    ```

=== "Exclusive Range"

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

Return a collection based on a pure predicate function holding truthy for the given element in a collection.

```
filter(predicate, collection)
```

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

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```santa
    filter(_ == "a", "ab")
    ```

Lazy Sequences return another Lazy Sequence which when resolved will apply the required mapping.

=== "Inclusive Range"

    ```santa
    filter(|v| v % 2 == 0, 1..5) |> list
    ```

=== "Exclusive Range"

    ```santa
    filter(|v| v % 2 == 0, 1..=5) |> list
    ```

=== "Unbounded Range"

    ```santa
    filter(|v| v % 2 == 0, 0..) |> take(3)
    ```

=== "Lazy Sequence"

    ```santa
    filter(_ != 2, cycle([1, 2, 3])) |> take(3)
    ```

### each

Apply a side-effecting function over each element in the given collection.

```
each(side_effect, collection)
```

=== "List"

    ```santa
    let mut a = 0;
    each(|v| a = a + v, [1, 2]);
    a;
    ```

=== "Set"

    ```santa
    let mut a = 0;
    each(|v| a = a + v, {1, 2});
    a;
    ```

=== "Dictionary"

    ```santa
    let mut a = 0;
    each(|v| a = a + v, #{1: 2, 3: 4});
    a;
    ```

    The `predicate` function is suppled both the value and key in the context of a Dictionary.

    ```santa
    let mut a = 0;
    each(|_, k| a = a + k, #{1: 2, 3: 4});
    a;
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```santa
    let mut a = 0;
    each(|_| a = a + 1, "ab");
    a;
    ```

The function can `break` which will break out of the iteration early.

=== "Inclusive Range"

    ```santa
    let mut a = 0;
    each(|v| a = a + v, 1..5);
    a;
    ```

=== "Exclusive Range"

    ```santa
    let mut a = 0;
    each(|v| a = a + v, 1..=5);
    a;
    ```

=== "Unbounded Range"

    ```santa
    let mut a = 0;
    each(
      |v| if v == 10 { break nil } else { a = a + v },
      0..
    );
    a;
    ```

=== "Lazy Sequence"

    ```santa
    let mut a = 0;
    each(
      |v| if v == 10 { break nil } else { a = a + v },
      iterate(_ + 1, 1)
    );
    a;
    ```

### reduce

```
reduce(reducer, collection)
```

Apply a reduction function over a given collection.
The initial accumulator value supplied upon first iteration is the first element in the collection.
If the collection is empty then an error is thrown.

### fold

```
fold(initial, folder, collection)
```

Apply a folding function over a given collection.
If the collection is empty then the initial value is returned.

### fold_s

```
fold_s(initial, folder, collection)
```

Apply a folding function over a given collection, with optional state which is passed throughout the fold.
The accumulated value is a List comprised of the first element being the resulting folded value, and other elements being _state_ you wish to pass on to the next iteration.
Upon completion, the extra state is discarded and the folded value is returned.
If the collection is empty then the initial folded value is returned.

### find

```
find(predicate, collection)
```

Apply a pure predicate function over a given collection, returning the first element where the predicate holds truthy.

### scan

```
scan(initial, mapper, collection)
```

### flat_map

```
flat_map(mapper, collection)
```

Apply a function over a given collection with the resulting mapped List results being flatterned into a single List.

### filter_map

```
filter_map(mapper, collection)
```

Apply a map function over a given collection and filter the mapped values based on the values being truthy.
This is a convience function for the common place `map(..) |> filter(..)` pattern.

### find_map

```
find_map(mapper, collection)
```

Apply a map function over a given collection and find the first mapped element where the predicate holds truthy.
This is a convience function for the common place `map(..) |> find(..)` pattern.

### count

```
count(predicate, collection)
```

Count the total number of elements where the pure predicate function holds truthy.

### zip

```
zip(collection, ..collections)
```

Takes any number of iterables as an argument and aggregates them together producing a List/Lazy Sequence of List tuples.
Each List tuple contains elements of all iterables occuring at the same position, stopping when the shortest iterables is exhausted.
If all the iterables have a finite size then a List is returned, else a Lazy Sequence is produced.

### sum

```
sum(collection)
```

Sum all the Integer elements within a collection.

### max

```
max(..values)
```

Find the largest (maxium) elemement within a collection.

### min

```
min(..values)
```

Find the smallest (minimum) elemement within a collection.

### skip

```
skip(total, collection)
```

Skip a number of elements within the a collection.
If the collection is a Lazy Sequence the skip is applied when the collection is resolved.

### take

```
take(total, collection)
```

Take a number of elements from a collection.
If the collection is a Lazy Sequence then the collection is resolved with any outstanding operations (`map`, `skip` etc.) being applied.

### sort

```
sort(comparator, collection)
```

Sort the collection based on a supplied pure comparator function.
The comparator function take in two values (a, b) and can either return:

- Boolean value, with `false` signifying _a < b_ and `true` signifying _a > b_.
- Integer value, with a negative value signifying _a < b_, zero signifying _a == b_, and a positive value signifying _a > b_.

### reverse

```
reverse(collection)
```

Reverse the order of a given List collection.

### repeat

```
repeat(value)
```

Generate a Lazy Sequence which repeats the provided value indefinitely.

### cycle

```
cycle(list)
```

Generate a Lazy Sequence which cycles through each element in a List indefinitely, looping back to the start once exhausted.

### iterate

```
iterate(generator, initial)
```

Generate a Lazy Sequence which takes a pure function and applies the previous result (starting with an initial value) upon each iteration.

### keys

```
keys(dictionary)
```

Return the _keys_ in a given Dictionary as a List.

### values

```
values(dictionary)
```

Return the _values_ in a given Dictionary as a List.

### first

```
first(collection)
```

Return the first element within the collection (aka head).

### second

```
second(collection)
```

Return the second element within the collection.

### rest

```
rest(collection)
```

Return the collection with the first element omitted (aka tail).

### union

```
union(..values)
```

Return the elements which are found in **any** of the provided collections as a Set.

### intersection

```
intersection(..values)
```

Return the elements which are found in **all** of the provided collections as a Set.

### rotate

```
rotate(steps, collection)
```

Rotate a given List a number of steps, moving the end item to the front of the List.
If the step number is positive the rotation will go forward.
If the step number is negative the rotaion will go backwards, with the first item moving to the end of the List.

### chunk

```
chunk(size, collection)
```

Split a List into chunks based on a given size.
If the List size is not divisible by the chunk size, the last chunk will have fewer than the desired chunk size.

### combinations

```
combinations(size, collection)
```

Generate a Lazy Sequence which produces all the possible combinations of a desired number of elements from within a List.

### includes?

```
includes?(collection, value)
```

Predicate to assert if a value is present within a given collection.

### excludes?

```
excludes?(collection, value)
```

Predicate to assert if a value is not present within a given collection.

### any?

```
any?(predicate, collection)
```

Predicate to assert if any value within the collection hold truthy based on the supplied pure predicate function.

### all?

```
all?(predicate, collection)
```

Predicate to assert if all values within the collection hold truthy based on the supplied pure predicate function.

## Math

### abs

```
abs(value)
```

Return the absolute value of a number.

### vec_add

```
vec_add(a, b)
```

Sum two Lists together using Vector addition.

### signum

```
signum(value)
```

Return the sign (`-1, 0, 1`) for the given value.

## Bitwise

### bit_and

```
bit_and(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of both operands are 1.

### bit_or

```
bit_or(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of either or both operands are 1.

### bit_xor

```
bit_xor(a, b)
```

Return an Integer whose binary representation has a 1 in each bit position for which the corresponding bits of either but not both operands are 1.

### bit_shift_left

```
bit_shift_left(a, b)
```

Return an Integer whose binary representation is the first operand shifted by the specified number of bits to the left.

### bit_shift_right

```
bit_shift_right(a, b)
```

Return an Integer whose binary representation is the first operand shifted by the specified number of bits to the right.

## String

### int

```
int(value)
```

Attempt to parse the provided value into an Integer represenation.
Upon failure `nil` is returned.

### ints

```
ints(value)
```

Return all parseable Integer values (as per [`int`](#int)) from a String value as a List.
If no Integers are found and empty List is returned.

### lines

```
lines(value)
```

Split a given String into a List of Strings, seperated on new lines `\n`.

### split

```
split(seperator, value)
```

Split a given String into a List of Strings, seperated based on the provided value.

### regex_match

```
regex_match(pattern, value)
```

Match and capture values from a subject String based on a provided Regular Expression.
Captured values are returned as a List of Strings.
If no match/capture can be found an empty List is returned.

### regex_match_all

```
regex_match_all(pattern, value)
```

Match and capture all values from a subject String based on a provided Regular Expression.
Captured values are returned as a List of Strings.
If no match/capture can be found an empty List is returned.

## Miscellaneous

### range

```
range(from, to, step)
```

Generate an [Inclusive Range](language.md#inclusive-range) using a custom desired step value (not the default +1, -1).

### id

```
id(value)
```

Return the value passed in as an argument.

### memoize

```
memoize(function)
```

Return a function which wraps a given pure function [memoizing](language.md#memoization) invocation calls for performance.
This is a trade-off between space and time complexity.

### evaluate

```
evaluate(source)
```

### type

```
type(value)
```

## Operators

### +

```
+(a, b)
```

### -

```
-(a, b)
```

### \*

```
*(a, b)
```

### /

```
/(a, b)
```

### %

```
%(a, b)
```

### ==

```
==(a, b)
```

### !=

```
!=(a, b)
```

### <

```
<(a, b)
```

### <=

```
<=(a, b)
```

### >

```
>(a, b)
```

### >=

```
>=(a, b)
```

### or

```
or(a, b)
```

### and

```
and(a, b)
```

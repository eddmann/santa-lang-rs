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

    ```
    list([1, 2, 3])
    ```

=== "Set"

    ```
    list({1, 2, 3})
    ```

=== "Map"

    Ouput is a List of List tuples (key, value).

    ```
    list(#{1: 2, 3: 4})
    ```

=== "String"

    ```
    list("ab")
    ```

=== "Inclusive Range"

    ```
    list(1..5)
    ```

=== "Exclusive Range"

    ```
    list(1..=5)
    ```

### set

Returns the Set represenation of the given value.

```
set(value)
```

=== "List"

    ```
    set([1, 2, 3])
    ```

=== "Set"

    ```
    set({1, 2, 3})
    ```

=== "String"

    ```
    set("ab")
    ```

=== "Inclusive Range"

    ```
    set(1..5)
    ```

=== "Exclusive Range"

    ```
    set(1..=5)
    ```

### hash

Returns the (Hash) Map represenation of the given value.

```
hash(value)
```

=== "List"

    Input is a List of List tuples (key, value).

    ```
    hash([[1, 2], [3, 4]])
    ```

=== "Map"

    ```
    hash(#{1: 2, 3: 4})
    ```

### get

Get an element within a collection, following the rules laid out in [Indexing](language.md#indexing).

```
get(index, collection)
```

=== "List"

    ```
    get(1, [1, 2])
    ```

=== "Set"

    ```
    get(1, {2, 1})
    ```

=== "Map"

    ```
    get(1, #{1: 2, 3: 4})
    ```

=== "String"

    ```
    get(1, "ab")
    ```

=== "Inclusive Range"

    ```
    get(1, 1..5)
    ```

=== "Exclusive Range"

    ```
    get(1, 1..=5)
    ```

=== "Unbounded Range"

    ```
    get(1, 0..)
    ```

### size

Get the size of a collection.

```
size(collection)
```

=== "List"

    ```
    size([1, 2])
    ```

=== "Set"

    ```
    size({1, 2})
    ```

=== "Map"

    ```
    size(#{1: 2, 3: 4})
    ```

=== "String"

    ```
    size("ab")
    ```

=== "Inclusive Range"

    ```
    size(1..5)
    ```

=== "Exclusive Range"

    ```
    size(1..=5)
    ```

### push

Add a new value to a collection.

```
push(value, collection)
```

=== "List"

    The value is added to the end of the List.

    ```
    push(3, [1, 2])
    ```

=== "Set"

    ```
    push(3, {1, 2})
    ```

### assoc

Associate the given key/index with the given value in a collection.

```
assoc(key, value, collection)
```

=== "List"

    ```
    assoc(0, 3, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

    ```
    assoc(1, 1, [])
    ```

=== "Map"

    ```
    assoc(1, 1, #{1: 2, 3: 4})
    ```

    ```
    assoc(0, 1, #{1: 2, 3: 4})
    ```

### update

Update the given index/key of a collection using the supplied `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present `nil` is supplied.

```
update(key, updater, collection)
```

=== "List"

    ```
    update(0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

    ```
    update(1, || 1, [])
    ```

=== "Map"

    ```
    update(0, || 1, #{})
    ```

    ```
    update(1, _ + 1, #{1: 2, 3: 4})
    ```

### update_d

Update the given index/key of a collection using the supplied `updater` function.
The `updater` function is supplied the current value at the given index/key, if not present the _default_ value is supplied.

```
update_d(key, default, updater, collection)
```

=== "List"

    ```
    update_d(0, 0, _ + 1, [1, 2])
    ```

    If the index is not already present `nil` values are inserted upto the given index.

    ```
    update_d(0, 0, _ + 1, [])
    ```

=== "Map"

    ```
    update_d(0, 0, _ + 1, #{})
    ```

    ```
    update_d(1, 0, _ + 1, #{1: 2, 3: 4})
    ```

### map

Apply a pure function over each element within the given collection.

```
map(mapper, collection)
```

=== "List"

    ```
    map(_ + 1, [1, 2])
    ```

=== "Set"

    ```
    map(_ + 1, {1, 2})
    ```

=== "Map"

    ```
    map(_ + 1, #{1: 2, 3: 4})
    ```

    The `mapper` function is suppled both the value and key in the context of a Map.

    ```
    map(|v, k| "" + k + ": " + v, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```
    map(_ * 2, "ab")
    ```

Lazy Sequences return another Lazy Sequence which when resolved will apply the required mapping.

=== "Inclusive Range"

    ```
    map(_ + 1, 1..5) |> list
    ```

=== "Exclusive Range"

    ```
    map(_ + 1, 1..=5) |> list
    ```

=== "Unbounded Range"

    ```
    map(_ + 1, 0..) |> take(3)
    ```

=== "Lazy Sequence"

    ```
    map(_ + 1, repeat(1)) |> take(3)
    ```

### filter

Return a collection based on a pure predicate function holding true for the given element in a collection.

```
filter(predicate, collection)
```

=== "List"

    ```
    filter(_ == 1, [1, 2])
    ```

=== "Set"

    ```
    filter(_ == 1, {1, 2})
    ```

=== "Map"

    ```
    filter(_ == 2, #{1: 2, 3: 4})
    ```

    The `predicate` function is suppled both the value and key in the context of a Map.

    ```
    map(|_, key| key == 3, #{1: 2, 3: 4})
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```
    filter(_ == "a", "ab")
    ```

Lazy Sequences return another Lazy Sequence which when resolved will apply the required mapping.

=== "Inclusive Range"

    ```
    filter(|v| v % 2 == 0, 1..5) |> list
    ```

=== "Exclusive Range"

    ```
    filter(|v| v % 2 == 0, 1..=5) |> list
    ```

=== "Unbounded Range"

    ```
    filter(|v| v % 2 == 0, 0..) |> take(3)
    ```

=== "Lazy Sequence"

    ```
    filter(_ != 2, cycle([1, 2, 3])) |> take(3)
    ```

### each

Apply a side-effecting function over each element in the given collection.

```
each(side_effect, collection)
```

=== "List"

    ```
    let mut a = 0;
    each(|v| a = a + v, [1, 2]);
    a;
    ```

=== "Set"

    ```
    let mut a = 0;
    each(|v| a = a + v, {1, 2});
    a;
    ```

=== "Map"

    ```
    let mut a = 0;
    each(|v| a = a + v, #{1: 2, 3: 4});
    a;
    ```

    The `predicate` function is suppled both the value and key in the context of a Map.

    ```
    let mut a = 0;
    each(|_, k| a = a + k, #{1: 2, 3: 4});
    a;
    ```

=== "String"

    Each character is considered an element within the mapping.
    The returned collection is a List.

    ```
    let mut a = 0;
    each(|_| a = a + 1, "ab");
    a;
    ```

The function can `break` which will break out of the iteration early.

=== "Inclusive Range"

    ```
    let mut a = 0;
    each(|v| a = a + v, 1..5);
    a;
    ```

=== "Exclusive Range"

    ```
    let mut a = 0;
    each(|v| a = a + v, 1..=5);
    a;
    ```

=== "Unbounded Range"

    ```
    let mut a = 0;
    each(
      |v| if v == 10 { break nil } else { a = a + v },
      0..
    );
    a;
    ```

=== "Lazy Sequence"

    ```
    let mut a = 0;
    each(
      |v| if v == 10 { break nil } else { a = a + v },
      cycle([1, 2, 3])
    );
    a;
    ```

### reduce

### fold

### fold_s

### flat_map

### filter_map

### find_map

### scan

### find

### count

### zip

### sum

### max

### min

### skip

### take

### sort

### reverse

### repeat

### cycle

### iterate

### keys

### values

### first

### second

### rest

### union

### intersection

### rotate

### chunk

### combinations

### includes?

### excludes?

### any?

### all?

## Math

### abs,

### vec_add

### signum

## Bitwise

### bit_and

### bit_or

### bit_xor

### bit_shift_left

### bit_shift_right

## String

### int

### ints

### lines

### split

### regex_match

### regex_match_all

## Miscellaneous

### range

### id

### memoize

### evaluate

### type

## Operators

### +

### -

### \*

### /

### %

### ==

### !=

### <

### <=

### >

### >=

### or

### and

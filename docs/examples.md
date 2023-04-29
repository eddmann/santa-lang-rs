# Examples

## Reimplementing `map`, `filter`, `fold` and `reduce`

These fundamental functions come part of the standard library, however, they can be [re-implemented](/examples/map-filter-fold-reduce.santa) within the language itself like so:

```
let map = |fn, list| match list {
  [] { [] }
  [head] { [fn(head)] }
  [head, ..tail] { [fn(head), ..map(fn, tail)] }
};

map(_ + 1, [1, 2, 3]);
```

```
let filter = |fn, list| match list {
  [] { [] }
  [head] if fn(head) { [head] }
  [head, ..tail] if fn(head) { [head, ..filter(fn, tail)] }
  [_, ..tail] { filter(fn, tail) }
};

filter(_ != 2, [1, 2, 3])
```

```
let fold = |initial, fn, list| {
  let recur = |acc, list| match list {
    [] { acc }
    [head] { fn(acc, head) }
    [head, ..tail] { recur(fn(acc, head), tail) }
  };
  recur(initial, list);
};

fold(0, +, [1, 2, 3]);
```

```
let reduce = |fn, list| {
  let recur = |acc, list| match list {
    [] { acc }
    [head] { fn(acc, head) }
    [head, ..tail] { recur(fn(acc, head), tail) }
  };
  recur(list[0], list[1..]);
};

reduce(+, [1, 2, 3]);
```

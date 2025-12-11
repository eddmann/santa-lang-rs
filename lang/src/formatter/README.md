# santa-lang Formatter

An opinionated code formatter for santa-lang that produces consistent, readable output.

## Overview

The formatter transforms santa-lang source code into a canonical format, ensuring consistent style across codebases. It follows a single, deterministic formatting style - there are no configuration options.

## Architecture

The formatter uses a three-stage pipeline based on the [Wadler-Lindig](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf) pretty printing algorithm (also used by [Prettier](https://prettier.io/docs/en/technical-details)):

```
Source Code → Lexer → Parser → AST
                                  ↓
                            Builder (AST → Doc IR)
                                  ↓
                            Printer (Doc IR → String)
                                  ↓
                            Formatted Output
```

### Components

| File         | Purpose                                   |
| ------------ | ----------------------------------------- |
| `mod.rs`     | Public API (`format()`, `is_formatted()`) |
| `doc.rs`     | Document intermediate representation (IR) |
| `builder.rs` | Converts AST to Doc IR (formatting logic) |
| `printer.rs` | Renders Doc IR to formatted string        |
| `tests.rs`   | Comprehensive test suite                  |

### Document IR

The `Doc` enum provides an intermediate representation that separates formatting decisions from output generation:

```rust
enum Doc {
    Nil,                              // Empty document
    Text(String),                     // Raw text output
    Line,                             // Soft line (space or newline)
    HardLine,                         // Always newline
    BlankLine,                        // Blank line separator
    Concat(Vec<Doc>),                 // Concatenation
    Group(Box<Doc>),                  // Try to fit on one line
    Nest(usize, Box<Doc>),            // Indentation level
    IfBreak { broken, flat },         // Conditional on line breaking
}
```

Key helpers:

- `Doc::bracketed(open, docs, close, trailing_comma)` - Smart collection formatting
- `Doc::join(docs, sep)` - Join documents with separator
- `Doc::soft_line()` - Optional line break (nothing in flat mode, newline in break mode)

## Formatting Rules

### General

- **Line width**: 100 characters
- **Indentation**: 2 spaces
- **Trailing newline**: Files end with a single newline
- **Blank lines**: One blank line between top-level statements

### Spacing

| Context               | Rule                   | Example      |
| --------------------- | ---------------------- | ------------ |
| Binary operators      | Spaces around          | `1 + 2`      |
| Commas                | Space after            | `[1, 2, 3]`  |
| Colons (dict/section) | Space after            | `key: value` |
| Function calls        | No space before parens | `func(arg)`  |
| Lambda parameters     | No space inside pipes  | `\|x, y\|`   |

### Collections

Collections (lists, sets, dictionaries) use smart wrapping:

```santa
// Fits on one line
[1, 2, 3]

// Wraps when exceeding 100 characters
[
  very_long_name_one,
  very_long_name_two,
  very_long_name_three
]
```

- **No trailing commas** in any mode
- Empty collections: `[]`, `{}`, `#{}`

### Dictionaries

Dictionary shorthand syntax is used when a key string matches the variable name:

```santa
// Shorthand syntax preferred
#{foo, bar, baz}

// Explicit syntax only when key differs from variable
#{"key": value, name}

// Explicit form is converted to shorthand
#{"foo": foo}  // becomes: #{foo}
```

### Pipe Chains (`|>`)

| Chain length                 | Format                        |
| ---------------------------- | ----------------------------- |
| Single pipe (2 elements)     | Inline: `data \|> sum`        |
| Multiple pipes (3+ elements) | Multiline with 2-space indent |

```santa
// Single pipe stays inline
[1, 2, 3] |> sum

// Multiple pipes break to multiple lines
[1, 2, 3]
  |> map(|x| x * 2)
  |> filter(|x| x > 2)
  |> sum
```

### Function Composition (`>>`)

Composition uses **line-width based** formatting:

| Scenario              | Format                        |
| --------------------- | ----------------------------- |
| Fits within 100 chars | Inline: `f >> g >> h`         |
| Exceeds 100 chars     | Multiline with 2-space indent |

```santa
// Short chains stay inline regardless of function count
parse >> validate >> transform

// Long chains wrap at line width
very_long_function_name
  >> another_long_function
  >> third_long_function
```

Note: This differs from pipe chains (`|>`), which always force multiline with 2+ functions.

### Lambda Functions

Single-expression bodies are unwrapped:

```santa
|x| x + 1
|x, y| x * y
```

Multi-statement bodies use braces:

```santa
|x| {
  let y = x * 2;

  y + 1
}
```

**Exceptions** - Braces are preserved when the body would be ambiguous:

1. **Set/Dictionary bodies** - `|x| {1, 2}` would parse `{1, 2}` as lambda body end
2. **Pipe/Composition bodies** - operators would bind to the lambda definition
3. **Match with collection subject** - `|x| match [a, b] { ... }` would parse `[a, b]` as body

### Trailing Lambda Syntax

Multi-statement lambdas as the last argument use trailing lambda style:

```santa
// Instead of keeping inside parentheses:
items |> map(|x| { let y = x * 2; y + 1 })

// Formatted with trailing lambda:
items
  |> map |x| {
    let y = x * 2;

    y + 1
  }
```

### If-Else Expressions

Simple if-else stays inline:

```santa
if x > 0 { "positive" } else { "non-positive" }
```

Complex bodies use multiline format:

```santa
if condition {
  do_something();

  result
} else {
  do_other();

  other_result
}
```

### Match Expressions

Always multiline with cases on separate lines:

```santa
match value {
  0 { "zero" }
  1 { "one" }
  n if n > 10 { "large" }
  _ { "other" }
}
```

### Sections (AoC Runner)

Single-expression sections are inline:

```santa
input: read("aoc://2024/1")
```

`part_one` and `part_two` always use braces at top level:

```santa
part_one: {
  input |> solve
}
```

Attributes appear on separate lines:

```santa
@slow
test: {
  input: "test data"
  part_one: 42
}
```

### Operator Precedence

The formatter preserves necessary parentheses and removes unnecessary ones:

```santa
// Parentheses preserved for precedence
(a + b) * c
a + (b * c)  // Unnecessary - removed → a + b * c

// Left-associative operators
a - b - c       // No parens needed (left-associative)
a - (b - c)     // Parens preserved (changes meaning)
```

Precedence levels (lowest to highest):

1. `&&`, `||` (AndOr)
2. `==`, `!=` (Equals)
3. `<`, `<=`, `>`, `>=` (LessGreater)
4. `|>`, `>>`, ranges (Composition)
5. `+`, `-` (Sum)
6. `*`, `/`, `%` (Product)

### String Escaping

Special characters are escaped:

| Character       | Escaped            |
| --------------- | ------------------ |
| Backslash       | `\\`               |
| Double quote    | `\"`               |
| Tab             | `\t`               |
| Carriage return | `\r`               |
| Newline         | `\n` (conditional) |

**Newline handling**: Literal newlines are preserved in "multiline" strings (>50 characters or >2 newlines). Otherwise, newlines are escaped as `\n`.

### Block Statements

A semicolon is inserted on the last statement before an implicit return expression, and a blank line is added for readability:

```santa
|x| {
  let a = 1
  do_something();

  a + 1
}
```

### Comments

Comments are preserved at top-level with blank line separation:

```santa
// First section

let a = 1

// Second section

let b = 2
```

## Usage

### API

```rust
use santa_lang::format;

// Format source code
let formatted = format("let x=1+2")?;
assert_eq!(formatted, "let x = 1 + 2\n");

// Check if already formatted
let is_fmt = is_formatted("let x = 1 + 2\n")?;
assert!(is_fmt);
```

### CLI

```bash
# Format to stdout
santa-cli -f file.santa

# Format in place
santa-cli --fmt-write file.santa

# Check if formatted (exit 1 if not)
santa-cli --fmt-check file.santa

# Format inline expression
santa-cli -e "1+2" -f

# Format from stdin
echo "1+2" | santa-cli -f
```

### WASM

```javascript
import { format, isFormatted } from "santa-lang-wasm";

const formatted = format("let x=1+2");
const needsFormat = !isFormatted(source);
```

## Properties

The formatter guarantees:

1. **Idempotency**: Formatting formatted code produces identical output
2. **Semantic preservation**: Formatted code parses to an equivalent AST
3. **Determinism**: Same input always produces same output

## Design Decisions

1. **No configuration**: Single canonical style eliminates debates
2. **100-char line width**: Balances readability with modern screen sizes
3. **2-space indentation**: Compact yet visible nesting
4. **No trailing commas inline**: Keeps single-line form compact
5. **Smart lambda unwrapping**: Maximizes readability while avoiding ambiguity
6. **Wadler-Lindig algorithm**: Proven approach for intelligent line breaking with optimal layout decisions

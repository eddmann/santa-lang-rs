# PHP Extension

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/eddmann/santa-lang-rs/tree/main/runtime/php-ext)

This runtime provides the ability to access the language/runner via a native PHP extension.
It includes the following functionality:

- Execute a given solution's source, providing benchmark timing for each defined part.
- Execute a given solution's source test suite.
- Execute a given script source.
- Execute a arbitrary language expression.

This runtime was built to highlight the power of Rust and explore how to build a PHP extension using [ext-php-rs](https://github.com/davidcole1340/ext-php-rs).
Using Rust macros and C bindings, it cleanly abstracts away the need to work directly with the Zend API.
The resulting values are implicitly converted into expected PHP data types.
Errors that occur within the interpreter are relayed back to the PHP runtime via Exceptions.

## Extension

The PHP extension is available for Linux x86 64bit: [santa-lang-php-ext-0.0.1-x86_64-linux]()

## API

With the extension loaded, the interpreter is made accessible via several top-level PHP functions:

```php
function santa_aoc_run(string $source, string $cwd = null): array;

function santa_aoc_test(string $source, string $cwd = null): array;

function santa_evaluate(string $expression, string $cwd = null): array;
```

## External Functions

The PHP extension runtime provides two specific functions, these are:

### puts

```
puts(..value)
```

Prints the supplied values (using their display format) to _stdout_.

```
puts("Hello", [1, 2.5, true])
```

### read

```
read(path)
```

Reads the contents of the given path into a String.
The path can either be:

- A local directory path, absolute or relative to the current working directory supplied within thr PHP invocation.
- Based on a `http(s)` schema being defined; a web URL location.
- Based on a `aoc` schema being defined; a specific Advent of Code problem input (i.e. `aoc://2015/1`).
  In this case an external `SANTA_CLI_SESSION_TOKEN` environment variable must be defined which includes a valid Advent of Code session token.
  This can be extracted from the cookie set upon successful login to the [platform](https://adventofcode.com/).

=== "Local"

    ```
    read("input.txt")
    ```

=== "URL"

    ```
    read("https://www.example.com/input.txt")
    ```

=== "AoC"

    ```
    read("aoc://2015/1")
    ```

## Example

Below is an example which documents the use of the three different PHP functions:

```php
$solution = file_get_contents(__DIR__ . '/solution.santa');

santa_aoc_run($solution, cwd: __DIR__);

santa_aoc_test($solution);

santa_evaluate("1.. |> filter(_ % 2) |> take(3);");
```

## Future scope

Along with providing PHP extension support, there is future scope to expose the interpreter to [Python](https://github.com/PyO3/pyo3) going forward.

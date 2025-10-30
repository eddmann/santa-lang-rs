# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

santa-lang-rs is a Rust implementation of santa-lang, a functional, C-like programming language specifically designed for solving Advent of Code puzzles. This repository includes a complete language implementation (lexer, parser, tree-walking evaluator) and multiple runtime targets.

Related repositories:
- [eddmann/santa-lang](https://github.com/eddmann/santa-lang) - Language specification/documentation
- [eddmann/santa-lang-ts](https://github.com/eddmann/santa-lang-ts) - TypeScript implementation
- [eddmann/santa-lang-editor](https://github.com/eddmann/santa-lang-editor) - Web-based editor

Documentation: https://eddmann.com/santa-lang/

## Development Commands

### Testing

```bash
# Via Make (Docker-based, ensures consistent environment)
make test              # Run all tests
make test/lang         # Test core language only
make test/cli          # Test CLI only
make test/wasm         # Test WebAssembly (runs on host machine)

# Direct cargo (faster for local iteration)
cargo test --package santa-lang          # Core language tests
cargo test --bin santa-cli              # CLI integration tests
wasm-pack test --node runtime/wasm      # WASM tests
```

### Linting & Formatting

```bash
make fmt               # Format all code
make lint              # Run rustfmt + clippy
make can-release       # Full check: lint + test (run before PRs)
```

### Building & Running

```bash
# CLI
cargo run --bin santa-cli -- solution.santa       # Run a solution
cargo run --bin santa-cli -- -t solution.santa    # Run tests
cargo run --bin santa-cli -- -r                   # Start REPL

# Lambda
make lambda/build      # Build Lambda runtime
make lambda/serve      # Serve locally
make lambda/invoke     # Test endpoint

# WASM
wasm-pack build runtime/wasm --target nodejs

# PHP Extension
make php-ext/build
make php-ext/test

# Jupyter
make jupyter/build
make jupyter/run
```

### Benchmarking

All benchmarks run in Docker for reproducible, isolated results:

```bash
# Build benchmark Docker image
make bench/build

# Run hyperfine benchmarks
make bench/run

# Compare versions (in isolated containers)
make bench/compare V1=main V2=HEAD

# Run Criterion microbenchmarks
make bench/criterion

# Generate visualizations
make bench/visualize RESULTS="benchmarks/results/*.json"

# Interactive Docker shell
make bench/shell

# Clean results
make bench/clean
```

**Automated CI Benchmarks:**
Pull requests automatically trigger performance benchmarks that compare against the base branch. Benchmarks run in Docker containers and post results as PR comments with status indicators (✓ unchanged, 🚀 improved, ⚠️ regressed).

See [benchmarks/README.md](benchmarks/README.md) for complete documentation.

### Docker Development

```bash
make shell             # Interactive shell in Docker build environment
```

## Architecture

### Core Language Pipeline (lang/ crate)

The interpreter follows a classic three-stage design:

1. **Lexer** (`lang/src/lexer/`) - Tokenizes source code
   - Handles santa-lang operators: `|>` (pipe), `>>` (composition), `..` (ranges)
   - Tracks source locations for error reporting

2. **Parser** (`lang/src/parser/`) - Builds AST from tokens
   - Pratt parser with precedence climbing
   - Key precedence levels: Lowest → AndOr → Equals → LessGreater → Composition → Sum → Product → Prefix → Call → Index
   - AST types in `ast.rs`: `Program`, `Statement`, `Expression`

3. **Evaluator** (`lang/src/evaluator/`) - Tree-walking interpreter
   - Environment-based lexical scoping with chaining
   - Frame stack for call traces
   - Tail call optimization via continuations
   - Returns `Result<Rc<Object>, RuntimeErr>`

### Object System

All runtime values are `Rc<Object>` (reference-counted, immutable):

- **Primitive types**: Nil, Integer, Decimal, Boolean, String
- **Collections**: List, Set, Dictionary (using `im-rc` persistent data structures)
- **Functions**: Closures with captured environments
- **LazySequence**: For infinite ranges and efficient iteration

Key design decision: `im-rc` fork with object pooling for performance.

### Built-in Functions

Located in `lang/src/evaluator/builtins/`, organized by category:
- `collection.rs` - `map`, `fold`, `filter`, `reduce`, `zip`, `chunk`, `flatten`, etc.
- `string.rs` - `split`, `lines`, `trim`, `chars`, `ints`, etc.
- `math.rs` - `abs`, `min`, `max`, `sum`, `product`, etc.
- `bitwise.rs` - `band`, `bor`, `bxor`, `bnot`, etc.
- `operator.rs` - `+`, `-`, `*`, `/`, `%` as first-class functions

To add a new built-in:
1. Add function to appropriate module in `builtins/`
2. Register in module's `definitions()` function
3. Add tests in `lang/src/evaluator/tests/builtins/`

### Runner (AoC-specific framework)

`lang/src/runner/` implements the AoC solution structure:

```santa
input: read("aoc://2015/1")

part_one: { /* solution */ }

part_two: { /* solution */ }

test: {
  input: "test data"
  part_one: expected_result
  part_two: expected_result
}
```

The runner:
- Evaluates sections in order
- Provides timing via `Time` trait
- Validates test cases
- Handles `break` keyword for early termination

### Multi-Runtime Architecture

All runtimes (`runtime/*/`) share the core `santa-lang` library but differ in:

1. **External functions** - Platform-specific I/O and capabilities
   - CLI: `read()`, `puts()`, AoC URL support (`read("aoc://2015/1")`)
   - WASM: JavaScript function interop via `wasm-bindgen`
   - Lambda: Event/context variables, handler section
   - PHP: PHP function integration
   - Jupyter: Notebook display functions

2. **Entry points** - Different interfaces to the evaluator
   - CLI: `main.rs` with clap for argument parsing
   - WASM: `lib.rs` with exported functions (`aoc_run`, `aoc_test`, `evaluate`)
   - Lambda: AWS Lambda handler in `main.rs`

3. **Time implementation** - Platform-specific timing
   - CLI: `std::time::Instant`
   - WASM: `web_sys::Performance`
   - Lambda: AWS timer utilities

External functions are defined in each runtime's `external_functions.rs`.

## Code Organization

```
santa-lang-rs/
├── lang/                    # Core language library (shared by all runtimes)
│   └── src/
│       ├── lexer/          # Tokenization
│       ├── parser/         # AST construction + parsing logic
│       ├── evaluator/      # Tree-walking interpreter + builtins
│       └── runner/         # AoC section handling + test framework
│
├── runtime/                # Runtime-specific implementations
│   ├── cli/               # Command-line interface (santa-cli binary)
│   ├── wasm/              # WebAssembly (for browser/Node.js)
│   ├── lambda/            # AWS Lambda runtime
│   ├── php-ext/           # PHP extension
│   └── jupyter/           # Jupyter kernel
│
├── Cargo.toml             # Workspace configuration
├── Makefile               # Docker-based build commands
└── .github/workflows/     # CI/CD: test, build-cli, build-lambda, etc.
```

### Key Dependencies

- `im-rc` - Persistent data structures (custom fork with pooling)
- `ordered-float` - Hashable decimal numbers
- `tikv-jemallocator` - Memory allocator for CLI
- `wasm-bindgen` - WebAssembly JavaScript interop
- `ext-php-rs` - PHP extension framework

## Language Features

Key santa-lang features to understand when working with test cases or examples:

- **Function composition**: `f >> g >> h` (right-to-left application)
- **Threading/piping**: `value |> f |> g` (left-to-right data flow)
- **Pattern matching**: `match value { [first, ..rest] if first > 0 { ... } }`
- **Destructuring**: `let [x, y, z] = [1, 2, 3]`
- **Ranges**: `1..10` (exclusive), `1..=10` (inclusive), `1..` (unbounded)
- **Lazy sequences**: Infinite sequences like `1..` are memory-efficient
- **Tail recursion**: Optimized via `return` keyword creating continuations

## Testing

Tests are organized hierarchically:

1. **Unit tests** - Embedded in source modules
   - `lang/src/lexer/tests.rs` - Tokenization tests
   - `lang/src/parser/tests.rs` - Parsing tests
   - `lang/src/evaluator/tests/` - Extensive evaluator test suite
   - Uses `expect-test` crate for snapshot testing

2. **Integration tests** - Runtime-specific
   - `runtime/cli/src/tests.rs` - CLI end-to-end tests
   - `runtime/wasm/src/tests.rs` - WASM binding tests

3. **AoC solution tests** - Built into language
   - Run with `santa-cli -t solution.santa`
   - Tests defined in `test:` section of santa-lang files

## Recent Updates

The codebase was recently updated to:
- Rust 1.85.0 (from 1.81.0)
- Rust 2024 Edition (from 2021)
- Updated dependencies
- Resolved all compiler warnings and deprecations

When making changes, maintain compatibility with these versions.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:
- **test.yml** - Runs on every push (test-lang, test-cli, test-wasm)
- **benchmark.yml** - Runs on PRs affecting lang/CLI, compares performance vs base branch
- **build.yml** - Triggered on draft-release branch (builds all runtimes)
- **build-cli.yml** - Cross-compiles CLI for Linux/macOS
- **build-lambda.yml** - Builds Lambda runtime
- **build-wasm.yml** - Builds WebAssembly package
- **build-php-ext.yml** - Builds PHP extension
- **build-jupyter.yml** - Builds Jupyter Docker image

All use Ubuntu 24.04 or macOS 14 runners with build caching.

**Performance Benchmarking:**
The benchmark workflow automatically runs on PRs that modify core language or CLI code. It:
- Builds and benchmarks both the PR and base branch
- Compares execution times across all test fixtures
- Posts results as a PR comment with ✓/🚀/⚠️ indicators
- Uploads charts and detailed results as artifacts
- Warns on significant regressions (>5% slower)

See [benchmarks/README.md](benchmarks/README.md) for complete documentation.

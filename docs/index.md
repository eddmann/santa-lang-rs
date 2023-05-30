<p align="center"><img src="assets/logo.png" alt="santa-lang" width="400px" /></p>
<p align="center"><a href="cli/">CLI</a> - <a href="web/">Web</a> - <a href="lambda/">Lambda</a> - <a href="php-ext/">PHP Extension</a></p>

# santa-lang

An functional, C-like programming language for solving Advent of Code puzzles.

_Influenced by:_ Rust, Python, Clojure, F#, Scala

## Why?

Over the past several years I have been slowly working through the previous Advent of Code calendars.
For each calendar I opt to solve the puzzles in a new programming language, to familiarise myself with other ways of understanding and working.
However, there comes a time in each calendar that I grow to dislike some aspect of the language.
So I had an idea... why not give this whole programming language design a go.
That way if I grow to dislike this language, I only have myself to blame!

Welcome **santa-lang**, my [tree-walking interpreted](<https://en.wikipedia.org/wiki/Interpreter_(computing)>) programming language designed to help tackle Advent of Code puzzles.

## Implementations

The language specification and concrete interpreter implementation can be seen as two separate concerns; similar to how the Python language has been implemented within [CPython](https://github.com/python/cpython)/[JPython](https://www.jython.org/), and ECMAScript within [V8](https://v8.dev/)/[SpiderMonkey](https://spidermonkey.dev/).
There are two separate implementations of the language (and runner), both of which follow the (informal) specification laid out in this documentation.
There is feature parity in regard to the core language, data types and builtin functions.
Any solution that is written for one implementation should be executable (without modification) on the other.
This is why along with extensive test coverage, each implementation is tested using the full [2022](https://github.com/eddmann/advent-of-code/tree/master/2022/santa-lang) Advent of Code calendar, to ensure of no regressions.

### TypeScript

This was the initial reference implementation used whilst creating of the language and runner.
It allowed for quick exploration of different language constructs, and exercise them with actual Advent of Code problems throughout the 2022 calendar.
TypeScript was chosen due to speed of development and minimal friction to try out new ideas.
The JavaScript runtime also made it easy to develop the initial CLI, Web and Lambda runtimes; with minimal unique delivery requirements.

### Rust

⭐ _This is the recommended implementation_

Taking all the learnings from the TypeScript counterpart, the Rust implementation was built to be a more stable and performant interpreter.
It was felt that the JavaScript runtime could only reach [so much performance](https://eddmann.com/posts/solving-the-advent-of-code-2022-calendar-using-my-own-programming-language-santa-lang/#so-whats-next), and a lower-level language could unlock more performance gains.
Along with performance considerations, during its development there was the addition of a WASM runtime (which aligns with TypeScript's Web counterpart), a PHP extension and a CLI REPL.
These features are all unique to the Rust implementation.

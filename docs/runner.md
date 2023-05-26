# Advent of Code Runner

The language has been designed with the primary goal of aiding in the development of solutions to Advent of Code (AoC) problems.
Whilst the [core language](language.md) provides the building blocks to express and evaluate a problem, it is the addition of the AoC Runner that truly helps aid in solution development.
Over the many years of AoC problems I have tackled, regardless of language used, I noticed a similar pattern of building up a form of framework for both running and testing each solution.
As such, during this languages development, a lot of effort was paid in providing a REPL which catered for a frictionless problem-solving environment.

With the aid of [Sections](language.md#sections), the Runner is able to provide a clear Domain Specific Language (DSL) for the programmer to express and validate the correctness of a solution.

## Solution

The solution source consists of a `part_one` and/or `part_two` section.
Each section is an expression, which when evaluated (by the desired runtime) returns the answer to the given problem.
The solution source is additionally able to define an `input` section, which will be evaluated and supplied to both `part_*` sections in the form of a `input` variable.
The Runner also benchmarks each parts execution time and supplies this to the runtime to be presented accordingly.

Below is an example solution for solving the [first ever](https://adventofcode.com/2015/day/1) AoC problem:

```
input: "()())"

part_one: {
  input |> fold(0) |floor, direction| {
    if direction == "(" { floor + 1 } else { floor - 1 };
  }
}

part_two: {
  zip(1.., input) |> fold(0) |floor, [index, direction]| {
    let next_floor = if direction == "(" { floor + 1 } else { floor - 1 };
    if next_floor < 0 { break index } else { next_floor };
  }
}
```

**Note:** in this example the input has been statically defined as a String response.
Typically, you will see other means of fetching the problem input, for example, via runtime specific implementations of the [external](language.md#external) `read` function.
If is advised to review your desired runtimes documentation to see what is available.

## Testing

Whilst solving a days problem it is common to be shown a smaller example, in which the expected answer is provided.
Along with defining a solution, you are able to provide test cases in the form of `test` sections.
Each test case includes a desired `input`, alongside a `part_one` and/or `part_two` expected result.
Upon testing your solution, the Runner will assert the given solutions result to the expected one and return success or failure accordingly.
It is upto the runtime to determine how this test results are presented.

```
test: {
  input: "()())"
  part_one: -1
  part_two: 5
}
```

## Example

For completeness, below is the full example solution combining both the solution and test blocks.
It also documents use of a runtime-specific external `read` function.

```
input: read("aoc://2015/1")

part_one: {
  input |> fold(0) |floor, direction| {
    if direction == "(" { floor + 1 } else { floor - 1 };
  }
}

part_two: {
  zip(1.., input) |> fold(0) |floor, [index, direction]| {
    let next_floor = if direction == "(" { floor + 1 } else { floor - 1 };
    if next_floor < 0 { break index } else { next_floor };
  }
}

test: {
  input: "()())"
  part_one: -1
  part_two: 5
}
```

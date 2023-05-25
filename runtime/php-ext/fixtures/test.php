<?php

var_dump(santa_aoc_run(file_get_contents(__DIR__ . '/script.santa')));

$solution = file_get_contents(__DIR__ . '/solution.santa');

var_dump(santa_aoc_run($solution, cwd: __DIR__));

var_dump(santa_aoc_test($solution));

var_dump(santa_evaluate("1.. |> filter(_ % 2) |> take(3);"));

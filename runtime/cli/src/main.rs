#![allow(clippy::collapsible_if)]

mod external_functions;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use getopts::Options;
use rustyline::DefaultEditor;
use santa_lang::{AoCRunner, Environment, Evaluator, Lexer, Location, Object, Parser, RunErr, RunEvaluation, Time};
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optopt("e", "eval", "evaluate inline script", "SCRIPT");
    opts.optflag("t", "test", "run the solution's test suite");
    opts.optflag("s", "slow", "include slow tests (marked with @slow)");
    opts.optflag("r", "repl", "begin an interactive REPL session");
    opts.optflag("f", "fmt", "format source code to stdout");
    opts.optflag("", "fmt-write", "format source code in place");
    opts.optflag("", "fmt-check", "check if source is formatted (exit 1 if not)");
    opts.optflag("h", "help", "list available commands");
    opts.optflag("v", "version", "display version information");
    #[cfg(feature = "profile")]
    opts.optflag("p", "profile", "profile the execution");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_help();
        return Ok(());
    }

    if matches.opt_present("v") {
        println!("santa-lang Comet {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if matches.opt_present("r") {
        return repl();
    }

    // Handle formatting options
    let fmt_stdout = matches.opt_present("f");
    let fmt_write = matches.opt_present("fmt-write");
    let fmt_check = matches.opt_present("fmt-check");

    if fmt_stdout || fmt_write || fmt_check {
        return handle_format(&matches, fmt_stdout, fmt_write, fmt_check);
    }

    // Determine source: -e flag > file argument > stdin
    let (source, source_path): (String, Option<String>) = if let Some(eval_script) = matches.opt_str("e") {
        // Eval mode - use inline script
        (eval_script, None)
    } else if matches.free.len() == 1 {
        // File mode
        let path = &matches.free[0];
        let canonical = fs::canonicalize(path)?;
        let source = fs::read_to_string(&canonical)?;
        (source, Some(canonical.to_string_lossy().into_owned()))
    } else if !atty::is(atty::Stream::Stdin) {
        // Stdin mode - read from stdin when not a TTY
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source)?;
        (source, None)
    } else {
        print_help();
        std::process::exit(1);
    };

    // Only change directory if we have a file path
    if let Some(ref path) = source_path {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::env::set_current_dir(parent)?;
        }
    }

    if matches.opt_present("t") {
        let include_slow = matches.opt_present("s");
        return aoc_test(&source, source_path.as_deref(), include_slow);
    }

    #[cfg(feature = "profile")]
    let profiler = if matches.opt_present("p") {
        Some(
            pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    aoc_run(&source, source_path.as_deref())?;

    #[cfg(feature = "profile")]
    if let Some(guard) = profiler {
        let report = guard.report().build().unwrap();

        let flamegraph = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(flamegraph).unwrap();

        use pprof::protos::Message;
        use std::io::Write;
        let mut protobuf = std::fs::File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();
        let mut content = Vec::new();
        profile.write_to_vec(&mut content).unwrap();
        protobuf.write_all(&content).unwrap();

        println!("\nProfile ⏱️");
        println!("- Flamegraph: ./flamegraph.svg");
        println!("- Protobuf: ./profile.pb");
    }

    Ok(())
}

struct CliTime {}
impl Time for CliTime {
    fn now(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }
}

fn print_help() {
    println!(
        "santa-lang CLI - Comet

USAGE:
    santa-cli <SCRIPT>             Run solution file
    santa-cli -e <CODE>            Evaluate inline script
    santa-cli -t <SCRIPT>          Run test suite
    santa-cli -t -s <SCRIPT>       Run test suite including @slow tests
    santa-cli -r                   Start REPL
    santa-cli -h                   Show this help
    cat file | santa-cli           Read script from stdin

OPTIONS:
    -e, --eval <CODE>              Evaluate inline script
    -t, --test                     Run the solution's test suite
    -s, --slow                     Include @slow tests (use with -t)
    -r, --repl                     Begin an interactive REPL session
    -f, --fmt                      Format source code to stdout
        --fmt-write                Format source code in place
        --fmt-check                Check if source is formatted (exit 1 if not)"
    );
    #[cfg(feature = "profile")]
    println!("    -p, --profile                  Enable CPU profiling");
    println!(
        "    -h, --help                     Show this help message
    -v, --version                  Display version information

ENVIRONMENT:
    SANTA_CLI_SESSION_TOKEN        AOC session token for aoc:// URLs"
    );
}

fn repl() -> Result<()> {
    let environment = Environment::new();

    let mut functions = crate::external_functions::definitions();
    let shared_environment = Rc::clone(&environment);
    functions.push((
        "env".to_owned(),
        vec![],
        Rc::new(move |_, _| {
            println!("Environment:");
            for (name, value) in shared_environment.borrow().variables() {
                println!("  {} = {}", name, value);
            }
            Ok(Rc::new(Object::Nil))
        }),
    ));

    let mut evaluator = Evaluator::new_with_external_functions(&functions);

    println!(
        "   ,--.\n  ()   \\\n   /    \\\n _/______\\_\n(__________)\n(/  @  @  \\)\n(`._,()._,')  Santa REPL\n(  `-'`-'  )\n \\        /\n  \\,,,,,,/\n"
    );

    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let expression = line.as_str();
                rl.add_history_entry(expression)?;

                let lexer = Lexer::new(expression);
                let mut parser = Parser::new(lexer);
                let program = match parser.parse() {
                    Ok(parsed) => parsed,
                    Err(error) => {
                        println!("{}", error.message);
                        continue;
                    }
                };

                match evaluator.evaluate_with_environment(&program, Rc::clone(&environment)) {
                    Ok(evaluated) => println!("{}", evaluated),
                    Err(error) => println!("{}", error.message),
                };
            }
            Err(_) => {
                println!("Goodbye");
                break;
            }
        }
    }

    Ok(())
}

fn aoc_run(source: &str, source_path: Option<&str>) -> Result<()> {
    let mut runner = AoCRunner::new_with_external_functions(CliTime {}, &crate::external_functions::definitions());
    match runner.run(source) {
        Ok(RunEvaluation::Script(result)) => {
            println!("{}", result.value);
            Ok(())
        }
        Ok(RunEvaluation::Solution { part_one, part_two }) => {
            if let Some(part_one) = part_one {
                println!(
                    "Part 1: \x1b[32m{}\x1b[0m \x1b[90m{}ms\x1b[0m",
                    part_one.value, part_one.duration
                )
            }

            if let Some(part_two) = part_two {
                println!(
                    "Part 2: \x1b[32m{}\x1b[0m \x1b[90m{}ms\x1b[0m",
                    part_two.value, part_two.duration
                )
            }

            Ok(())
        }
        Err(error) => {
            print_error(source_path.unwrap_or("<stdin>"), source, error);
            std::process::exit(2);
        }
    }
}

fn aoc_test(source: &str, source_path: Option<&str>, include_slow: bool) -> Result<()> {
    let mut runner = AoCRunner::new_with_external_functions(CliTime {}, &crate::external_functions::definitions());
    match runner.test(source, include_slow) {
        Ok(test_cases) => {
            let mut exit_code = 0;

            for (number, test_case) in test_cases.iter().enumerate() {
                if number > 0 {
                    println!()
                }
                if test_case.slow {
                    println!("\x1b[4mTestcase #{}\x1b[0m \x1b[33m(slow)\x1b[0m", number + 1);
                } else {
                    println!("\x1b[4mTestcase #{}\x1b[0m", number + 1);
                }

                if test_case.part_one.is_none() && test_case.part_two.is_none() {
                    println!("No expectations");
                    continue;
                }

                if let Some(part_one) = &test_case.part_one {
                    if part_one.passed {
                        println!("Part 1: {} \x1b[32m✔\x1b[0m", part_one.actual);
                    } else {
                        println!(
                            "Part 1: {} \x1b[31m✘ (Expected: {})\x1b[0m",
                            part_one.actual, part_one.expected
                        );
                        exit_code = 3;
                    }
                }

                if let Some(part_two) = &test_case.part_two {
                    if part_two.passed {
                        println!("Part 2: {} \x1b[32m✔\x1b[0m", part_two.actual);
                    } else {
                        println!(
                            "Part 2: {} \x1b[31m✘ (Expected: {})\x1b[0m",
                            part_two.actual, part_two.expected
                        );
                        exit_code = 3;
                    }
                }
            }

            if exit_code != 0 {
                std::process::exit(exit_code);
            }

            Ok(())
        }
        Err(error) => {
            print_error(source_path.unwrap_or("<stdin>"), source, error);
            std::process::exit(2);
        }
    }
}

fn handle_format(matches: &getopts::Matches, to_stdout: bool, write_file: bool, check_only: bool) -> Result<()> {
    // Determine source: -e flag > file argument > stdin
    let (source, source_path): (String, Option<String>) = if let Some(eval_script) = matches.opt_str("e") {
        (eval_script, None)
    } else if matches.free.len() == 1 {
        let path = &matches.free[0];
        let canonical = fs::canonicalize(path)?;
        let source = fs::read_to_string(&canonical)?;
        (source, Some(canonical.to_string_lossy().into_owned()))
    } else if !atty::is(atty::Stream::Stdin) {
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source)?;
        (source, None)
    } else {
        eprintln!("Error: No source provided for formatting");
        std::process::exit(1);
    };

    match santa_lang::format(&source) {
        Ok(formatted) => {
            if check_only {
                if formatted == source {
                    // Already formatted
                    std::process::exit(0);
                } else {
                    // Needs formatting
                    if let Some(path) = &source_path {
                        eprintln!("{} needs formatting", path);
                    } else {
                        eprintln!("Input needs formatting");
                    }
                    std::process::exit(1);
                }
            } else if write_file {
                if let Some(path) = &source_path {
                    fs::write(path, &formatted)?;
                    println!("Formatted {}", path);
                } else {
                    eprintln!("Error: --fmt-write requires a file path");
                    std::process::exit(1);
                }
            } else if to_stdout {
                print!("{}", formatted);
            }
            Ok(())
        }
        Err(error) => {
            eprintln!("Parse error: {}", error.message);
            std::process::exit(2);
        }
    }
}

fn print_error(source_path: &str, source: &str, error: RunErr) {
    let (line, column) = calculate_line_column(source, error.source);

    println!("\x1b[31m{}\x1b[0m\n", error.message);

    for (position, source_line) in source.split('\n').enumerate() {
        if line > 1 && (position < line - 2 || position > line + 2) {
            continue;
        }

        if position == line {
            println!("  \x1b[37m{:0>2}: {}\x1b[0m", position + 1, source_line);
            println!(
                "  \x1b[31m{}\x1b[0m",
                " ".repeat(format!("{:0>2}: ", position + 1).len() + column) + "^~~"
            );
        } else {
            println!("  \x1b[2m{:0>2}: {}\x1b[0m", position + 1, source_line);
        }
    }

    println!("\n{}:\x1b[32m{}:{}\x1b[0m", source_path, line + 1, column + 1);

    if !error.trace.is_empty() {
        for location in error.trace {
            let (line, column) = calculate_line_column(source, location);
            println!(
                "  \x1b[2m{}:\x1b[0m\x1b[32m{}:{}\x1b[0m",
                &source[location.start..location.end]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
                line + 1,
                column + 1
            );
        }
    }
}

fn calculate_line_column(source: &str, location: Location) -> (usize, usize) {
    let mut line = 0;
    let mut column = 0;

    for (position, character) in source.chars().enumerate() {
        if position == location.start {
            return (line, column);
        }

        column += 1;
        if character == '\n' {
            line += 1;
            column = 0;
        }
    }

    unreachable!()
}

mod external_functions;

use getopts::Options;
use rustyline::DefaultEditor;
use santa_lang::{Environment, Location, Object, RunErr, RunEvaluation, Runner, Time};
use std::fs;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("t", "test", "run the solution's test suite");
    opts.optflag("r", "repl", "begin an interactive REPL session");
    opts.optflag("h", "help", "list available commands");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    if matches.opt_present("r") {
        return repl();
    }

    let source_path = if matches.free.len() == 1 {
        &matches.free[0]
    } else {
        print_usage(&program, opts);
        std::process::exit(1);
    };

    let path = fs::canonicalize(source_path)?;
    let root = path.parent().unwrap();
    std::env::set_current_dir(root)?;

    if matches.opt_present("t") {
        return test(source_path);
    }

    run(source_path)
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

fn print_usage(program: &str, opts: Options) {
    let summary = format!("Usage: {program} [options] <SCRIPT>");
    print!("{}", opts.usage(&summary));
}

fn repl() -> Result<()> {
    let enviornment = Environment::new();

    let mut functions = crate::external_functions::definitions();
    let shared_enviornment = Rc::clone(&enviornment);
    functions.push((
        "env".to_owned(),
        vec![],
        Rc::new(move |_, _| Ok(Rc::new(Object::String(format!("{:?}", shared_enviornment.borrow()))))),
    ));

    let mut runner = Runner::new_with_external_functions(CliTime {}, &functions);

    println!("   ,--.\n  ()   \\\n   /    \\\n _/______\\_\n(__________)\n(/  @  @  \\)\n(`._,()._,')  Santa REPL\n(  `-'`-'  )\n \\        /\n  \\,,,,,,/\n");

    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let expression = line.as_str();
                rl.add_history_entry(expression)?;
                match runner.evaluate(expression, Rc::clone(&enviornment)) {
                    Ok(result) => println!("{}", result),
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

fn run(source_path: &str) -> Result<()> {
    let source = std::fs::read_to_string(source_path)?;

    let mut runner = Runner::new_with_external_functions(CliTime {}, &crate::external_functions::definitions());
    match runner.run(&source) {
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
            print_error(source_path, &source, error);
            std::process::exit(2);
        }
    }
}

fn test(source_path: &str) -> Result<()> {
    let source = std::fs::read_to_string(source_path)?;

    let mut runner = Runner::new_with_external_functions(CliTime {}, &crate::external_functions::definitions());
    match runner.test(&source) {
        Ok(test_cases) => {
            let mut exit_code = 0;

            for (number, test_case) in test_cases.iter().enumerate() {
                println!("\x1b[4mTestcase #{}\x1b[0m", number + 1);

                if test_case.part_one.is_none() && test_case.part_two.is_none() {
                    println!("No expectations");
                    continue;
                }

                if let Some(part_one) = &test_case.part_one {
                    if part_one.passed {
                        println!("Part 1: {} \x1b[32m✔️\x1b[0m", part_one.actual);
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
                        println!("Part 2: {} \x1b[32m✔️\x1b[0m", part_two.actual);
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
            print_error(source_path, &source, error);
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

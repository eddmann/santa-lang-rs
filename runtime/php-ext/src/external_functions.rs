use ext_php_rs::{php_print, php_println};
use santa_lang::{Arguments, Evaluation, ExpressionKind, ExternalFnDef, Location, Object, RuntimeErr};
use std::env;
use std::fs;
use std::rc::Rc;
use url::Url;

pub fn definitions() -> Vec<ExternalFnDef> {
    vec![
        (
            "puts".to_owned(),
            vec![ExpressionKind::RestIdentifier("values".to_owned())],
            Rc::new(puts),
        ),
        (
            "read".to_owned(),
            vec![ExpressionKind::Identifier("path".to_owned())],
            Rc::new(read),
        ),
    ]
}

fn puts(arguments: Arguments, _source: Location) -> Evaluation {
    match &**arguments.get("values").expect("Parameter guaranteed by function signature") {
        Object::List(values) => {
            for value in values {
                php_print!("{}", value)
            }
            php_println!();
            Ok(Rc::new(Object::Nil))
        }
        _ => unreachable!(),
    }
}

fn read(arguments: Arguments, source: Location) -> Evaluation {
    match &**arguments.get("path").expect("Parameter guaranteed by function signature") {
        Object::String(path) => match Url::parse(path) {
            Ok(uri) if uri.scheme() == "aoc" => {
                let host = uri.host_str()
                    .ok_or_else(|| RuntimeErr {
                        message: format!("Invalid AoC URI: missing host in {}", path),
                        source,
                        trace: vec![],
                    })?;

                let cache = format!(
                    "aoc{}_day{:0>2}.input",
                    host,
                    uri.path().replace('/', "")
                );

                if let Ok(content) = fs::read_to_string(&cache) {
                    return Ok(Rc::new(Object::String(content)));
                }

                let token = match env::var_os("SANTA_CLI_SESSION_TOKEN") {
                    Some(token) => token.into_string().map_err(|_| RuntimeErr {
                        message: "SANTA_CLI_SESSION_TOKEN contains invalid UTF-8".to_owned(),
                        source,
                        trace: vec![],
                    })?,
                    None => {
                        return Err(RuntimeErr {
                            message: "Missing SANTA_CLI_SESSION_TOKEN environment variable".to_owned(),
                            source,
                            trace: vec![],
                        })
                    }
                };

                let request = ureq::get(&format!(
                    "https://adventofcode.com/{}/day{}/input",
                    host,
                    uri.path()
                ))
                .set("Cookie", &format!("session={}", token));
                if let Ok(response) = request.call() {
                    if let Ok(input) = response.into_string() {
                        fs::write(cache, input.trim_end().as_bytes()).expect("");
                        return Ok(Rc::new(Object::String(input.trim_end().to_string())));
                    }
                }

                Err(RuntimeErr {
                    message: format!("Failed to read AoC input: {}", path),
                    source,
                    trace: vec![],
                })
            }
            Ok(_) => {
                if let Ok(response) = ureq::get(path).call() {
                    if let Ok(body) = response.into_string() {
                        return Ok(Rc::new(Object::String(body)));
                    }
                }

                Err(RuntimeErr {
                    message: format!("Failed to read URL: {}", path),
                    source,
                    trace: vec![],
                })
            }
            Err(_) => {
                if let Ok(content) = fs::read_to_string(path) {
                    return Ok(Rc::new(Object::String(content)));
                }

                Err(RuntimeErr {
                    message: format!("Failed to read file: {}", path),
                    source,
                    trace: vec![],
                })
            }
        },
        object => Err(RuntimeErr {
            message: format!(
                "Invalid arguments: read(path: {})\nExpected arguments:\nread(path: String)",
                object.name(),
            ),
            source,
            trace: vec![],
        }),
    }
}

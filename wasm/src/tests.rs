use super::*;
use wasm_bindgen_test::*;

const SCRIPT: &str = "1..5 |> map(_ + 1) |> reduce(+);";

const SOLUTION: &str = r#"
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

test: {
  input: "()())"
  part_one: -1
  part_two: 5
}
"#;

#[wasm_bindgen_test]
fn script() {
    let result = aoc_run("1..5 |> map(_ + 1) |> reduce(+);", js_sys::Object::new()).unwrap();

    assert_eq!(
        "14",
        Reflect::get(&result, &"value".into()).unwrap().as_string().unwrap()
    );
}

#[wasm_bindgen_test]
fn solution() {
    let result = aoc_run(SOLUTION, js_sys::Object::new()).unwrap();

    let part_one = Reflect::get(&result, &"part_one".into()).unwrap();
    assert_eq!(
        "-1",
        Reflect::get(&part_one, &"value".into()).unwrap().as_string().unwrap()
    );

    let part_two = Reflect::get(&result, &"part_two".into()).unwrap();
    assert_eq!(
        "5",
        Reflect::get(&part_two, &"value".into()).unwrap().as_string().unwrap()
    );
}

#[wasm_bindgen_test]
fn test_solution() {
    let result = js_sys::Array::from(&aoc_test(SOLUTION, js_sys::Object::new()).unwrap())
        .iter()
        .collect::<Vec<_>>();

    assert_eq!(1, result.len());

    let part_one = Reflect::get(&result[0], &"part_one".into()).unwrap();
    assert_eq!(
        "-1",
        Reflect::get(&part_one, &"expected".into())
            .unwrap()
            .as_string()
            .unwrap()
    );
    assert_eq!(
        "-1",
        Reflect::get(&part_one, &"actual".into()).unwrap().as_string().unwrap()
    );
    assert!(Reflect::get(&part_one, &"passed".into()).unwrap().as_bool().unwrap());

    let part_two = Reflect::get(&result[0], &"part_two".into()).unwrap();
    assert_eq!(
        "5",
        Reflect::get(&part_two, &"expected".into())
            .unwrap()
            .as_string()
            .unwrap()
    );
    assert_eq!(
        "5",
        Reflect::get(&part_two, &"actual".into()).unwrap().as_string().unwrap()
    );
    assert!(Reflect::get(&part_two, &"passed".into()).unwrap().as_bool().unwrap());
}

#[wasm_bindgen_test]
fn evaluation() {
    let result = evaluate(SCRIPT, None).unwrap();

    assert_eq!("14", result.as_string().unwrap())
}

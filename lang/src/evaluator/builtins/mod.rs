#[macro_use]
mod macros;
mod collection;
mod operators;

builtins! {
    operators::plus,
    operators::minus,
    collection::push,
    collection::map,
    collection::filter,
    collection::fold,
    collection::reduce,
    collection::flat_map,
    collection::zip,
    collection::skip,
    collection::take,
    collection::list,
    collection::repeat,
    collection::cycle,
    collection::iterate
}

builtin_aliases! {
    "+" => operators::plus,
    "-" => operators::minus
}

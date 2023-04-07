#[macro_use]
mod macros;
mod collection;
pub mod operators;

builtins! {
    collection::push,
    collection::map,
    collection::filter,
    collection::fold,
    collection::reduce,
    collection::flat_map,
    collection::each,
    collection::zip,
    collection::skip,
    collection::take,
    collection::list,
    collection::set,
    collection::hash,
    collection::repeat,
    collection::cycle,
    collection::iterate,
    operators::or,
    operators::and,
    operators::memoize
}

builtin_aliases! {
    "+" => operators::plus,
    "-" => operators::minus,
    "*" => operators::asterisk,
    "/" => operators::slash,
    "%" => operators::modulo,
    "==" => operators::equal,
    "!=" => operators::not_equal,
    "<" => operators::less_than,
    "<=" => operators::less_than_equal,
    ">" => operators::greater_than,
    ">=" => operators::greater_than_equal
}

#[macro_use]
mod macros;
mod collection;
pub mod operators;

builtins! {
    collection::push,
    collection::size,
    collection::map,
    collection::filter,
    collection::fold,
    collection::each,
    collection::reduce,
    collection::flat_map,
    collection::find,
    collection::count,
    collection::sum,
    collection::max,
    collection::min,
    collection::zip,
    collection::skip,
    collection::take,
    collection::list,
    collection::set,
    collection::hash,
    collection::repeat,
    collection::cycle,
    collection::iterate,
    collection::keys,
    collection::values,
    collection::first,
    collection::rest,
    collection::get,
    operators::or,
    operators::and,
    operators::memoize,
    operators::evaluate
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
    ">=" => operators::greater_than_equal,
    "includes?" => collection::includes,
    "excludes?" => collection::excludes,
    "any?" => collection::any,
    "all?" => collection::all
}

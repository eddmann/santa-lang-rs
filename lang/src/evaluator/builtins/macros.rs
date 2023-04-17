#[macro_export]
macro_rules! builtin {
    ( $name: ident ($( $parameter: ident ),*, ..$rest: ident) [$evaluator: ident, $source: ident] match { $( $pattern: pat_param => $body: block )* }) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*, $crate::parser::ast::ExpressionKind::RestIdentifier(stringify!($rest).to_owned())]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                let $rest = arguments.get(stringify!($rest)).unwrap();
                match ($( &**arguments.get(stringify!($parameter)).unwrap() ),*, &**arguments.get(stringify!($rest)).unwrap()) {
                    $( $pattern => $body ),*
                    _ => {
                        let mut message = String::new();

                        let arguments = vec![$( (stringify!($parameter), arguments.get(stringify!($parameter)).unwrap().name()) ),*]
                            .iter()
                            .map(|(parameter, argument)| format!("{}", argument))
                            .collect::<Vec<_>>()
                            .join(", ");
                        message.push_str(&format!("Unexpected argument: {}({})", stringify!($name), arguments));

                        Err($crate::evaluator::RuntimeErr { message, source, trace: $evaluator.get_trace() })
                    }
                }
            }
        }
    };
    ( $name: ident (..$rest: ident) [$evaluator: ident, $source: ident] match { $( $pattern: pat_param => $body: block )* }) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$crate::parser::ast::ExpressionKind::RestIdentifier(stringify!($rest).to_owned())]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                let $rest = arguments.get(stringify!($rest)).unwrap();
                match (&**arguments.get(stringify!($rest)).unwrap()) {
                    $( $pattern => $body ),*
                    _ => {
                        let mut message = String::new();

                        let arguments = vec![$( (stringify!($parameter), arguments.get(stringify!($parameter)).unwrap().name()) ),*]
                            .iter()
                            .map(|(parameter, argument)| format!("{}", argument))
                            .collect::<Vec<_>>()
                            .join(", ");
                        message.push_str(&format!("Unexpected argument: {}({})", stringify!($name), arguments));

                        Err($crate::evaluator::RuntimeErr { message, source, trace: $evaluator.get_trace() })
                    }
                }
            }
        }
    };
    ( $name: ident ($( $parameter: ident ),*) match { $( $pattern: pat_param => $body: block )* }) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                match ($( &**arguments.get(stringify!($parameter)).unwrap() ),*) {
                    $( $pattern => $body ),*
                    _ => {
                        let mut message = String::new();

                        let arguments = vec![$( (stringify!($parameter), arguments.get(stringify!($parameter)).unwrap().name()) ),*]
                            .iter()
                            .map(|(parameter, argument)| format!("{}", argument))
                            .collect::<Vec<_>>()
                            .join(", ");
                        message.push_str(&format!("Unexpected argument: {}({})", stringify!($name), arguments));

                        Err($crate::evaluator::RuntimeErr { message, source, trace: evaluator.get_trace() })
                    }
                }
            }
        }
    };
    ( $name: ident ($( $parameter: ident ),*) [$evaluator: ident, $source: ident] match { $( $pattern: pat_param => $body: block )* }) => {
        #[allow(dead_code)]
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                match ($( &**arguments.get(stringify!($parameter)).unwrap() ),*) {
                    $( $pattern => $body ),*
                    _ => {
                        let mut message = String::new();

                        let arguments = vec![$( (stringify!($parameter), arguments.get(stringify!($parameter)).unwrap().name()) ),*]
                            .iter()
                            .map(|(parameter, argument)| format!("{}", argument))
                            .collect::<Vec<_>>()
                            .join(", ");
                        message.push_str(&format!("Unexpected argument: {}({})", stringify!($name), arguments));

                        Err($crate::evaluator::RuntimeErr { message, source, trace: $evaluator.get_trace() })
                    }
                }
            }
        }
    };
    ( $name: ident ($( $parameter: ident ),*) [$evaluator: ident, $source: ident] $body: block ) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                $body
            }
        }
    };
    ( $name: ident ($( $parameter: ident ),*, ..$rest: ident) [$evaluator: ident, $source: ident] $body: block ) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*, $crate::parser::ast::ExpressionKind::RestIdentifier(stringify!($rest).to_owned())]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                let $rest = arguments.get(stringify!($rest)).unwrap();
                $body
            }
        }
    };
    ( $name: ident (..$rest: ident) [$evaluator: ident, $source: ident] $body: block ) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$crate::parser::ast::ExpressionKind::RestIdentifier(stringify!($rest).to_owned())]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                let $evaluator = evaluator;
                let $source = source;
                let $rest = arguments.get(stringify!($rest)).unwrap();
                $body
            }
        }
    };
    ( $name: ident ($( $parameter: ident ),*) $body: block ) => {
        pub mod $name {
            use super::*;

            #[inline]
            pub fn parameters() -> Vec<$crate::parser::ast::ExpressionKind> {
                vec![$( $crate::parser::ast::ExpressionKind::Identifier(stringify!($parameter).to_owned()) ),*]
            }

            #[inline]
            #[allow(unused_variables)]
            pub fn body(evaluator: &mut $crate::evaluator::Evaluator, arguments: std::collections::HashMap<String, Rc<Object>>, source: $crate::lexer::Location) -> Result<std::rc::Rc<$crate::evaluator::object::Object>, $crate::evaluator::RuntimeErr> {
                $( let $parameter = arguments.get(stringify!($parameter)).unwrap(); )*
                $body
            }
        }
    };
}

#[macro_export]
macro_rules! builtins {
    ($( $library: ident :: $name: ident ),*) => {
        pub fn builtins(name: &str) -> Option<std::rc::Rc<$crate::evaluator::Object>> {
            match name {
                $( stringify!($name) => Some(
                    std::rc::Rc::new(
                        $crate::evaluator::Object::Function(
                            $crate::evaluator::Function::Builtin {
                                parameters: $crate::evaluator::builtins::$library::$name::parameters(),
                                body: $crate::evaluator::builtins::$library::$name::body,
                                partial: None
                            }
                        )
                    )
                ), )*
                _ => None
            }
        }
   }
}

#[macro_export]
macro_rules! builtin_aliases {
    ($( $alias: tt => $library: ident :: $name: ident ),*) => {
        pub fn builtin_aliases(name: &str) -> Option<std::rc::Rc<$crate::evaluator::Object>> {
            match name {
                $( $alias => Some(
                    std::rc::Rc::new(
                        $crate::evaluator::Object::Function(
                            $crate::evaluator::Function::Builtin {
                                parameters: $crate::evaluator::builtins::$library::$name::parameters(),
                                body: $crate::evaluator::builtins::$library::$name::body,
                                partial: None
                            }
                        )
                    )
                ), )*
                _ => None
            }
        }
   }
}

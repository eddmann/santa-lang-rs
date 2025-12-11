/// Document intermediate representation for pretty printing.
///
/// This enum represents a "document algebra" that can be rendered to a string
/// with intelligent line-breaking. The printer decides whether content fits
/// on one line (flat mode) or needs to be broken across lines (break mode).
#[derive(Debug, Clone, PartialEq)]
pub enum Doc {
    /// Empty document - produces no output
    Nil,
    /// Literal text - always printed as-is
    Text(String),
    /// Soft line - becomes space in flat mode, newline+indent in break mode
    Line,
    /// Hard line - always becomes newline+indent regardless of mode
    HardLine,
    /// Blank line - always becomes newline without indent
    BlankLine,
    /// Concatenation of documents
    Concat(Vec<Doc>),
    /// Grouping - content that should try to fit on one line
    Group(Box<Doc>),
    /// Nesting - increases indentation level for nested content
    Nest(usize, Box<Doc>),
    /// Conditional - different output for flat vs break mode
    IfBreak { broken: Box<Doc>, flat: Box<Doc> },
}

impl Doc {
    /// Creates a text document from any string-like value.
    #[must_use]
    pub fn text(s: impl Into<String>) -> Doc {
        Doc::Text(s.into())
    }

    /// Creates a soft line (space in flat mode, newline in break mode).
    #[must_use]
    pub fn line() -> Doc {
        Doc::Line
    }

    /// Wraps a document in a group that tries to fit on one line.
    #[must_use]
    pub fn group(doc: Doc) -> Doc {
        Doc::Group(Box::new(doc))
    }

    /// Nests a document with additional indentation.
    #[must_use]
    pub fn nest(indent: usize, doc: Doc) -> Doc {
        Doc::Nest(indent, Box::new(doc))
    }

    /// Concatenates multiple documents, flattening nested concats and filtering Nil.
    #[must_use]
    pub fn concat(docs: Vec<Doc>) -> Doc {
        // Flatten nested concats and filter out Nil
        let flattened: Vec<Doc> = docs
            .into_iter()
            .flat_map(|d| match d {
                Doc::Concat(inner) => inner,
                Doc::Nil => vec![],
                other => vec![other],
            })
            .collect();

        match flattened.len() {
            0 => Doc::Nil,
            1 => flattened.into_iter().next().expect("length is 1"),
            _ => Doc::Concat(flattened),
        }
    }

    /// Creates a conditional document with different output for flat vs break mode.
    #[must_use]
    pub fn if_break(broken: Doc, flat: Doc) -> Doc {
        Doc::IfBreak {
            broken: Box::new(broken),
            flat: Box::new(flat),
        }
    }

    /// Joins documents with a separator between each.
    #[must_use]
    pub fn join(docs: Vec<Doc>, sep: Doc) -> Doc {
        if docs.is_empty() {
            return Doc::Nil;
        }

        let mut result = Vec::with_capacity(docs.len() * 2 - 1);
        let mut first = true;

        for doc in docs {
            if !first {
                result.push(sep.clone());
            }
            result.push(doc);
            first = false;
        }

        Doc::concat(result)
    }

    /// Creates a soft line that becomes nothing in flat mode, newline in break mode.
    #[must_use]
    pub fn soft_line() -> Doc {
        Doc::if_break(Doc::Line, Doc::Nil)
    }

    /// Creates a bracketed group with smart line-breaking for collections.
    #[must_use]
    pub fn bracketed(open: &str, docs: Vec<Doc>, close: &str, trailing_comma: bool) -> Doc {
        if docs.is_empty() {
            return Doc::concat(vec![Doc::text(open), Doc::text(close)]);
        }

        // Separator: ", " in flat mode, ",\n" in broken mode
        let sep = Doc::concat(vec![Doc::text(","), Doc::if_break(Doc::Line, Doc::text(" "))]);

        // Trailing comma only in broken mode
        let trailing = if trailing_comma {
            Doc::if_break(Doc::text(","), Doc::Nil)
        } else {
            Doc::Nil
        };

        Doc::group(Doc::concat(vec![
            Doc::text(open),
            Doc::nest(2, Doc::concat(vec![Doc::soft_line(), Doc::join(docs, sep), trailing])),
            Doc::soft_line(),
            Doc::text(close),
        ]))
    }
}

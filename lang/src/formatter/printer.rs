use super::doc::Doc;

/// Maximum line width for formatted output.
/// Lines exceeding this width will be broken into multiple lines when possible.
const LINE_WIDTH: usize = 100;

/// Pre-allocated indentation buffer to avoid repeated string allocations.
/// Contains 100 spaces, sufficient for most indentation levels (LINE_WIDTH / 2 = 50 indent levels).
const INDENT_BUFFER: &str =
    "                                                                                                    ";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Flat,
    Break,
}

/// Command for the printer's work stack.
/// Uses references to avoid cloning Doc nodes during printing.
#[derive(Debug, Clone, Copy)]
struct Cmd<'a> {
    indent: usize,
    mode: Mode,
    doc: &'a Doc,
}

impl<'a> Cmd<'a> {
    fn new(indent: usize, mode: Mode, doc: &'a Doc) -> Self {
        Self { indent, mode, doc }
    }
}

/// Returns a string of spaces for the given indentation level.
/// Uses a pre-allocated buffer to avoid allocations for common indent levels.
#[inline]
fn indent_str(indent: usize) -> &'static str {
    &INDENT_BUFFER[..indent.min(INDENT_BUFFER.len())]
}

/// Renders a Doc to a formatted string with intelligent line-breaking.
pub fn print(doc: &Doc) -> String {
    let mut output = String::new();
    let mut column = 0;
    let mut cmds = vec![Cmd::new(0, Mode::Break, doc)];

    while let Some(cmd) = cmds.pop() {
        match cmd.doc {
            Doc::Nil => {}

            Doc::Text(s) => {
                output.push_str(s);
                column += s.len();
            }

            Doc::Line => {
                if cmd.mode == Mode::Flat {
                    output.push(' ');
                    column += 1;
                } else {
                    output.push('\n');
                    output.push_str(indent_str(cmd.indent));
                    column = cmd.indent;
                }
            }

            Doc::HardLine => {
                output.push('\n');
                output.push_str(indent_str(cmd.indent));
                column = cmd.indent;
            }

            Doc::BlankLine => {
                output.push('\n');
                column = 0;
            }

            Doc::Concat(docs) => {
                for d in docs.iter().rev() {
                    cmds.push(Cmd::new(cmd.indent, cmd.mode, d));
                }
            }

            Doc::Nest(n, inner) => {
                cmds.push(Cmd::new(cmd.indent + n, cmd.mode, inner));
            }

            Doc::Group(inner) => {
                if cmd.mode == Mode::Flat {
                    cmds.push(Cmd::new(cmd.indent, Mode::Flat, inner));
                } else {
                    let flat_width = measure_flat(inner, LINE_WIDTH.saturating_sub(column));
                    if flat_width.is_some() {
                        cmds.push(Cmd::new(cmd.indent, Mode::Flat, inner));
                    } else {
                        cmds.push(Cmd::new(cmd.indent, Mode::Break, inner));
                    }
                }
            }

            Doc::IfBreak { broken, flat } => {
                if cmd.mode == Mode::Flat {
                    cmds.push(Cmd::new(cmd.indent, cmd.mode, flat));
                } else {
                    cmds.push(Cmd::new(cmd.indent, cmd.mode, broken));
                }
            }
        }
    }

    output
}

fn measure_flat(doc: &Doc, remaining: usize) -> Option<usize> {
    let mut width = 0;
    let mut stack = vec![doc];

    while let Some(d) = stack.pop() {
        if width > remaining {
            return None;
        }

        match d {
            Doc::Nil => {}
            Doc::Text(s) => width += s.len(),
            Doc::Line => width += 1,
            Doc::HardLine | Doc::BlankLine => return None,
            Doc::Concat(docs) => {
                for d in docs.iter().rev() {
                    stack.push(d);
                }
            }
            Doc::Nest(_, inner) => stack.push(inner),
            Doc::Group(inner) => stack.push(inner),
            Doc::IfBreak { flat, .. } => stack.push(flat),
        }
    }

    Some(width)
}

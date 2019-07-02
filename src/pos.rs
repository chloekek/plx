use std::path::Path;

/// A position in the source code is tracked for each abstract syntax tree node.
/// This allows diagnostics and debug information to point to relevant places in
/// the source code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos<'a>
{
    pub file: &'a Path,
    pub line: usize,
}

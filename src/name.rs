/// An identifier defines or refers to a feature.
///
/// Identifiers typically occur literally in the source code being compiled.
/// There are no restrictions as to which code points occur within an identifier,
/// and even the empty identifier is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id<'a>
{
    pub name: &'a str,
}

/// The name of a namespace is a list of identifiers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ns<'a>
{
    pub segments: &'a [Id<'a>],
}

/// A qualified identifier is an identifier paired with the name of a namespace.
///
/// A qualified identifier refers to a particular globally-defined feature
/// unambiguously.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Qid<'a>
{
    pub namespace: Ns<'a>,
    pub member: Id<'a>,
}

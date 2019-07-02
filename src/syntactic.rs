//! A syntactic abstract syntax tree represents a program that was just parsed
//! from source code.
//!
//! No type information is attached and names have not been disambiguated.

use name::Id;
use pos::Pos;

/// Top-level definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Def<'a>
{
    Namespace{
        pos: Pos<'a>,
        /// The name of a namespace definition is relative to the name of the
        /// parent namespace definition; it is not absolute unless the namespace
        /// definition is not nested inside another namespace definition.
        name: &'a [Id<'a>],
        members: &'a [&'a Def<'a>],
    },

    Value{
        pos: Pos<'a>,
        name: Id<'a>,
        signature: &'a Type<'a>,
        definition: &'a Term<'a>,
    },
}

/// Type-level expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type<'a>
{
    Variable{
        pos: Pos<'a>,
        name: Id<'a>,
    },

    Apply{
        pos: Pos<'a>,
        constructor: &'a Type<'a>,
        argument: &'a Type<'a>,
    },

    Row{
        pos: Pos<'a>,
        members: &'a [(Id<'a>, &'a Type<'a>)],
        tail: &'a Type<'a>,
    },

    ForAll{
        pos: Pos<'a>,
        variable: Id<'a>,
        quantified: &'a Type<'a>,
    },

    /// A member type is a type such as <em>foo.bar</em>. The part before the
    /// period is only syntactically type-like; in reality it can only be the
    /// name of a namespace.
    Member{
        pos: Pos<'a>,
        parent: &'a Type<'a>,
        member: Id<'a>,
    },
}

/// Term-level expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term<'a>
{
    Variable{
        pos: Pos<'a>,
        name: Id<'a>,
    },

    Abstract{
        pos: Pos<'a>,
        parameters: &'a [Id<'a>],
        body: &'a Term<'a>,
    },

    Apply{
        pos: Pos<'a>,
        constructor: &'a Term<'a>,
        argument: &'a Term<'a>,
    },

    Record{
        pos: Pos<'a>,
        fields: &'a [(Id<'a>, &'a Term<'a>)],
    },

    Variant{
        pos: Pos<'a>,
        tag: Id<'a>,
        value: &'a Term<'a>,
    },

    /// A member term is a term such as <em>foo.bar</em>. The part before the
    /// period may be either another term or the name of a namespace.
    Member{
        pos: Pos<'a>,
        parent: &'a Term<'a>,
        member: Id<'a>,
    },

    Match{
        pos: Pos<'a>,
        scrutinee: &'a Term<'a>,
        cases: &'a [(Id<'a>, Id<'a>, &'a Term<'a>)],
    },

    If{
        pos: Pos<'a>,
        condition: &'a Term<'a>,
        consequence: &'a Term<'a>,
        alternative: &'a Term<'a>,
    },

    AndAlso{
        pos: Pos<'a>,
        left: &'a Term<'a>,
        right: &'a Term<'a>,
    },

    OrElse{
        pos: Pos<'a>,
        left: &'a Term<'a>,
        right: &'a Term<'a>,
    },
}

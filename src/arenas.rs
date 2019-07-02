use typed_arena::Arena;

use name::Id;
use name::Ns;
use std::str;

use syntactic as sy;

/// The compiler uses arena allocation for the data structures that represent the
/// program being compiled. This eases the implementation because no care needs
/// to be taken about dropping data. It is also faster than any alternative
/// allocation strategy since we never drop the data.
pub struct Arenas<'a>
{
    pub u8: &'a Arena<u8>,
    pub Id: &'a Arena<Id<'a>>,
    pub syDef: &'a Arena<sy::Def<'a>>,
    pub syType: &'a Arena<sy::Type<'a>>,
    pub syTerm: &'a Arena<sy::Term<'a>>,
}

impl<'a> Arenas<'a>
{
    pub fn allocId<'b>(&self, name: &'b str) -> Id<'a>
    {
        let utf8Bytes = self.u8.alloc_extend(name.bytes());
        let string = unsafe { str::from_utf8_unchecked(utf8Bytes) };
        Id{name: string}
    }

    pub fn allocNs<I>(&self, segments: I) -> Ns<'a>
        where I: IntoIterator<Item=Id<'a>>
    {
        Ns{segments: self.Id.alloc_extend(segments)}
    }

    pub fn allocSyDef(&self, def: sy::Def<'a>) -> &'a sy::Def
    {
        self.syDef.alloc(def)
    }

    pub fn allocSyType(&self, type_: sy::Type<'a>) -> &'a sy::Type
    {
        self.syType.alloc(type_)
    }

    pub fn allocSyTerm(&self, term: sy::Term<'a>) -> &'a sy::Term
    {
        self.syTerm.alloc(term)
    }
}

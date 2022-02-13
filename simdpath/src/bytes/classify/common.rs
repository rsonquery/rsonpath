#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Structural {
    Closing(usize),
    Colon(usize),
    Opening(usize),
}
use Structural::*;

impl Structural {
    #[inline(always)]
    pub fn idx(self) -> usize {
        match self {
            Closing(idx) => idx,
            Colon(idx) => idx,
            Opening(idx) => idx,
        }
    }

    #[inline(always)]
    pub fn offset(self, amount: usize) -> Self {
        match self {
            Closing(idx) => Closing(idx + amount),
            Colon(idx) => Colon(idx + amount),
            Opening(idx) => Opening(idx + amount),
        }
    }
}

pub trait StructuralIterator<'a>: Iterator<Item = Structural> + len_trait::Empty + 'a {}

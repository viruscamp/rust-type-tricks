use crate::{W, True, False};

pub trait IsCopy {
    type const Impled: bool;
    type Answer;
}

impl<T> IsCopy for W<T> {
    // due to https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md#hazard-interactions-with-type-checking
    // an associated type from a blanket impl must be treated "opaquely" if it's marked default
    default type const Impled: bool = false;
    default type Answer = False;
}

impl<T: Copy> IsCopy for W<T> {
    type const Impled: bool = true;
    type Answer = True;
}

pub trait NotCopy : IsCopy {}
impl<T: ?Sized + IsCopy<Answer = False>> NotCopy for T {}

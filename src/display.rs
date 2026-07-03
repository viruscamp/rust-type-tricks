use crate::{NamedImplBase, is::Is};

use core::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

pub trait NamedDisplay: NamedImplBase {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub trait NamedDisplayProvider: NamedImplBase
    where <Self::Impl as NamedImplBase>::Target: Is<Type = Self::Target>
{
    type Impl: NamedDisplay;
}

impl<N: NamedDisplay> NamedDisplayProvider for N {
    type Impl = Self;
}

impl<NP: NamedDisplayProvider, const ImplDeref: bool> Display for crate::Wrap<NP, ImplDeref> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(Is::from_ref(&self.0), f)
    }
}

pub struct DefaultDisplay<T: Display>(PhantomData<T>);
impl<T: Display> NamedImplBase for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display> NamedDisplay for DefaultDisplay<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(this, f)
    }
}

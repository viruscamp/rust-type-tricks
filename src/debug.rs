use crate::{NamedImplBase, is::Is};

use core::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData;

pub trait NamedDebug: NamedImplBase {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result;
}

pub trait NamedDebugProvider: NamedImplBase
    where <Self::Impl as NamedImplBase>::Target: Is<Type = Self::Target>
{
    type Impl: NamedDebug;
}

impl<N: NamedDebug> NamedDebugProvider for N {
    type Impl = Self;
}

// https://github.com/rust-lang/rust/issues/124449
impl<NP: NamedDebugProvider, const ImplDeref: bool> Debug for crate::Wrap<NP, ImplDeref> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        NP::Impl::fmt(Is::from_ref(&self.0), f)
    }
}

pub struct DefaultDebug<T: Debug>(PhantomData<T>);
impl<T: Debug> NamedImplBase for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug> NamedDebug for DefaultDebug<T> {
    fn fmt(this: &Self::Target, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(this, f)
    }
}

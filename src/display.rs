use crate::{ShadowTrait, Named, Wrap};

use bytemuck::TransparentWrapper;
use core::fmt::{Display, Formatter, Result};
use core::marker::PhantomData;

pub trait DisplayProvider: ShadowTrait
where
    Named<Self::Impl>: Display,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> DisplayProvider for N
where
    N: ShadowTrait,
    Named<N>: Display
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> Display for Wrap<NP, ImplDeref>
where
    NP: DisplayProvider,
    Named<NP::Impl>: Display,
    NP::Impl: ShadowTrait<Target = NP::Target>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::fmt(Named::wrap_ref(&self.0), f)
    }
}

pub struct DefaultDisplay<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display + ?Sized> Display for Named<DefaultDisplay<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(&self.0, f)
    }
}

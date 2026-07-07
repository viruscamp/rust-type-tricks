use crate::{ShadowTrait, Wrap, Named};

use bytemuck::TransparentWrapper;
use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;

pub trait DebugProvider: ShadowTrait
where
    Named<Self::Impl>: Debug,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> DebugProvider for N
where
    N: ShadowTrait,
    Named<N>: Debug
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> Debug for Wrap<NP, ImplDeref>
where
    NP: DebugProvider,
    Named<NP::Impl>: Debug,
    NP::Impl: ShadowTrait<Target = NP::Target>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Named::fmt(Named::wrap_ref(&self.0), f)
    }
}

pub struct DefaultDebug<T: Debug + ?Sized>(PhantomData<T>);
impl<T: Debug + ?Sized> ShadowTrait for DefaultDebug<T> {
    type Target = T;
}
impl<T: Debug + ?Sized> Debug for Named<DefaultDebug<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Debug>::fmt(&self.0, f)
    }
}

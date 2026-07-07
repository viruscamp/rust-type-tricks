#![no_std]
#![feature(fundamental)]
#![feature(with_negative_coherence)]

// https://internals.rust-lang.org/t/pre-rfc-forward-impls/4628/29
// named impl base
// likes `core::ops::Receiver`, can we use `Receiver`?
pub trait ShadowTrait {
    type Target: ?Sized;
}

#[fundamental]
#[repr(transparent)]
pub struct Named<N: ShadowTrait>(pub N::Target);

unsafe impl<N: ShadowTrait> bytemuck::TransparentWrapper<N::Target>
    for Named<N>
{
}

impl<N> Clone for Named<N>
where
    N: ShadowTrait,
    N::Target: Copy,
{
    fn clone(&self) -> Self {
        Named(self.0)
    }
}

impl<N> Copy for Named<N>
where
    N: ShadowTrait,
    N::Target: Copy,
{
}

pub mod wrap;
pub mod is;
pub mod display;
pub mod debug;

pub use wrap::Wrap;

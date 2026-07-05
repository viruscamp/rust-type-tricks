//#![no_std]
#![feature(fundamental)]
#![feature(with_negative_coherence)]

// https://internals.rust-lang.org/t/pre-rfc-forward-impls/4628/29
// named impl base
// likes `core::ops::Receiver`, can we use `Receiver`?
pub trait ShadowTrait {
    type Target: ?Sized;
}

pub mod wrap;
pub mod is;
pub mod display;
pub mod debug;

pub use wrap::Wrap;
pub use is::Is;

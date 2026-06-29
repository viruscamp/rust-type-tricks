#![no_std]
#![feature(fundamental)]
#![feature(negative_impls)]
#![feature(specialization)]
#![feature(min_generic_const_args)]

// https://internals.rust-lang.org/t/pre-rfc-forward-impls/4628/29
// named impl base
pub trait NamedImplBase {
    type Target: ?Sized;
}

pub trait TrueTrait {}
pub struct True;
impl TrueTrait for True {}
impl !FalseTrait for True {}

pub trait FalseTrait {}
pub enum False {}
impl !TrueTrait for False {}
impl FalseTrait for False {}

pub struct W<T: ?Sized>(core::marker::PhantomData<T>);

pub mod wrap;
pub mod wrap_ref;
pub mod wrap_mut;
pub mod is;
pub mod is_type;
pub mod is_copy;

pub use wrap::Wrap;
pub use wrap_ref::WrapRef;
pub use wrap_mut::WrapMut;

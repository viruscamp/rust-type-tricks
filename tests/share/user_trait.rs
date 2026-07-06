use std::marker::PhantomData;

use bytemuck::TransparentWrapper;
use shadow_traits::{Named, ShadowTrait, Wrap, Is};

pub trait UserSuper: Sync + Send + Copy {
    fn new() -> Self;
    fn consume(self);
}

pub trait UserSuperProvider: ShadowTrait
where
    Named<Self::Impl>: UserSuper,
    Self::Impl: ShadowTrait,
    Self::Target: Is<Type = <Self::Impl as ShadowTrait>::Target>,
{
    type Impl;
}

impl<N> UserSuperProvider for N
where
    N: ShadowTrait,
    Named<N>: UserSuper
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> UserSuper for Wrap<NP, ImplDeref>
where
    NP: UserSuperProvider,
    Named<NP::Impl>: UserSuper,
    NP::Impl: ShadowTrait,
    NP::Target: Is<Type = <NP::Impl as ShadowTrait>::Target>,
    NP::Target: Sync + Send + Copy,
    <NP::Impl as ShadowTrait>::Target: Sync + Send + Copy,
{
    fn new() -> Self {
        let a = Named::new();
        let b = a.0;
        let c = <NP::Target as Is>::to_left(b);
        Wrap::new(c)
    }

    fn consume(self) {
        let a = self.0;
        let b = <NP::Target as Is>::to_right(a);
        let c = Named::wrap(b);
        Named::consume(c)
    }
}

pub struct DefaultUserSuper<T: UserSuper>(PhantomData<T>);
impl<T: UserSuper> ShadowTrait for DefaultUserSuper<T> {
    type Target = T;
}
impl<T: UserSuper> UserSuper for Named<DefaultUserSuper<T>> {
    fn new() -> Self {
        Named::wrap(T::new())
    }

    fn consume(self) {
        T::consume(self.0)
    }
}

pub trait UserTrait : UserSuper {
    fn use_ref(&self);
    fn return_ref() -> &'static Self {
        let b = Box::new(Self::new());
        Box::leak(b)
    }
}

pub trait UserTraitProvider: ShadowTrait
where
    Named<Self::Impl>: UserTrait,
    Self::Impl: ShadowTrait,
    Self::Target: Is<Type = <Self::Impl as ShadowTrait>::Target>,
{
    type Impl;
}

impl<N> UserTraitProvider for N
where
    N: ShadowTrait,
    Named<N>: UserTrait
{
    type Impl = Self;
}

impl<NP, const ImplDeref: bool> UserTrait for Wrap<NP, ImplDeref>
where
    Self: UserSuper,
    NP: UserTraitProvider,
    Named<NP::Impl>: UserTrait,
    NP::Impl: ShadowTrait,
    NP::Target: Is<Type = <NP::Impl as ShadowTrait>::Target>,
    NP::Target: Sync + Send + Copy,
    <NP::Impl as ShadowTrait>::Target: Sync + Send + Copy,
{
    fn use_ref(&self) {
        let a = &self.0;
        let b = <NP::Target as Is>::to_ref_right(a);
        let c = Named::wrap_ref(b);
        Named::use_ref(c)
    }
    
    fn return_ref() -> &'static Self {
        let a = Named::return_ref();
        let b = &a.0;
        let c = <NP::Target as Is>::to_ref_left(b);
        Self::wrap_ref(c)
    }
}

pub struct DefaultUserTrait<T: UserTrait>(PhantomData<T>);
impl<T: UserTrait> ShadowTrait for DefaultUserTrait<T> {
    type Target = T;
}
impl<T: UserTrait> UserSuperProvider for DefaultUserTrait<T> {
    type Impl = DefaultUserSuper<T>;
}
impl<T: UserTrait> UserTrait for Named<DefaultUserTrait<T>>
    where Self: UserSuper
{
    fn use_ref(&self) {
        T::use_ref(&self.0)
    }

    fn return_ref() -> &'static Self {
        Named::wrap_ref(T::return_ref())
    }
}

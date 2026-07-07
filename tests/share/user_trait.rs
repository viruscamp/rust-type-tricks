use std::marker::PhantomData;

use bytemuck::TransparentWrapper;
use shadow_traits::{Named, ShadowTrait, Wrap};

pub trait UserSuper: Sync + Send + Copy {
    fn new() -> Self;
    fn consume(self);
}

pub trait UserSuperProvider: ShadowTrait
where
    Named<Self::Impl>: UserSuper,
    Self::Impl: ShadowTrait<Target = Self::Target>,
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
    NP::Impl: ShadowTrait<Target = NP::Target>,
    NP::Target: Sync + Send + Copy,
{
    fn new() -> Self {
        Wrap::new(Named::new().0)
    }

    fn consume(self) {
        Named::consume(Named::wrap(self.0))
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
    Self::Impl: ShadowTrait<Target = Self::Target>,
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
    NP::Impl: ShadowTrait<Target = NP::Target>,
    NP::Target: Sync + Send + Copy,
{
    fn use_ref(&self) {
        let c = Named::wrap_ref(&self.0);
        Named::use_ref(c)
    }
    
    fn return_ref() -> &'static Self {
        let a = Named::return_ref();
        let b = &a.0;
        Self::wrap_ref(b)
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

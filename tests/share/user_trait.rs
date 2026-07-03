use std::marker::PhantomData;

use bytemuck::TransparentWrapper;
use type_tricks::{NamedImplBase, Wrap, is::Is};

pub trait UserSuper: Sync + Send + Copy {
    fn new() -> Self;
    fn consume(self);
}

pub trait NamedUserSuper: NamedImplBase
    where Self::Target : Sync + Send + Copy
{
    fn new() -> Self::Target;
    fn consume(this: Self::Target);
}

pub trait NamedUserSuperProvider
    where <Self::Impl as NamedImplBase>::Target: Sync + Send + Copy
{
    type Impl: NamedUserSuper;
}

impl<N: NamedUserSuper> NamedUserSuperProvider for N
    where <N as NamedImplBase>::Target: Sync + Send + Copy
{
    type Impl = Self;
}

impl<NP> UserSuper for Wrap<NP>
where
    Self: Sync + Send + Copy,
    NP: NamedImplBase + NamedUserSuperProvider,
    NP::Target: Sync + Send + Copy,
    NP::Target: Is<Type = <NP::Impl as NamedImplBase>::Target>,
{
    fn new() -> Self {
        let a = <NP::Impl as NamedUserSuper>::new();
        let b: NP::Target = <NP::Target as Is>::from_val(a);
        Wrap::new(b)
    }

    fn consume(self) {
        let a = <NP::Target as Is>::into_val(self.0);
        <NP::Impl as NamedUserSuper>::consume(a)
    }
}

pub struct DefaultUserSuper<T: UserSuper>(PhantomData<T>);
impl<T: UserSuper> NamedImplBase for DefaultUserSuper<T> {
    type Target = T;
}
impl<T: UserSuper> NamedUserSuper for DefaultUserSuper<T> {
    fn new() -> Self::Target {
        T::new()
    }

    fn consume(this: Self::Target) {
        T::consume(this)
    }
}

pub trait UserTrait : UserSuper {
    fn use_ref(&self);
    fn return_ref() -> &'static Self {
        let b = Box::new(Self::new());
        Box::leak(b)
    }
}

pub trait NamedUserTrait: NamedImplBase + NamedUserSuper
    where Self::Target : Sync + Send + Copy
{
    fn use_ref(this: &Self::Target);
    fn return_ref() -> &'static Self::Target {
        let b = Box::new(Self::new());
        Box::leak(b)
    }
}

pub trait NamedUserTraitProvider
    where <Self::Impl as NamedImplBase>::Target: Sync + Send + Copy
{
    type Impl: NamedUserTrait;
}

impl<N: NamedUserTrait> NamedUserTraitProvider for N
    where <N as NamedImplBase>::Target: Sync + Send + Copy
{
    type Impl = Self;
}

impl<NP> UserTrait for Wrap<NP>
where
    Self: Sync + Send + Copy + UserSuper,
    NP: NamedImplBase + NamedUserTraitProvider,
    NP::Target: Sync + Send + Copy,
    NP::Target: Is<Type = <NP::Impl as NamedImplBase>::Target>,
{
    fn use_ref(&self) {
        let a = <NP::Target as Is>::to_ref(&self.0);
        <NP::Impl as NamedUserTrait>::use_ref(a)
    }
    
    fn return_ref() -> &'static Self {
        let a = <NP::Impl as NamedUserTrait>::return_ref();
        let b = <NP::Target as Is>::from_ref(a);
        <Self as TransparentWrapper<_>>::wrap_ref(b)
    }
}

pub struct DefaultUserTrait<T: UserTrait>(PhantomData<T>);
impl<T: UserTrait> NamedImplBase for DefaultUserTrait<T> {
    type Target = T;
}
impl<T: UserTrait> NamedUserSuper for DefaultUserTrait<T> {
    fn new() -> Self::Target {
        <T as UserSuper>::new()
    }

    fn consume(this: Self::Target) {
        <T as UserSuper>::consume(this)
    }
}
impl<T: UserTrait> NamedUserTrait for DefaultUserTrait<T> {
    fn use_ref(this: &Self::Target) {
        <T as UserTrait>::use_ref(this)
    }

    fn return_ref() -> &'static Self::Target {
        <T as UserTrait>::return_ref()
    }
}

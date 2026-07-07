# An experiment lib to simulate the named impl draft

There are lots draft or post try to relax the orphan rule. One of them is my
[Draft: Named impl with Implementation Selection Variant](https://internals.rust-lang.org/t/named-impl-with-implementation-selection-variant/24374). After hard working, I have wrote this lib to simulate most features in current Rust Edition.  

NEW!! `shadow trait` was totally removed.

## 1. base trait, should be included in rust core lib
provides: `ShadowTrait`  
```rust
// need a new name
pub trait ShadowTrait {
    type Target: ?Sized;
}
```

## 2. wrap, should be included in rust core lib
depends: `ShadowTrait`  
provides: `Named`, `Wrap`  
```rust
// used to define a named impl, need a clear name
#[fundamental]
#[repr(transparent)]
pub struct Named<N: ShadowTrait>(pub N::Target);

// the final wrap user uses
#[fundamental]
#[repr(transparent)]
pub struct Wrap<N: ShadowTrait>(pub N::Target);
```

## 3. traits provider
depends: `ShadowTrait`
  It should be defined along with the orignal Trait
```rust
// required, delegate trait on Wrap to named-impl, external crates cannot do this
pub trait DisplayProvider: ShadowTrait
where
    Named<Self::Impl>: Display,
    Self::Impl: ShadowTrait<Target = Self::Target>,
{
    type Impl;
}

impl<N> DisplayProvider for N
where
    Named<N>: Display,
    N: ShadowTrait,
{
    type Impl = Self;
}

impl<NP: DisplayProvider> Display for Wrap<NP, ImplDeref> {}

// optinal, a default impl
pub struct DefaultDisplay<T: Display + ?Sized>(PhantomData<T>);
impl<T: Display + ?Sized> ShadowTrait for DefaultDisplay<T> {
    type Target = T;
}
impl<T: Display + ?Sized> Display for Named<DefaultDisplay<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <T as Display>::fmt(&self.0, f)
    }
}
```

## 4. named impls provider, should be an external lib
depends `ShadowTrait` and `Named`
```rust
pub struct DisplayImpl1;
impl ShadowTrait for DisplayImpl1 {
    type Target = i32;
}
impl Display for Named<DisplayImpl1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("DisplayImpl1")
    }
}

pub struct DisplayImplProxy<T: Display>(PhantomData<T>);
impl<T: Display> ShadowTrait for DisplayImplProxy<T> {
    type Target = T;
}
impl<T: Display> Display for Named<DisplayImplProxy<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("Display Pre ")?;
        self.0.fmt(f)?;
        f.write_str(" Post")?;
        Ok(())
    }
}
```

## 5. comsumer
```rust
pub struct SimpleMultipleTag;
impl ShadowTrait for SimpleMultipleTag {
    type Target = i32;
}
impl DisplayProvider for SimpleMultipleTag {
    type Impl = DisplayImplProxy<i32>;
}
impl DebugProvider for SimpleMultipleTag {
    type Impl = DebugImpl1;
}

#[test]
fn test_simple_multiple() {
    let num = 42;
    let a1 = Wrap::<SimpleMultipleTag>::wrap_ref(&num);
    assert_eq!(format!("{a1}"), "Display Pre 42 Post");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");
}
```

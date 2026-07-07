use core::marker::PhantomData;
use std::fmt::{Debug,Display};

use bytemuck::TransparentWrapper;
use shadow_traits::Named;
use shadow_traits::ShadowTrait;
use shadow_traits::Wrap;

mod share;

use share::named_display_impls::*;
use share::named_debug_impls::*;

use shadow_traits::display::DefaultDisplay;
use shadow_traits::display::DisplayProvider;
use shadow_traits::debug::DebugProvider;

pub struct MultipleImplSelector<T, N1, N2>
(PhantomData<T>, PhantomData<N1>, PhantomData<N2>);

impl<T, N1, N2> ShadowTrait for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowTrait<Target = T>,
    Named<N1>: Display,
    N2: ShadowTrait<Target = T>,
    Named<N2>: Debug,
{
    type Target = T;
}

impl<T, N1, N2> DisplayProvider for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowTrait<Target = T>,
    Named<N1>: Display,
    N2: ShadowTrait<Target = T>,
    Named<N2>: Debug,
{
    type Impl = N1;
}

impl<'a, T, N1, N2> DebugProvider for MultipleImplSelector<T, N1, N2>
where
    N1: ShadowTrait<Target = T>,
    Named<N1>: Display,
    N2: ShadowTrait<Target = T>,
    Named<N2>: Debug,
{
    type Impl = N2;
}

#[test]
fn test_named_impl_multiple() {
    let num = 42;

    // note: to_string() calls Display, format!("{:?}") calls Debug
    
    let a1 = Wrap::<MultipleImplSelector::<i32, DisplayImpl1, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a1.to_string(), "DisplayImpl1");
    assert_eq!(format!("{a1:?}"), "DebugImpl1");

    let a2 = Wrap::<MultipleImplSelector::<i32, DisplayImplProxy<i32>, DebugImpl1>>::wrap_ref(&num);
    assert_eq!(a2.to_string(), "Display Pre 42 Post");
    assert_eq!(format!("{a2:?}"), "DebugImpl1");

    let a3 = Wrap::<MultipleImplSelector::<i32, DefaultDisplay<i32>, DebugImplProxy<i32>>>::wrap_ref(&num);
    assert_eq!(a3.to_string(), "42");
    assert_eq!(format!("{a3:?}"), "Debug Pre 42 Post");
}

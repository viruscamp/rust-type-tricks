pub trait Is {
    type Type: ?Sized;

    fn into_ref(&self) -> &Self::Type;
    fn into_mut_ref(&mut self) -> &mut Self::Type;
    fn from_ref(x: &Self::Type) -> &Self;
    fn from_mut_ref(x: &mut Self::Type) -> &mut Self;
}

impl<T: ?Sized> Is for T {
    type Type = T;

    fn into_ref(&self) -> &Self::Type {
        self
    }

    fn into_mut_ref(&mut self) -> &mut Self::Type {
        self
    }

    fn from_ref(x: &Self::Type) -> &Self {
        x
    }

    fn from_mut_ref(x: &mut Self::Type) -> &mut Self {
        x
    }
}

// https://docs.rs/higher-kinded-types/latest/src/higher_kinded_types/advanced/type_eq.rs.html#99-100

pub fn into_val<T>(it: T) -> <T as Is>::Type
{
    it
}

pub fn from_val<T>(x: <T as Is>::Type) -> T {
    x
}

#[cfg(feature = "alloc")]
pub fn into_box_val<T>(it: Box<T>) -> Box<<T as Is>::Type>
{
    it
}

#[cfg(feature = "alloc")]
pub fn from_box_val<T>(x: Box<<T as Is>::Type>) -> Box<T> {
    x
}

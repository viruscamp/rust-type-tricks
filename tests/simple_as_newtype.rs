use type_tricks::{NamedImplBase, Wrap, debug::NamedDebugProvider};

mod share;
use share::named_debug_impls::NamedDebugProxy;

struct WrapI32Tag;
impl NamedImplBase for WrapI32Tag {
    type Target = i32;
}
/// For this type, I donot want it to be a transparent wrapper, 
/// so Deref is disabled with ImplDeref=false
type WrapI32 = Wrap<WrapI32Tag, false>;

// impl trait directly
impl ToString for WrapI32 {
    fn to_string(&self) -> String {
        format!("WrapI32({})", self.0)
    }
}

// impl trait by delegation to a named impl
impl NamedDebugProvider for WrapI32Tag {
    type Impl = NamedDebugProxy<i32>;
}

#[test]
fn test_wrap_as_newtype() {
    let wrap = WrapI32::new(42);
    assert_eq!(wrap.to_string(), "WrapI32(42)");
    assert_eq!(format!("{wrap:?}"), "Debug Pre 42 Post");
}

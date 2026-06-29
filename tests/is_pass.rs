use type_tricks::is::*;

struct Test<U, T: Is<Type = U>>(U, T);

impl<U, T: Is<Type = U>> Test<U, T> {
    fn check(self) {
        let a: U = self.0;
        let b: T = self.1;
    }

    fn swap(self) {        
        let c: U = into_val(self.1);
        let d: T = from_val(self.0);
    }
}

use Step;

macro_rules! primitive_impl {
    ($t:ty) => {
        impl Step for $t {
            fn step(&self) -> Option<Self> {
                self.checked_add(1 as $t)
            }

            fn step_by(&self, by: &Self) -> Option<Self> {
                self.checked_add(*by)
            }

            fn step_back(&self) -> Option<Self> {
                self.checked_sub(1 as $t)
            }

            fn step_back_by(&self, by: &Self) -> Option<Self> {
                self.checked_sub(*by)
            }
        }
    }
}

primitive_impl!(i8);
primitive_impl!(i16);
primitive_impl!(i32);
primitive_impl!(i64);
primitive_impl!(isize);
primitive_impl!(u8);
primitive_impl!(u16);
primitive_impl!(u32);
primitive_impl!(u64);
primitive_impl!(usize);

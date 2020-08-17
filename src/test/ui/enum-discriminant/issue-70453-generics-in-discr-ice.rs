#![feature(core_intrinsics)]

extern crate core;
use core::intrinsics::discriminant_value;

#[repr(usize)]
enum MyWeirdOption<T> {
//~^ ERROR parameter `T` is never used
    None = 0,
    Some = std::mem::size_of::<T>(),
    //~^ ERROR constant expression depends on a generic parameter
}

fn main() {
    assert_eq!(discriminant_value(&MyWeirdOption::<u8>::None), 0);
    assert_eq!(discriminant_value(&MyWeirdOption::<u8>::Some), 1);
}

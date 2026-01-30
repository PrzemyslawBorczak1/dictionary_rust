extern crate libc;

pub mod avl_helpers;
pub mod dict;
pub mod ffi;
pub mod string;

pub use dict::Dictionary;
pub use string::MyString;

#[macro_export]
macro_rules! dict {
    ( $( { $k:expr, $v:expr } ),* $(,)? ) => {{
        let mut d = $crate::Dictionary::new();
        $(
            d.insert($k, $crate::MyString::from_str($v));
        )*
        d
    }};
}

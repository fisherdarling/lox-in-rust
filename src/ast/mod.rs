pub(crate) mod ast;
pub(crate) mod operator;
pub(crate) mod printer;
pub(crate) mod span;
pub(crate) mod visit;
pub(crate) mod function;

pub use self::ast::*;

#[macro_export]
macro_rules! impl_try_from {
    ($($name:ident < $from:ident :: $var:ident),+$(,)?) => {
       $(
            impl std::convert::TryFrom<$from> for $name {
                type Error = $crate::error::Error;

                fn try_from(value: $from) -> Result<Self, $crate::error::Error> {
                    if let $from::$var(v) = value {
                        Ok(v)
                    } else {
                        Err($crate::error::Error::TypeMismatch(value.to_string(), stringify!($name).to_owned()))
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_from {
    ($($ty:ty > $name:ident :: $inner:ident),+$(,)?) => {
        $(
            impl From<$ty> for $name {
                fn from(f: $ty) -> Self {
                    Self::$inner(f)
                }
            }
        )+
    };
}

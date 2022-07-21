use std::collections::{HashMap, HashSet};

pub(crate) type Str = Box<str>;
pub(crate) type List<T> = Box<[T]>;
pub(crate) type OderedMap<K, T> = HashMap<K, T>;
pub(crate) type HashedList<T> = HashSet<T>;
//pub(crate) type OrderedDict<T> = OderedMap<Str, T>;

macro_rules! flatten {
    ($($name:ident),*) => {
        $(
        mod $name;
        pub use $name::*;
        )*
    };
}
/*
macro_rules! xsd_decl {
    ($(#[$m:meta])* $name:ident:$def:path, $($field:ident:$ty:ty),*) => {
        $(#[$m])*
        #[derive(Debug)]
        pub enum $name {
            Global($def),
            Internal{
                decl:xsdrs::Declaration<$def>,
                $($field:$ty),*
            }
        }
    };
}*/
#[derive(Debug)]
pub struct ParseError(pub Str);
pub(crate) use flatten;

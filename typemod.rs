use crate::{Ptr, Word};

#[repr(u64)]
#[derive(Debug, PartialEq)]
pub enum Type {
    Double = 0,
    Int,
    Ptr
}

impl From::<f64> for Type {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            Type::Int
        } else {
            Type::Double
        }
    }
}

impl From::<Word> for Type {
    fn from(_: Word) -> Self {
        Type::Int
    }
}

impl From::<Ptr> for Type {
    fn from(_: Ptr) -> Self {
        Type::Ptr
    }
}

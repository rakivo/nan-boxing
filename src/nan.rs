// Stolen from: <https://github.com/rakivo/nan-boxing>

const EXP_MASK: u64 = ((1 << 11) - 1) << 52;
const TYPE_MASK: u64 = ((1 << 4) - 1) << 48;
const VALUE_MASK: u64 = (1 << 48) - 1;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Type {
    F64,
    I64,
    U64,
    Ptr,
}

#[derive(Copy)]
pub struct NaNBox(pub f64);

impl std::clone::Clone for NaNBox {
    #[inline]
    fn clone(&self) -> Self {
        NaNBox(self.0)
    }
}

impl std::fmt::Display for NaNBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_type() {
            Ok(Type::F64) => write!(f, "{:?}", self.as_f64()),
            Ok(Type::I64) => write!(f, "{}", self.as_i64()),
            Ok(Type::U64) => write!(f, "{}", self.as_u64()),
            Ok(Type::Ptr) => write!(f, "{:#?}", self.as_ptr()),
            Err(bit)      => write!(f, "Failed to get type of: {f}, type bit is: {bit}", f = self.0)
        }
    }
}

impl std::fmt::Debug for NaNBox {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl NaNBox {
    #[inline(always)]
    fn mk_inf() -> f64 {
        f64::from_bits(EXP_MASK)
    }

    #[inline(always)]
    pub fn set_type(x: f64, nanbox_type: Type) -> f64 {
        let type_val = match nanbox_type {
            Type::F64 => 1,
            Type::I64 => 2,
            Type::U64 => 3,
            Type::Ptr => 4
        };
        let mut bits = x.to_bits();
        bits = (bits & !TYPE_MASK) | (((TYPE_MASK >> 48) & type_val) << 48);
        f64::from_bits(bits)
    }

    #[inline(always)]
    pub fn get_type(&self) -> Result::<Type, u64> {
        if self.is_f64() { return Ok(Type::F64) }

        let bits = self.0.to_bits();
        match (bits & TYPE_MASK) >> 48 {
            1 => Ok(Type::F64),
            2 => Ok(Type::I64),
            3 => Ok(Type::U64),
            4 => Ok(Type::Ptr),
            b @ _ => Err(b)
        }
    }

    #[inline(always)]
    pub fn set_value(x: f64, value: i64) -> f64 {
        let mut bits = x.to_bits();
        let sign_bit = (value.is_negative() as u64) << 63;
        bits = (bits & !VALUE_MASK) | ((value.abs() as u64) & VALUE_MASK) | sign_bit;
        f64::from_bits(bits)
    }

    #[inline(always)]
    pub fn get_value(&self) -> i64 {
        let bits = self.0.to_bits();
        let value = (bits & VALUE_MASK) as i64;
        if (bits & (1 << 63)) != 0 {
            -value
        } else {
            value
        }
    }

    #[inline(always)]
    pub fn is_f64(&self) -> bool {
        !self.0.is_nan()
    }

    #[inline(always)]
    pub fn is_i64(&self) -> bool {
        self.0.is_nan() && self.get_type() == Ok(Type::I64)
    }

    #[inline(always)]
    pub fn is_u64(&self) -> bool {
        self.0.is_nan() && self.get_type() == Ok(Type::U64)
    }

    #[inline(always)]
    pub fn is_ptr(&self) -> bool {
        self.0.is_nan() && self.get_type() == Ok(Type::Ptr)
    }

    #[inline(always)]
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub fn as_i64(&self) -> i64 {
        self.get_value()
    }

    #[inline(always)]
    pub fn as_u64(&self) -> u64 {
        self.get_value() as _
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut u8 {
        self.get_value() as _
    }

    #[inline(always)]
    pub fn from_f64(value: f64) -> NaNBox {
        NaNBox(value)
    }

    #[inline(always)]
    pub fn from_u64(value: u64) -> NaNBox {
        NaNBox(Self::set_type(Self::set_value(Self::mk_inf(), value as _), Type::U64))
    }

    #[inline(always)]
    pub fn from_i64(value: i64) -> NaNBox {
        NaNBox(Self::set_type(Self::set_value(Self::mk_inf(), value), Type::I64))
    }

    #[inline(always)]
    pub fn from_ptr(value: *mut u8) -> NaNBox {
        NaNBox(Self::set_type(Self::set_value(Self::mk_inf(), value as _), Type::Ptr))
    }

    #[inline(always)]
    pub fn get_f64(&self) -> Option::<f64> {
        if self.is_f64() {
            Some(self.as_f64())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_i64(&self) -> Option::<i64> {
        if self.is_i64() {
            Some(self.as_i64())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_u64(&self) -> Option::<u64> {
        if self.is_u64() {
            Some(self.as_u64())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_ptr(&self) -> Option::<*const u8> {
        if self.is_ptr() {
            Some(self.as_ptr())
        } else {
            None
        }
    }
}

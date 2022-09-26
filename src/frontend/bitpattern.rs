
/// Signifies that a signed primitive type can be constructed
/// from a hex/oct/bin representation for its unsigned counterpart.
///
/// # Example
/// i16::from_hex_pattern("0x8000") == i16::MIN
/// which does not work with from_str_radix
pub trait FromBitPattern : Sized {
    fn from_hex_pattern(s: &str) -> Option<Self>;
    fn from_oct_pattern(s: &str) -> Option<Self>;
    fn from_bin_pattern(s: &str) -> Option<Self>;
}

macro_rules! impl_from_bit_pattern {
    ($dst:ty, $raw:ty) => {
        impl FromBitPattern for $dst {
            fn from_hex_pattern(s: &str) -> Option<Self> {
                if let Ok(number) = <$raw>::from_str_radix(s, 16) {
                    Some(number as $dst)
                } else {
                    None
                }
            }
            
            fn from_oct_pattern(s: &str) -> Option<Self> {
                if let Ok(number) = <$raw>::from_str_radix(s, 8) {
                    Some(number as $dst)
                } else {
                    None
                }
            }
            
            fn from_bin_pattern(s: &str) -> Option<Self> {
                if let Ok(number) = <$raw>::from_str_radix(s, 2) {
                    Some(number as $dst)
                } else {
                    None
                }
            }
        }
    }
}

impl_from_bit_pattern!(i8, u8);
impl_from_bit_pattern!(u8, u8);
impl_from_bit_pattern!(i16, u16);
impl_from_bit_pattern!(u16, u16);
impl_from_bit_pattern!(i32, u32);
impl_from_bit_pattern!(u32, u32);
impl_from_bit_pattern!(i64, u64);
impl_from_bit_pattern!(u64, u64);


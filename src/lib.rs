//! Implementation of FourCC struct.

use std::cmp::Ordering;
use std::hash::Hash;

/// Basic FourCC byte array alias.
pub type TypeId = [u8;4];

/// Four character code (FourCC) struct.
///
/// [FourCC] is a method of encoding 32-bit unsigned integer values with human
/// readable semantics. `FourCC` can be used directly as a 32-bit index in
/// `HashMap`.
///
/// [FourCC]: https://en.wikipedia.org/wiki/FourCC
#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy, Default)]
pub struct FourCC(pub TypeId);

//------------------------------------------------------------------------------

/// Creates a new `FourCC` instance from a four character string.
///
/// # Examples
/// ```
/// use fourcc::FourCC;
///
/// let rgba = FourCC::from("RGBA");
/// assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
/// ```
impl From<&str> for FourCC {
    fn from(s: &str) -> Self
        { FourCC(s.as_bytes()[..4].try_into().unwrap()) }
}

/// Creates a new `FourCC` instance from a four character byte sequence.
impl From<&TypeId> for FourCC {
    fn from(bytes: &TypeId) -> Self
        { Self(*bytes) }
}

/// Creates a new `FourCC` instance from a 32-bit unsigned integer.
impl From<u32> for FourCC {
    fn from(num: u32) -> Self
        { Self(u32::to_be_bytes(num)) }
}

//------------------------------------------------------------------------------

impl From<FourCC> for TypeId {
    fn from(fourcc: FourCC) -> TypeId
        { fourcc.0 }
}

impl From<FourCC> for u32 {
    fn from(fourcc: FourCC) -> u32
        { u32::from_be_bytes(fourcc.0) }
}

//------------------------------------------------------------------------------

impl PartialEq<&TypeId> for FourCC {
    fn eq(&self, other: &&TypeId) -> bool
        { self.0.as_ref() == *other }
}

impl PartialEq<&str> for FourCC {
    fn eq(&self, other: &&str) -> bool
        { self == &FourCC::from(*other) }
}

impl PartialEq<u32> for FourCC {
    fn eq(&self, other: &u32) -> bool
        { self == &FourCC::from(*other) }
}

impl PartialOrd<&TypeId> for FourCC {
    fn partial_cmp(&self, other: &&TypeId) -> Option<Ordering>
        { self.0.partial_cmp(*other) }
}

impl PartialOrd<&str> for FourCC {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering>
        { self.partial_cmp(&FourCC::from(*other)) }
}

impl PartialOrd<u32> for FourCC {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering>
        { self.partial_cmp(&FourCC::from(*other)) }
}

//------------------------------------------------------------------------------

impl FourCC {
    /// Checks whether the `FourCC` value is a valid four character code.
    pub fn is_valid(&self) -> bool
        { self.0.iter().all(|&b| b.is_ascii_graphic()) }
}

// Format FourCC into human readable string.
impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "{}{}{}{}", self.0[0] as char, self.0[1] as char, self.0[2] as char, self.0[3] as char) }
}

// Format FourCC into quoted human readable string.
impl std::fmt::Debug for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "'{}{}{}{}'", self.0[0] as char, self.0[1] as char, self.0[2] as char, self.0[3] as char) }
}

//==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes() {
        let rgba = FourCC::from(b"RGBA");
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());

        let argb = FourCC(*b"ARGB");
        assert_eq!(argb.0, 1095911234_u32.to_be_bytes());
    }

    #[test]
    fn from_string() {
        let rgba = FourCC::from("RGBA");
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    }

    #[test]
    #[should_panic(expected = "range end index 4 out of range for slice of length 0")]
    fn from_string_panic() {
        let invalid = FourCC::from("");
        assert!(!invalid.is_valid());
    }

    #[test]
    fn from_u32() {
        let rgba = FourCC::from(1380401729);
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    }

    #[test]
    fn into_bytes() {
        let rgba = FourCC::from("RGBA");
        assert_eq!(<TypeId>::from(rgba), 1380401729_u32.to_be_bytes());
    }

    #[test]
    fn to_string() {
        let rgba = FourCC::from("RGBA");
        assert_eq!(rgba.to_string(), "RGBA");
    }

    #[test]
    fn into_u32() {
        let rgba = FourCC::from("RGBA");
        assert_eq!(u32::from(rgba), 1380401729);
    }

    #[test]
    fn hash_map_key() {
        use std::collections::HashMap;
        let mut map: HashMap<FourCC, &str> = HashMap::new();
        let rgba = FourCC::from("RGBA");
        map.insert(rgba, "RGBA colour format");
        assert_eq!(map.get(&rgba), Some(&"RGBA colour format"));
    }

    #[test]
    fn equality() {
        let rgba = FourCC::from("RGBA");
        let argb = FourCC::from("ARGB");
        assert_eq!(rgba, rgba);
        assert_eq!(rgba, 1380401729_u32);
        assert_eq!(rgba, b"RGBA");
        assert_eq!(rgba, "RGBA");
        assert_ne!(rgba, argb);
        assert!(rgba > argb);
        assert!(rgba > 1095911234_u32);
        assert!(rgba > b"ARGB");
        assert!(rgba > "ARGB");
        assert!(rgba >= argb);
        assert!(argb < rgba);
        assert!(argb <= rgba);
    }

    #[test]
    fn validate() {
        let rgba = FourCC::from("RGBA");
        assert!(rgba.is_valid());

        let invalid = FourCC::from("\0\x01\x02\x03");
        assert!(!invalid.is_valid());
    }

    #[test]
    fn fmt() {
        let rgba = FourCC::from("RGBA");
        let output = format!("{}", rgba);
        assert_eq!(output, "RGBA");
    }
}

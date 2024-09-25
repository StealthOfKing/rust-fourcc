//! Implementation of FourCC struct.

use std::cmp::Ordering;
use std::hash::Hash;

/// Four character code (FourCC) struct.
///
/// [FourCC] is a method of encoding 32-bit unsigned integer values with human
/// readable semantics. `FourCC` can be used directly as a 32-bit index in
/// `HashMap`.
///
/// [FourCC]: https://en.wikipedia.org/wiki/FourCC
#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct FourCC(pub [u8; 4]);

impl FourCC {
    /// Creates a new `FourCC` instance from a four character byte sequence.
    pub fn from_bytes(bytes: &[u8; 4]) -> FourCC {
        FourCC(*bytes)
    }

    /// Creates a new `FourCC` instance from a four character string.
    ///
    /// # Examples
    /// ```
    /// use fourcc::FourCC;
    ///
    /// let rgba = FourCC::from_string("RGBA");
    /// assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    /// ```
    pub fn from_string(s: &str) -> FourCC {
        FourCC([s.as_bytes()[0], s.as_bytes()[1], s.as_bytes()[2], s.as_bytes()[3]])
    }

    /// Creates a new `FourCC` instance from a 32-bit unsigned integer.
    pub fn from_u32(num: &u32) -> FourCC {
        FourCC(u32::to_be_bytes(*num))
    }

    //--------------------------------------------------------------------------

    /// Checks whether the `FourCC` value is a valid four character code.
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|&b| b.is_ascii_graphic())
    }
}

//------------------------------------------------------------------------------

impl PartialEq<&[u8; 4]> for FourCC {
    fn eq(&self, other: &&[u8; 4]) -> bool {
        self.0.as_ref() == *other
    }
}

impl PartialEq<&str> for FourCC {
    fn eq(&self, other: &&str) -> bool {
        self == &FourCC::from_string(*other)
    }
}

impl PartialEq<u32> for FourCC {
    fn eq(&self, other: &u32) -> bool {
        self == &FourCC::from_u32(other)
    }
}

impl PartialOrd<&[u8; 4]> for FourCC {
    fn partial_cmp(&self, other: &&[u8; 4]) -> Option<Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl PartialOrd<&str> for FourCC {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(&FourCC::from_string(other))
    }
}

impl PartialOrd<u32> for FourCC {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.partial_cmp(&FourCC::from_u32(other))
    }
}

//------------------------------------------------------------------------------

// Format FourCC into human readable string.
impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.0[0] as char, self.0[1] as char, self.0[2] as char, self.0[3] as char)
}
}

//==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes() {
        let rgba = FourCC::from_bytes(b"RGBA");
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    }

    #[test]
    fn from_string() {
        let rgba = FourCC::from_string("RGBA");
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn from_string_panic() {
        let invalid = FourCC::from_string("");
        assert!(!invalid.is_valid());
    }

    #[test]
    fn from_u32() {
        let rgba = FourCC::from_u32(&1380401729);
        assert_eq!(rgba.0, 1380401729_u32.to_be_bytes());
    }

    #[test]
    fn hash_map_key() {
        use std::collections::HashMap;
        let mut map: HashMap<FourCC, &str> = HashMap::new();
        let rgba = FourCC::from_string("RGBA");
        map.insert(rgba, "RGBA colour format");
        assert_eq!(map.get(&rgba), Some(&"RGBA colour format"));
    }

    #[test]
    fn equality() {
        let rgba = FourCC::from_string("RGBA");
        let argb = FourCC::from_string("ARGB");
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
        let rgba = FourCC::from_string("RGBA");
        assert!(rgba.is_valid());

        let invalid = FourCC::from_string("\0\x01\x02\x03");
        assert!(!invalid.is_valid());
    }

    #[test]
    fn fmt() {
        let rgba = FourCC::from_string("RGBA");
        let output = format!("{}", rgba);
        assert_eq!(output, "RGBA");
    }
}

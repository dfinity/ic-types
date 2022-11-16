use std::fmt;
use crate::utils::byte_slice_fmt::truncate_and_format;

/// A binary "blob", i.e. a byte array.
///
/// Use `serde_bytes` so that the `Vec<u8>` is deserialized as a sequence
/// (array) of bytes, whereas we want an actual CBOR "byte array", e.g. a
/// byte string.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] pub Vec<u8>);

impl Blob {
    fn format(&self, f: &mut fmt::Formatter<'_>, max_bytes_to_format: usize) -> fmt::Result {
        f.write_fmt(format_args!(
            "Blob{{{}}}",
            truncate_and_format(self.0.as_slice(), max_bytes_to_format)
        ))
    }
}

impl fmt::Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f, usize::MAX)
    }
}

impl fmt::Display for Blob {
    // Just like Debug, except we truncate long ones
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f, 40_usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_to_cbor() {
        let bytes = vec![1, 2, 3, 4, 5, 6];
        let blob = Blob(bytes);

        let cbor_bytes = serde_cbor::to_vec(&blob).expect("Failed to encode blob to cbor");
        let cbor_hex = hex::encode(cbor_bytes);

        assert_eq!(cbor_hex, String::from("46010203040506"));
    }

    #[test]
    fn format() {
        let bytes = vec![1, 2, 3, 4, 5, 6];
        let blob = Blob(bytes);

        let formatted_blob = format!("{:?}", blob);

        assert_eq!(formatted_blob, "Blob{6 bytes;010203040506}");
    }

    #[test]
    fn format_large_blob() {
        let bytes = vec![1; 42];
        let blob = Blob(bytes);

        let formatted_blob = format!("{}", blob);

        assert_eq!(formatted_blob, "Blob{42 bytes;01010101010101010101010101010101010101010101010101010101010101010101010101010101…}");
    }

    #[test]
    fn format_large_blob_debug() {
        let bytes = vec![1; 42];
        let blob = Blob(bytes);

        let formatted_blob = format!("{:?}", blob);

        assert_eq!(formatted_blob, "Blob{42 bytes;010101010101010101010101010101010101010101010101010101010101010101010101010101010101}");
    }
}

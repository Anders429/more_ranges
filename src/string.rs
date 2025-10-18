const INVALID: &str = "\u{fffd}\u{fffd}\u{fffd}";

/// Converts from UTF-8 bytes to `&str`. If the bytes are not valid, instead returns a sequence of
/// three repeated replacement characters (U+FFFD, or �) to indicate a decoding error.
pub(crate) fn from_utf8_lossy(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap_or(INVALID)
}

#[cfg(test)]
mod tests {
    use super::{from_utf8_lossy, INVALID};

    #[test]
    fn empty() {
        assert_eq!(from_utf8_lossy(b""), "");
    }

    #[test]
    fn valid() {
        assert_eq!(from_utf8_lossy(b"abcd"), "abcd");
    }

    #[test]
    fn invalid() {
        assert_eq!(from_utf8_lossy(b"\xff"), INVALID);
    }
}

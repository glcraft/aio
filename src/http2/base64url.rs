
/// Encode a byte slice into a base64url string.
pub fn encode(data: &[u8]) -> String {
    const BASE64URL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut encoded = String::new();
    let mut buffer = 0;
    let mut bits_left = 0;
    for &byte in data {
        buffer <<= 8;
        buffer |= byte as u32;
        bits_left += 8;
        while bits_left >= 6 {
            bits_left -= 6;
            encoded.push(BASE64URL[((buffer >> bits_left) & 0x3F) as usize] as char);
        }
    }
    if bits_left > 0 {
        buffer <<= 6 - bits_left;
        encoded.push(BASE64URL[(buffer & 0x3F) as usize] as char);
        // base64url doesn't require padding
    }
    encoded
}

/// Decode a base64url string into a byte vector.
pub fn decode(encoded: &str) -> Result<Vec<u8>, ()> {
    let mut decoded = Vec::new();
    let mut buffer = 0;
    let mut bits_left = 0;
    for &byte in encoded.as_bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err(()),
        };
        buffer <<= 6;
        buffer |= value as u32;
        bits_left += 6;
        if bits_left >= 8 {
            bits_left -= 8;
            decoded.push((buffer >> bits_left) as u8);
        }
    }
    if bits_left > 0 {
        buffer <<= 8 - bits_left;
        decoded.push((buffer & 0xFF) as u8);
    }
    Ok(decoded)
}
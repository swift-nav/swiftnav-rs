fn u8_to_nibbles(byte: u8) -> (u8, u8) {
    // The high nibble is obtained by shifting the byte 4 bits to the right.
    // This discards the lower 4 bits and moves the upper 4 bits into the lower 4 bit positions.
    let high_nibble = byte >> 4;

    // The low nibble is obtained by masking the byte with 0x0F (binary 0000_1111).
    // This keeps only the lower 4 bits and sets the upper 4 bits to zero.
    let low_nibble = byte & 0x0F;

    (high_nibble, low_nibble)
}

/// Convert a nibble (4 bits) to its ASCII character representation
fn u8_to_ascii_char(nibble: u8) -> char {
    if nibble <= 0x9 {
        (nibble + b'0') as char
    } else {
        (nibble - 10 + b'A') as char
    }
}

// Calculate the NMEA checksum for a given sentence
// https://forum.arduino.cc/t/nmea-checksums-explained/1046083
#[must_use]
pub fn calculate_checksum(sentence: &str) -> String {
    let mut checksum = 0;

    for (i, byte) in sentence.bytes().enumerate() {
        // Skip the starting '$' and the '*' before the checksum
        if (i == 0 && byte == b'$') || byte == b'*' {
            continue;
        }

        checksum ^= byte;
    }

    let (nibble1, nibble2) = u8_to_nibbles(checksum);

    let char1 = u8_to_ascii_char(nibble1);
    let char2 = u8_to_ascii_char(nibble2);

    format!("{char1}{char2}")
}

// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

fn u8_to_nibbles(byte: u8) -> (u8, u8) {
    // The high nibble is obtained by shifting the byte 4 bits to the right.
    // This discards the lower 4 bits and moves the upper 4 bits into the lower 4 bit positions.
    let high_nibble = (byte >> 4) & 0x0F;

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
        if i == 0 && byte == b'$' {
            continue;
        }

        if byte == b'*' {
            break;
        }

        checksum ^= byte;
    }

    let (nibble1, nibble2) = u8_to_nibbles(checksum);

    let char1 = u8_to_ascii_char(nibble1);
    let char2 = u8_to_ascii_char(nibble2);

    format!("{char1}{char2}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_checksum() {
        let sentence = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "47");

        let sentence = "$GPGLL,5057.970,N,00146.110,E,142451,A";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "27");

        let sentence = "$GPVTG,089.0,T,,,15.2,N,,";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "7F");
    }

    #[test]
    fn calculate_checksum_ignores_dollar_and_asterisk_tails() {
        // NOTE(ted): All of these examples should produce the same checksum

        // check with '$'
        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "37");

        //check with '$' and '*'
        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "37");

        //check with '$' and '*' and fake checksum
        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*00";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "37");

        //check '*'
        let sentence = "GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "37");

        //check '*' and fake checksum
        let sentence = "GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*00";
        let checksum = super::calculate_checksum(sentence);
        assert_eq!(checksum, "37");
    }
}

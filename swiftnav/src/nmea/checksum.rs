// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

/// Calculate the NMEA checksum for a given sentence as a raw byte.
///
/// XORs all bytes between `$` and `*` (exclusive). A leading `$` and
/// anything from `*` onward are ignored.
///
/// <https://forum.arduino.cc/t/nmea-checksums-explained/1046083>
#[must_use]
pub fn calculate_checksum(sentence: &str) -> u8 {
    let mut cs: u8 = 0;

    for (i, byte) in sentence.bytes().enumerate() {
        if i == 0 && byte == b'$' {
            continue;
        }
        if byte == b'*' {
            break;
        }
        cs ^= byte;
    }

    cs
}

#[cfg(test)]
mod tests {
    use super::calculate_checksum;

    #[test]
    fn test_calculate_checksum() {
        let sentence = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
        assert_eq!(calculate_checksum(sentence), 0x47);

        let sentence = "$GPGLL,5057.970,N,00146.110,E,142451,A";
        assert_eq!(calculate_checksum(sentence), 0x27);

        let sentence = "$GPVTG,089.0,T,,,15.2,N,,";
        assert_eq!(calculate_checksum(sentence), 0x7F);
    }

    #[test]
    fn calculate_checksum_ignores_dollar_and_asterisk_tails() {
        // All of these examples should produce the same checksum
        let expected = 0x37;

        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42";
        assert_eq!(calculate_checksum(sentence), expected);

        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*";
        assert_eq!(calculate_checksum(sentence), expected);

        let sentence = "$GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*00";
        assert_eq!(calculate_checksum(sentence), expected);

        let sentence = "GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*";
        assert_eq!(calculate_checksum(sentence), expected);

        let sentence = "GPGGA,0189.00,34.052200,N,-118.243700,W,2,8,1.2,0.0,M,1.00,2,42*00";
        assert_eq!(calculate_checksum(sentence), expected);
    }
}

// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Error detection code

/// Calculate Qualcomm 24-bit Cyclical Redundancy Check (CRC-24Q).
///
/// This CRC is used with the RTCM protocol
///
/// The CRC polynomial used is:
///   x^{24} + x^{23} + x^{18} + x^{17} + x^{14} + x^{11} + x^{10} +
///   x^7    + x^6    + x^5    + x^4    + x^3    + x+1
///
/// Mask 0x1864CFB, not reversed, not XOR'd
pub fn compute_crc24q(buf: &[u8], initial_value: u32) -> u32 {
    unsafe { swiftnav_sys::crc24q(buf.as_ptr(), buf.len() as u32, initial_value) }
}

#[cfg(test)]
mod tests {
    const TEST_DATA: &[u8] = "123456789".as_bytes();

    #[test]
    fn crc24q() {
        let crc = super::compute_crc24q(&TEST_DATA[0..0], 0);
        assert!(
            crc == 0,
            "CRC of empty buffer with starting value 0 should be 0, not {}",
            crc
        );

        let crc = super::compute_crc24q(&TEST_DATA[0..0], 22);
        assert!(
            crc == 22,
            "CRC of empty buffer with starting value 22 should be 22, not {}",
            crc
        );

        /* Test value taken from python crcmod package tests, see:
         * http://crcmod.sourceforge.net/crcmod.predefined.html */
        let crc = super::compute_crc24q(TEST_DATA, 0xB704CE);
        assert!(
            crc == 0x21CF02,
            "CRC of \"123456789\" with init value 0xB704CE should be {}, not 0x%06X",
            crc
        );
    }
}

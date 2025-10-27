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
//!
//! The checksum used by the RTCM protocol is CRC-24Q. This module provides a
//! function for calculating that checksum value over a set of data.
//!
//! # Examples
//!
//! To generate a CRC value in one shot you can simply give the [`compute_crc24q`]
//! function all of the data as a slice of bytes and the initial value:
//!
//! ```
//! # use swiftnav::edc::compute_crc24q;
//! let msg_data = vec![0xD3, 0x00, 0x13, 0x3E, 0xD7, 0xD3, 0x02, 0x02, 0x98, 0x0E, 0xDE, 0xEF, 0x34, 0xB4, 0xBD, 0x62, 0xAC, 0x09, 0x41, 0x98, 0x6F, 0x33];
//! let init = 0;
//!
//! let crc = compute_crc24q(&msg_data, init);
//! assert_eq!(crc, 0x00360B98);
//! ```
//!
//! If the data is split up you can use the CRC from a previous invokation as the initial
//! value for a subsequent invokation:
//!
//! ```
//! # use swiftnav::edc::compute_crc24q;
//! let block1 = vec![0xD3, 0x00, 0x13, 0x3E, 0xD7, 0xD3, 0x02, 0x02, 0x98, 0x0E];
//! let block2 = vec![0xDE, 0xEF, 0x34, 0xB4, 0xBD, 0x62, 0xAC, 0x09, 0x41, 0x98, 0x6F, 0x33];
//! let init = 0;
//!
//! let intermediate = compute_crc24q(&block1, init);
//! let crc = compute_crc24q(&block2, intermediate);
//! assert_eq!(crc, 0x00360B98);
//! ```

const CRC24Q_TABLE: [u32; 256] = [
    0x0000_0000,
    0x0086_4CFB,
    0x008A_D50D,
    0x000C_99F6,
    0x0093_E6E1,
    0x0015_AA1A,
    0x0019_33EC,
    0x009F_7F17,
    0x00A1_8139,
    0x0027_CDC2,
    0x002B_5434,
    0x00AD_18CF,
    0x0032_67D8,
    0x00B4_2B23,
    0x00B8_B2D5,
    0x003E_FE2E,
    0x00C5_4E89,
    0x0043_0272,
    0x004F_9B84,
    0x00C9_D77F,
    0x0056_A868,
    0x00D0_E493,
    0x00DC_7D65,
    0x005A_319E,
    0x0064_CFB0,
    0x00E2_834B,
    0x00EE_1ABD,
    0x0068_5646,
    0x00F7_2951,
    0x0071_65AA,
    0x007D_FC5C,
    0x00FB_B0A7,
    0x000C_D1E9,
    0x008A_9D12,
    0x0086_04E4,
    0x0000_481F,
    0x009F_3708,
    0x0019_7BF3,
    0x0015_E205,
    0x0093_AEFE,
    0x00AD_50D0,
    0x002B_1C2B,
    0x0027_85DD,
    0x00A1_C926,
    0x003E_B631,
    0x00B8_FACA,
    0x00B4_633C,
    0x0032_2FC7,
    0x00C9_9F60,
    0x004F_D39B,
    0x0043_4A6D,
    0x00C5_0696,
    0x005A_7981,
    0x00DC_357A,
    0x00D0_AC8C,
    0x0056_E077,
    0x0068_1E59,
    0x00EE_52A2,
    0x00E2_CB54,
    0x0064_87AF,
    0x00FB_F8B8,
    0x007D_B443,
    0x0071_2DB5,
    0x00F7_614E,
    0x0019_A3D2,
    0x009F_EF29,
    0x0093_76DF,
    0x0015_3A24,
    0x008A_4533,
    0x000C_09C8,
    0x0000_903E,
    0x0086_DCC5,
    0x00B8_22EB,
    0x003E_6E10,
    0x0032_F7E6,
    0x00B4_BB1D,
    0x002B_C40A,
    0x00AD_88F1,
    0x00A1_1107,
    0x0027_5DFC,
    0x00DC_ED5B,
    0x005A_A1A0,
    0x0056_3856,
    0x00D0_74AD,
    0x004F_0BBA,
    0x00C9_4741,
    0x00C5_DEB7,
    0x0043_924C,
    0x007D_6C62,
    0x00FB_2099,
    0x00F7_B96F,
    0x0071_F594,
    0x00EE_8A83,
    0x0068_C678,
    0x0064_5F8E,
    0x00E2_1375,
    0x0015_723B,
    0x0093_3EC0,
    0x009F_A736,
    0x0019_EBCD,
    0x0086_94DA,
    0x0000_D821,
    0x000C_41D7,
    0x008A_0D2C,
    0x00B4_F302,
    0x0032_BFF9,
    0x003E_260F,
    0x00B8_6AF4,
    0x0027_15E3,
    0x00A1_5918,
    0x00AD_C0EE,
    0x002B_8C15,
    0x00D0_3CB2,
    0x0056_7049,
    0x005A_E9BF,
    0x00DC_A544,
    0x0043_DA53,
    0x00C5_96A8,
    0x00C9_0F5E,
    0x004F_43A5,
    0x0071_BD8B,
    0x00F7_F170,
    0x00FB_6886,
    0x007D_247D,
    0x00E2_5B6A,
    0x0064_1791,
    0x0068_8E67,
    0x00EE_C29C,
    0x0033_47A4,
    0x00B5_0B5F,
    0x00B9_92A9,
    0x003F_DE52,
    0x00A0_A145,
    0x0026_EDBE,
    0x002A_7448,
    0x00AC_38B3,
    0x0092_C69D,
    0x0014_8A66,
    0x0018_1390,
    0x009E_5F6B,
    0x0001_207C,
    0x0087_6C87,
    0x008B_F571,
    0x000D_B98A,
    0x00F6_092D,
    0x0070_45D6,
    0x007C_DC20,
    0x00FA_90DB,
    0x0065_EFCC,
    0x00E3_A337,
    0x00EF_3AC1,
    0x0069_763A,
    0x0057_8814,
    0x00D1_C4EF,
    0x00DD_5D19,
    0x005B_11E2,
    0x00C4_6EF5,
    0x0042_220E,
    0x004E_BBF8,
    0x00C8_F703,
    0x003F_964D,
    0x00B9_DAB6,
    0x00B5_4340,
    0x0033_0FBB,
    0x00AC_70AC,
    0x002A_3C57,
    0x0026_A5A1,
    0x00A0_E95A,
    0x009E_1774,
    0x0018_5B8F,
    0x0014_C279,
    0x0092_8E82,
    0x000D_F195,
    0x008B_BD6E,
    0x0087_2498,
    0x0001_6863,
    0x00FA_D8C4,
    0x007C_943F,
    0x0070_0DC9,
    0x00F6_4132,
    0x0069_3E25,
    0x00EF_72DE,
    0x00E3_EB28,
    0x0065_A7D3,
    0x005B_59FD,
    0x00DD_1506,
    0x00D1_8CF0,
    0x0057_C00B,
    0x00C8_BF1C,
    0x004E_F3E7,
    0x0042_6A11,
    0x00C4_26EA,
    0x002A_E476,
    0x00AC_A88D,
    0x00A0_317B,
    0x0026_7D80,
    0x00B9_0297,
    0x003F_4E6C,
    0x0033_D79A,
    0x00B5_9B61,
    0x008B_654F,
    0x000D_29B4,
    0x0001_B042,
    0x0087_FCB9,
    0x0018_83AE,
    0x009E_CF55,
    0x0092_56A3,
    0x0014_1A58,
    0x00EF_AAFF,
    0x0069_E604,
    0x0065_7FF2,
    0x00E3_3309,
    0x007C_4C1E,
    0x00FA_00E5,
    0x00F6_9913,
    0x0070_D5E8,
    0x004E_2BC6,
    0x00C8_673D,
    0x00C4_FECB,
    0x0042_B230,
    0x00DD_CD27,
    0x005B_81DC,
    0x0057_182A,
    0x00D1_54D1,
    0x0026_359F,
    0x00A0_7964,
    0x00AC_E092,
    0x002A_AC69,
    0x00B5_D37E,
    0x0033_9F85,
    0x003F_0673,
    0x00B9_4A88,
    0x0087_B4A6,
    0x0001_F85D,
    0x000D_61AB,
    0x008B_2D50,
    0x0014_5247,
    0x0092_1EBC,
    0x009E_874A,
    0x0018_CBB1,
    0x00E3_7B16,
    0x0065_37ED,
    0x0069_AE1B,
    0x00EF_E2E0,
    0x0070_9DF7,
    0x00F6_D10C,
    0x00FA_48FA,
    0x007C_0401,
    0x0042_FA2F,
    0x00C4_B6D4,
    0x00C8_2F22,
    0x004E_63D9,
    0x00D1_1CCE,
    0x0057_5035,
    0x005B_C9C3,
    0x00DD_8538,
];

/// Calculate Qualcomm 24-bit Cyclical Redundancy Check (CRC-24Q).
///
/// This CRC is used with the RTCM protocol
///
/// The CRC polynomial used is:
/// $$[
///   x^{24} + x^{23} + x^{18} + x^{17} + x^{14} + x^{11} + x^{10} +
///   x^7    + x^6    + x^5    + x^4    + x^3    + x+1
/// ]$$
///
/// Mask 0x1864CFB, not reversed, not XOR'd
///
/// # Notes
///
/// Only the lower 24 bits of the initial value are used!
#[must_use]
pub fn compute_crc24q(buf: &[u8], initial_value: u32) -> u32 {
    let mut crc = initial_value & 0x00FF_FFFF;
    for &byte in buf {
        let index = ((crc >> 16) ^ u32::from(byte)) as usize & 0xFF;
        crc = ((crc << 8) & 0x00FF_FFFF) ^ CRC24Q_TABLE[index];
    }
    crc
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const TEST_DATA: &[u8] = "123456789".as_bytes();

    /// Helper function to append a CRC-24Q value as 3 bytes (big-endian) to a buffer
    #[expect(clippy::cast_possible_truncation)]
    fn append_crc24q(data: &mut Vec<u8>, crc: u32) {
        data.push((crc >> 16) as u8);
        data.push((crc >> 8) as u8);
        data.push(crc as u8);
    }

    /// Helper function to flip a single bit in the data at the given bit position
    fn flip_bit(data: &mut [u8], bit_position: usize) {
        if !data.is_empty() {
            let byte_index = (bit_position / 8) % data.len();
            let bit_index = bit_position % 8;
            data[byte_index] ^= 1 << bit_index;
        }
    }

    #[test]
    fn test_crc24q() {
        let crc = compute_crc24q(&TEST_DATA[0..0], 0);
        assert!(
            crc == 0,
            "CRC of empty buffer with starting value 0 should be 0, not {crc}",
        );

        let crc = compute_crc24q(&TEST_DATA[0..0], 22);
        assert!(
            crc == 22,
            "CRC of empty buffer with starting value 22 should be 22, not {crc}",
        );

        /* Test value taken from python crcmod package tests, see:
         * http://crcmod.sourceforge.net/crcmod.predefined.html */
        let crc = compute_crc24q(TEST_DATA, 0x00B7_04CE);
        assert!(
            crc == 0x0021_CF02,
            "CRC of \"123456789\" with init value 0xB704CE should be 0x21CF02, not {crc}",
        );
    }

    // Property-based tests using proptest
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        /// Property: Appending the CRC to data and recalculating should yield zero.
        /// This is the fundamental property used for error detection in protocols.
        #[test]
        fn prop_crc_append_yields_zero(data in prop::collection::vec(any::<u8>(), 0..1000)) {
            let crc = compute_crc24q(&data, 0);
            let mut data_with_crc = data.clone();
            append_crc24q(&mut data_with_crc, crc);

            let verification_crc = compute_crc24q(&data_with_crc, 0);
            prop_assert_eq!(verification_crc, 0,
                "CRC of data with appended CRC should be 0, got 0x{:06X} for data length {}",
                verification_crc, data.len());
        }

        /// Property: CRC calculation is deterministic - same input always produces same output.
        #[test]
        fn prop_crc_is_deterministic(data in prop::collection::vec(any::<u8>(), 0..1000), init in any::<u32>()) {
            let crc1 = compute_crc24q(&data, init);
            let crc2 = compute_crc24q(&data, init);
            prop_assert_eq!(crc1, crc2, "CRC calculation should be deterministic");
        }

        /// Property: CRC result always stays within 24-bit bounds (0x000000 to 0xFFFFFF).
        #[test]
        fn prop_crc_stays_within_24_bits(data in prop::collection::vec(any::<u8>(), 0..1000), init in any::<u32>()) {
            let crc = compute_crc24q(&data, init);
            prop_assert!(crc <= 0x00FF_FFFF, "CRC result 0x{:08X} exceeds 24-bit maximum", crc);
        }

        /// Property: Incremental CRC calculation equals full calculation.
        /// CRC(data1 + data2) should equal CRC(data2, initial=CRC(data1))
        #[test]
        fn prop_crc_incremental_calculation(
            data1 in prop::collection::vec(any::<u8>(), 0..500),
            data2 in prop::collection::vec(any::<u8>(), 0..500),
            init in any::<u32>()
        ) {
            // Calculate CRC on combined data
            let mut combined_data = data1.clone();
            combined_data.extend_from_slice(&data2);
            let full_crc = compute_crc24q(&combined_data, init);

            // Calculate CRC incrementally
            let intermediate_crc = compute_crc24q(&data1, init);
            let incremental_crc = compute_crc24q(&data2, intermediate_crc);

            prop_assert_eq!(full_crc, incremental_crc,
                "Incremental CRC calculation should match full calculation");
        }

        /// Property: Initial values are properly masked to 24 bits.
        /// init and (init & 0xFFFFFF) should produce the same result.
        #[test]
        fn prop_crc_initial_value_masked(data in prop::collection::vec(any::<u8>(), 0..100), init in any::<u32>()) {
            let crc1 = compute_crc24q(&data, init);
            let crc2 = compute_crc24q(&data, init & 0x00FF_FFFF);
            prop_assert_eq!(crc1, crc2,
                "CRC with init 0x{:08X} should equal CRC with masked init 0x{:06X}",
                init, init & 0x00FF_FFFF);
        }

        /// Property: Single bit errors are detected (CRC changes).
        /// Flipping any single bit in non-empty data should change the CRC.
        #[test]
        fn prop_crc_detects_single_bit_errors(
            mut data in prop::collection::vec(any::<u8>(), 1..100),
            bit_position in any::<usize>(),
            init in any::<u32>()
        ) {
            let original_crc = compute_crc24q(&data, init);
            flip_bit(&mut data, bit_position);
            let modified_crc = compute_crc24q(&data, init);

            prop_assert_ne!(original_crc, modified_crc,
                "CRC should change when a bit is flipped (original: 0x{:06X}, modified: 0x{:06X})",
                original_crc, modified_crc);
        }

        /// Property: CRC calculation is associative when split into arbitrary chunks.
        #[test]
        fn prop_crc_associative_chunks(
            data in prop::collection::vec(any::<u8>(), 1..200),
            chunk_sizes in prop::collection::vec(1usize..50, 1..10),
            init in any::<u32>()
        ) {
            // Calculate CRC on full data
            let full_crc = compute_crc24q(&data, init);

            // Calculate CRC in chunks
            let mut current_crc = init;
            let mut pos = 0;

            for &chunk_size in &chunk_sizes {
                if pos >= data.len() {
                    break;
                }
                let end = std::cmp::min(pos + chunk_size, data.len());
                current_crc = compute_crc24q(&data[pos..end], current_crc);
                pos = end;
            }

            // Process any remaining data
            if pos < data.len() {
                current_crc = compute_crc24q(&data[pos..], current_crc);
            }

            prop_assert_eq!(full_crc, current_crc,
                "CRC calculated in chunks should match full calculation");
        }
    }
}

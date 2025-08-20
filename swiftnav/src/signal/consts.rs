// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
use crate::math::compile_time_max_u16;

/// Total number of satellites in the GPS constellation.
pub const NUM_SATS_GPS: u16 = 32;
/// Total number of satellites in the SBAS constellation.
pub const NUM_SATS_SBAS: u16 = 19;
/// Total number of satellites in the GLONASS constellation.
/// refer to <https://igscb.jpl.nasa.gov/pipermail/igsmail/2012/007771.html> and
/// <https://igscb.jpl.nasa.gov/pipermail/igsmail/2015/008391.html>
pub const NUM_SATS_GLO: u16 = 28;
/// Total number of satellites in the BeiDou constellation.
pub const NUM_SATS_BDS: u16 = 64;
/// Total number of satellites in the Galileo constellation.
pub const NUM_SATS_GAL: u16 = 36;
/// Total number of satellites in the QZSS constellation.
pub const NUM_SATS_QZS: u16 = 10;

/// Total number of satellites in all constellations
pub const NUM_SATS: u16 =
    NUM_SATS_GPS + NUM_SATS_SBAS + NUM_SATS_GLO + NUM_SATS_BDS + NUM_SATS_QZS + NUM_SATS_GAL;

/// The maximum number of satellites in a single constellation
pub const MAX_NUM_SATS: u16 = compile_time_max_u16(
    NUM_SATS_GPS,
    compile_time_max_u16(
        NUM_SATS_SBAS,
        compile_time_max_u16(
            NUM_SATS_GLO,
            compile_time_max_u16(
                NUM_SATS_BDS,
                compile_time_max_u16(NUM_SATS_QZS, NUM_SATS_GAL),
            ),
        ),
    ),
);

/// Total number of codes in the GPS constellation.
pub const NUM_CODES_GPS: u16 = 13;
/// Total number of codes in the SBAS constellation.
pub const NUM_CODES_SBAS: u16 = 5;
/// Total number of codes in the GLONASS constellation.
pub const NUM_CODES_GLO: u16 = 4;
/// Total number of codes in the BeiDou constellation.
pub const NUM_CODES_BDS: u16 = 15;
/// Total number of codes in the QZSS constellation.
pub const NUM_CODES_QZS: u16 = 11;
/// Total number of codes in the Galileo constellation.
pub const NUM_CODES_GAL: u16 = 16;

/// Total number of codes across all constellations
pub const NUM_CODES: u16 =
    NUM_CODES_GPS + NUM_CODES_SBAS + NUM_CODES_GLO + NUM_CODES_BDS + NUM_CODES_GAL + NUM_CODES_QZS;

/// Max number of GLO frequency slot, correspond to frequency slot 6
pub const GLO_MAX_FCN: i16 = 14;

/// Min number of GLO frequency slot, correspond to frequency slot -7
pub const GLO_MIN_FCN: i16 = 1;

/// Used to produce an unshifted GLO frequency slot out of GLO slots in
/// GLO_MIN_FCN .. GLO_MAX_FCN range
pub const GLO_FCN_OFFSET: i16 = 8;

/// Total number of L1C/A signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L1CA: u16 = NUM_SATS_GPS;
/// Total number of L2C signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L2C: u16 = NUM_SATS_GPS;
/// Total number of L5 signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L5: u16 = NUM_SATS_GPS;
/// Total number of L1P signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L1P: u16 = NUM_SATS_GPS;
/// Total number of L2P signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L2P: u16 = NUM_SATS_GPS;
/// Total number of L1C signals in the GPS constellation.
pub const NUM_SIGNALS_GPS_L1C: u16 = NUM_SATS_GPS;

/// Total number of L1C/A signals in the SBAS constellation.
pub const NUM_SIGNALS_SBAS_L1CA: u16 = NUM_SATS_SBAS;
/// Total number of L5 signals in the SBAS constellation.
pub const NUM_SIGNALS_SBAS_L5: u16 = NUM_SATS_SBAS;

/// Total number of L1OF signals in the GLONASS constellation.
pub const NUM_SIGNALS_GLO_L1OF: u16 = NUM_SATS_GLO;
/// Total number of L2OF signals in the GLONASS constellation.
pub const NUM_SIGNALS_GLO_L2OF: u16 = NUM_SATS_GLO;
/// Total number of L1P signals in the GLONASS constellation.
pub const NUM_SIGNALS_GLO_L1P: u16 = NUM_SATS_GLO;
/// Total number of L1P signals in the GLONASS constellation.
pub const NUM_SIGNALS_GLO_L2P: u16 = NUM_SATS_GLO;

/// Total number of B1 signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS2_B1: u16 = NUM_SATS_BDS;
/// Total number of B2 signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS2_B2: u16 = NUM_SATS_BDS;
/// Total number of B1C signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS3_B1C: u16 = NUM_SATS_BDS;
/// Total number of B5 signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS3_B5: u16 = NUM_SATS_BDS;
/// Total number of B7 signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS3_B7: u16 = NUM_SATS_BDS;
/// Total number of B3 signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS3_B3: u16 = NUM_SATS_BDS;

/// Total number of E1 signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL_E1: u16 = NUM_SATS_GAL;
/// Total number of E6 signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL_E6: u16 = NUM_SATS_GAL;
/// Total number of E7 signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL_E7: u16 = NUM_SATS_GAL;
/// Total number of E8 signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL_E8: u16 = NUM_SATS_GAL;
/// Total number of E5 signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL_E5: u16 = NUM_SATS_GAL;

/// Total number of L1 signals in the QZSS constellation.
pub const NUM_SIGNALS_QZS_L1: u16 = NUM_SATS_QZS;
/// Total number of L1C signals in the QZSS constellation.
pub const NUM_SIGNALS_QZS_L1C: u16 = NUM_SATS_QZS;
/// Total number of L2C signals in the QZSS constellation.
pub const NUM_SIGNALS_QZS_L2C: u16 = NUM_SATS_QZS;
/// Total number of L5 signals in the QZSS constellation.
pub const NUM_SIGNALS_QZS_L5: u16 = NUM_SATS_QZS;

/// Total number of signals in the GPS constellation.
pub const NUM_SIGNALS_GPS: u16 = 2 * NUM_SIGNALS_GPS_L1CA
    + 3 * NUM_SIGNALS_GPS_L2C
    + NUM_SIGNALS_GPS_L1P
    + NUM_SIGNALS_GPS_L2P
    + 3 * NUM_SIGNALS_GPS_L5
    + 3 * NUM_SIGNALS_GPS_L1C;
/// Total number of signals in the SBAS constellation.
pub const NUM_SIGNALS_SBAS: u16 = 2 * NUM_SIGNALS_SBAS_L1CA + 3 * NUM_SIGNALS_SBAS_L5;
/// Total number of signals in the GLONASS constellation.
pub const NUM_SIGNALS_GLO: u16 =
    NUM_SIGNALS_GLO_L1OF + NUM_SIGNALS_GLO_L2OF + NUM_SIGNALS_GLO_L1P + NUM_SIGNALS_GLO_L2P;
/// Total number of signals in the BeiDou constellation.
pub const NUM_SIGNALS_BDS: u16 = 2 * NUM_SIGNALS_BDS2_B1
    + NUM_SIGNALS_BDS2_B2
    + 3 * NUM_SIGNALS_BDS3_B1C
    + 3 * NUM_SIGNALS_BDS3_B5
    + 3 * NUM_SIGNALS_BDS3_B7
    + 3 * NUM_SIGNALS_BDS3_B3;
/// Total number of signals in the Galileo constellation.
pub const NUM_SIGNALS_GAL: u16 = 4 * NUM_SIGNALS_GAL_E1
    + 3 * NUM_SIGNALS_GAL_E6
    + 3 * NUM_SIGNALS_GAL_E7
    + 3 * NUM_SIGNALS_GAL_E8
    + 3 * NUM_SIGNALS_GAL_E5;
/// Total number of signals in the QZSS constellation.
pub const NUM_SIGNALS_QZS: u16 = 2 * NUM_SIGNALS_QZS_L1
    + 3 * NUM_SIGNALS_QZS_L1C
    + 3 * NUM_SIGNALS_QZS_L2C
    + 3 * NUM_SIGNALS_QZS_L5;
/// Total number of signals across all constellations.
pub const NUM_SIGNALS: u16 = NUM_SIGNALS_GPS
    + NUM_SIGNALS_SBAS
    + NUM_SIGNALS_GLO
    + NUM_SIGNALS_BDS
    + NUM_SIGNALS_GAL
    + NUM_SIGNALS_QZS;

/// The first PRN number used in the GPS constellation.
pub const GPS_FIRST_PRN: u16 = 1;
/// The first PRN number used in the SBAS constellation.
pub const SBAS_FIRST_PRN: u16 = 120;
/// The first PRN number used in the GLONASS constellation.
pub const GLO_FIRST_PRN: u16 = 1;
/// The first PRN number used in the BeiDou constellation.
pub const BDS_FIRST_PRN: u16 = 1;
/// The first PRN number used in the Galileo constellation.
pub const GAL_FIRST_PRN: u16 = 1;
/// The first PRN number used in the QZSS constellation.
pub const QZS_FIRST_PRN: u16 = 193;

/// The GPS L1 center frequency in Hz.
pub const GPS_L1_HZ: f64 = 1.57542e9;
/// The GPS L2 center frequency in Hz.
pub const GPS_L2_HZ: f64 = 1.22760e9;
/// The GPS L5 center frequency in Hz.
pub const GPS_L5_HZ: f64 = 115. * 10.23e6;
/// The GLO L1 center frequency in Hz.
pub const GLO_L1_HZ: f64 = 1.602e9;
/// The GLO L2 center frequency in Hz.
pub const GLO_L2_HZ: f64 = 1.246e9;
/// Centre frequency of SBAS L1
pub const SBAS_L1_HZ: f64 = 1.023e6 * 1540.;
/// Centre frequency of SBAS L5
pub const SBAS_L5_HZ: f64 = 1.023e6 * 1150.;
/// Centre frequency of Beidou2 B1I
pub const BDS2_B1I_HZ: f64 = 1.023e6 * (1540. - 14.);
/// Centre frequency of Beidou2 B2
pub const BDS2_B2_HZ: f64 = 1.023e6 * 1180.;
/// Centre frequency of Beidou3 B1C
pub const BDS3_B1C_HZ: f64 = 154. * 10.23e6;
/// Centre frequency of Beidou3 B3
pub const BDS3_B3_HZ: f64 = 124. * 10.23e6;
/// Centre frequency of Beidou3 B2b
pub const BDS3_B7_HZ: f64 = 118. * 10.23e6;
/// Centre frequency of Beidou3 B2a
pub const BDS3_B5_HZ: f64 = 115. * 10.23e6;
/// Centre frequency of Galileo E1
pub const GAL_E1_HZ: f64 = 1.023e6 * 1540.;
/// Centre frequency of Galileo E6
pub const GAL_E6_HZ: f64 = 1.023e6 * 1250.;
/// Centre frequency of Galileo E5b
pub const GAL_E7_HZ: f64 = 1.023e6 * 1180.;
/// Centre frequency of Galileo E5AltBOC
pub const GAL_E8_HZ: f64 = 1.023e6 * 1165.;
/// Centre frequency of Galileo E5a
pub const GAL_E5_HZ: f64 = 1.023e6 * 1150.;
/// Centre frequency of QZSS L1CA
pub const QZS_L1_HZ: f64 = 1.023e6 * 1540.;
/// Centre frequency of QZSS L2C
pub const QZS_L2_HZ: f64 = 1.023e6 * 1200.;
/// Centre frequency of QZSS L5
pub const QZS_L5_HZ: f64 = 1.023e6 * 1150.;

/// Frequency range between two adjacent GLO channel in Hz for L1 band
pub const GLO_L1_DELTA_HZ: f64 = 5.625e5;
/// Frequency range between two adjacent GLO channel in Hz for L2 band
pub const GLO_L2_DELTA_HZ: f64 = 4.375e5;

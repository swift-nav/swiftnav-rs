use crate::c_bindings;

pub fn compute_crc24q(buf: &[u8], initial_value: u32) -> u32 {
    unsafe { c_bindings::crc24q(buf.as_ptr(), buf.len() as u32, initial_value) }
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

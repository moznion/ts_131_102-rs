//! PLMN contents processor.

use crate::binaryable::Binaryable;
use anyhow::Result;
use thiserror::Error;

/// PLMN: ref 10.5.1.13 / 3GPP TS 24.008 version 13.7.0 Release 13.
pub struct PLMN {
    mcc: u16,
    mnc: u16,
}

#[derive(Debug, Error, PartialEq)]
pub enum PLMNError {
    #[error("invalid MCC value; this must be within [0, 999] but the given value is {0}")]
    InvalidMCCValueRangeError(u16),
    #[error("invalid MNC value; this must be within [0, 999] but the given value is {0}")]
    InvalidMNCValueRangeError(u16),
}

pub fn new_plmn(mcc: u16, mnc: u16) -> Result<PLMN, PLMNError> {
    if mcc >= 1000 {
        return Err(PLMNError::InvalidMCCValueRangeError(mcc));
    }

    if mnc >= 1000 {
        return Err(PLMNError::InvalidMNCValueRangeError(mnc));
    }

    Ok(PLMN { mcc, mnc })
}

impl Binaryable for PLMN {
    fn to_bytes(&self) -> Vec<u8> {
        let mcc_digit1 = (self.mcc / 100) as u8;
        let mcc_digit2 = (self.mcc / 10 % 10) as u8;
        let mcc_digit3 = (self.mcc % 10) as u8;

        let mnc_digit1 = (self.mnc / 100) as u8;
        let mnc_digit2 = (self.mnc / 10 % 10) as u8;
        let mnc_digit3 = (self.mnc % 10) as u8;

        Vec::from([
            mcc_digit2 << 4 | mcc_digit1,
            mnc_digit3 << 4 | mcc_digit3,
            mnc_digit2 << 4 | mnc_digit1,
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::binaryable::Binaryable;
    use crate::plmn::{new_plmn, PLMNError};

    #[test]
    fn should_raise_error_when_mcc_is_invalid_on_new_plmn() {
        let result = new_plmn(1000, 410);
        assert_eq!(
            result.err().unwrap(),
            PLMNError::InvalidMCCValueRangeError(1000)
        );
    }

    #[test]
    fn should_raise_error_when_mnc_is_invalid_on_new_plmn() {
        let result = new_plmn(310, 1000);
        assert_eq!(
            result.err().unwrap(),
            PLMNError::InvalidMNCValueRangeError(1000)
        );
    }

    #[test]
    fn should_transform_struct_to_bytes_successfully() {
        let rai = new_plmn(310, 410).unwrap();
        assert_eq!(rai.to_bytes(), Vec::from([0x13, 0x00, 0x14]));
    }
}

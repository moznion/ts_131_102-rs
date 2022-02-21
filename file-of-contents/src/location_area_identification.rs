//! Location Area Identification (LAI) contents processor.

use crate::binaryable::Binaryable;
use crate::plmn::PLMN;

/// LocationAreaIdentification: ref 10.5.1.3 / 3GPP TS 24.008 version 13.7.0 Release 13.
/// Coding format:
///   +-------------------------------+
///   | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
///   +-------------------------------+
///   |  MCC digit 2  |  MCC digit 1  |
///   +-------------------------------+
///   |  MNC digit 3  |  MCC digit 3  |
///   +-------------------------------+
///   |  MNC digit 2  |  MNC digit 1  |
///   +-------------------------------+
///   |             LAC\[0]           |
///   +-------------------------------+
///   |             LAC\[1]           |
///   +-------------------------------+
pub struct LocationAreaIdentification<'a> {
    plmn: &'a PLMN,
    lac: &'a [u8; 2],
}

pub fn new_location_area_identification<'a>(
    plmn: &'a PLMN,
    lac: &'a [u8; 2],
) -> LocationAreaIdentification<'a> {
    LocationAreaIdentification { plmn, lac }
}

impl<'a> Binaryable for LocationAreaIdentification<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.plmn.to_bytes();
        bytes.extend_from_slice(&[self.lac[0], self.lac[1]]);
        bytes
    }
}

#[cfg(test)]
mod test {
    use crate::binaryable::Binaryable;
    use crate::location_area_identification::new_location_area_identification;
    use crate::plmn::new_plmn;

    #[test]
    fn should_transform_struct_to_bytes_successfully() {
        let plmn = new_plmn(310, 410).unwrap();
        let lai = new_location_area_identification(&plmn, &[0x01, 0x02]);
        assert_eq!(lai.to_bytes(), Vec::from([0x13, 0x00, 0x14, 0x01, 0x02]));
    }
}

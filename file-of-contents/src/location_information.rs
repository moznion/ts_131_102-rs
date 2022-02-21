//! Location Information (LOCI) contents processor.

use crate::binaryable::Binaryable;
use crate::identifier_providable::IdentifierProvidable;
use crate::location_area_identification::LocationAreaIdentification;
use crate::location_update_status::LocationUpdateStatus;

/// LocationInformation represents the LOCI data structure.
///
/// coding format:
///   +-------------------------------+
///   | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
///   +-------------------------------+
///   |            TMSI\[0]           |
///   +-------------------------------+
///   |            TMSI\[1]           |
///   +-------------------------------+
///   |            TMSI\[2]           |
///   +-------------------------------+
///   |            TMSI\[3]           |
///   +-------------------------------+
///   |            LAI\[0]            |
///   +-------------------------------+
///   |            LAI\[1]            |
///   +-------------------------------+
///   |            LAI\[2]            |
///   +-------------------------------+
///   |            LAI\[3]            |
///   +-------------------------------+
///   |            LAI\[4]            |
///   +-------------------------------+
///   |              RFU              |
///   +-------------------------------+
///   |     Location update stats     |
///   +-------------------------------+
/// ref: 4.2.17 / 3GPP TS 31.102 version 12.8.1 Release 12
pub struct LocationInformation<'a> {
    tmsi: &'a [u8; 4],
    lai: &'a LocationAreaIdentification<'a>,
    location_update_status: &'a LocationUpdateStatus,
}

pub fn new_location_information<'a>(
    tmsi: &'a [u8; 4],
    lai: &'a LocationAreaIdentification,
    location_update_status: &'a LocationUpdateStatus,
) -> LocationInformation<'a> {
    LocationInformation {
        tmsi,
        lai,
        location_update_status,
    }
}

impl<'a> Binaryable for LocationInformation<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from([self.tmsi[0], self.tmsi[1], self.tmsi[2], self.tmsi[3]]);
        bytes.extend_from_slice(self.lai.to_bytes().as_slice());
        bytes.extend_from_slice(&[0x00]);
        bytes.extend_from_slice(self.location_update_status.to_bytes().as_slice());
        bytes
    }
}

impl<'a> IdentifierProvidable for LocationInformation<'a> {
    fn get_identifier(&self) -> [u8; 2] {
        [0x6f, 0x7e]
    }
}

#[cfg(test)]
mod test {
    use crate::binaryable::Binaryable;
    use crate::location_area_identification::new_location_area_identification;
    use crate::location_information::new_location_information;
    use crate::location_update_status::LOCATION_UPDATE_STATUS_NOT_UPDATED;
    use crate::plmn::new_plmn;

    #[test]
    fn should_transform_struct_to_bytes() {
        let tmsi = [0x01, 0x02, 0x03, 0x04];
        let plmn = new_plmn(310, 260).unwrap();
        let lac = [0x10, 0x11];
        let lai = new_location_area_identification(&plmn, &lac);

        let location_information =
            new_location_information(&tmsi, &lai, &LOCATION_UPDATE_STATUS_NOT_UPDATED);

        assert_eq!(
            location_information.to_bytes(),
            Vec::from([0x01, 0x02, 0x03, 0x04, 0x13, 0x00, 0x62, 0x10, 0x11, 0x00, 0x01])
        );
    }
}

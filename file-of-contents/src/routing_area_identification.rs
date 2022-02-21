//! Routing Area Identification (RAI) contents processor.

use crate::binaryable::Binaryable;
use crate::location_area_identification::LocationAreaIdentification;

/// RoutingAreaIdentification represents the routing area identification.
///
/// coding format:
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
///   |               RAC             |
///   +-------------------------------+
/// the above table came from 10.5.148/3GPP TS 24.008
pub struct RoutingAreaIdentification<'a> {
    lai: &'a LocationAreaIdentification<'a>,
    rac: u8,
}

pub fn new_routing_area_identification<'a>(
    lai: &'a LocationAreaIdentification,
    rac: u8,
) -> RoutingAreaIdentification<'a> {
    RoutingAreaIdentification { lai, rac }
}

impl<'a> Binaryable for RoutingAreaIdentification<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.lai.to_bytes();
        bytes.extend_from_slice(&[self.rac]);
        bytes
    }
}

#[cfg(test)]
mod test {
    use crate::binaryable::Binaryable;
    use crate::location_area_identification::new_location_area_identification;
    use crate::plmn::new_plmn;
    use crate::routing_area_identification::new_routing_area_identification;

    #[test]
    fn should_transform_struct_to_bytes_successfully() {
        let plmn = new_plmn(310, 410).unwrap();
        let rac = [0x01, 0x02];
        let lai = new_location_area_identification(&plmn, &rac);
        let rai = new_routing_area_identification(&lai, 0xff);
        assert_eq!(
            rai.to_bytes(),
            Vec::from([0x13, 0x00, 0x14, 0x01, 0x02, 0xff])
        );
    }
}

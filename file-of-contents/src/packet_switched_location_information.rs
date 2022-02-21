//! Packet Switched Location Information (EF PSLOCI) contents processor.

use crate::binaryable::Binaryable;
use crate::identifier_providable::IdentifierProvidable;
use crate::routing_area_identification::RoutingAreaIdentification;
use crate::routing_area_update_status::RoutingAreaUpdateStatus;

/// PacketSwitchedLocationInformation represents the PSLOCI data structure.
///
/// coding format:
///   +-------------------------------+
///   | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
///   +-------------------------------+
///   |          P-TMSI\[0]           |
///   +-------------------------------+
///   |          P-TMSI\[1]           |
///   +-------------------------------+
///   |          P-TMSI\[2]           |
///   +-------------------------------+
///   |          P-TMSI\[3]           |
///   +-------------------------------+
///   |      P-TMSI Signature\[0]     |
///   +-------------------------------+
///   |      P-TMSI Signature\[1]     |
///   +-------------------------------+
///   |      P-TMSI Signature\[2]     |
///   +-------------------------------+
///   |            RAI\[0]            |
///   +-------------------------------+
///   |            RAI\[1]            |
///   +-------------------------------+
///   |            RAI\[2]            |
///   +-------------------------------+
///   |            RAI\[3]            |
///   +-------------------------------+
///   |            RAI\[4]            |
///   +-------------------------------+
///   |            RAI\[5]            |
///   +-------------------------------+
///   |   Routing Area update stats   |
///   +-------------------------------+
/// the above table came from 4.2.23 / 3GPP TS 31.102 version 12.8.1 Release 12
pub struct PacketSwitchedLocationInformation<'a> {
    packet_tmsi: &'a [u8; 4],
    packet_tmsi_signature: &'a [u8; 3],
    rai: &'a RoutingAreaIdentification<'a>,
    routing_area_update_status: &'a RoutingAreaUpdateStatus,
}

pub fn new_packet_switched_location_information<'a>(
    packet_tmsi: &'a [u8; 4],
    packet_tmsi_signature: &'a [u8; 3],
    rai: &'a RoutingAreaIdentification,
    routing_area_update_status: &'a RoutingAreaUpdateStatus,
) -> PacketSwitchedLocationInformation<'a> {
    PacketSwitchedLocationInformation {
        packet_tmsi,
        packet_tmsi_signature,
        rai,
        routing_area_update_status,
    }
}

impl<'a> Binaryable for PacketSwitchedLocationInformation<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from([
            self.packet_tmsi[0],
            self.packet_tmsi[1],
            self.packet_tmsi[2],
            self.packet_tmsi[3],
            self.packet_tmsi_signature[0],
            self.packet_tmsi_signature[1],
            self.packet_tmsi_signature[2],
        ]);
        bytes.extend_from_slice(self.rai.to_bytes().as_slice());
        bytes.extend_from_slice(self.routing_area_update_status.to_bytes().as_slice());
        bytes
    }
}

impl<'a> IdentifierProvidable for PacketSwitchedLocationInformation<'a> {
    fn get_identifier(&self) -> [u8; 2] {
        [0x6f, 0x73]
    }
}

#[cfg(test)]
mod test {
    use crate::binaryable::Binaryable;
    use crate::location_area_identification::new_location_area_identification;
    use crate::packet_switched_location_information::new_packet_switched_location_information;
    use crate::plmn::new_plmn;
    use crate::routing_area_identification::new_routing_area_identification;
    use crate::routing_area_update_status::ROUTING_AREA_UPDATE_STATUS_NOT_UPDATED;

    #[test]
    fn should_transform_struct_to_bytes() {
        let plmn = new_plmn(310, 410).unwrap();
        let rac = [0x01, 0x02];
        let lai = new_location_area_identification(&plmn, &rac);
        let rai = new_routing_area_identification(&lai, 0xff);
        let psloci = new_packet_switched_location_information(
            &[0xe0, 0xe1, 0xe2, 0xe3],
            &[0xf0, 0xf1, 0xf2],
            &rai,
            &ROUTING_AREA_UPDATE_STATUS_NOT_UPDATED,
        );

        let mut expected = Vec::from([0xe0, 0xe1, 0xe2, 0xe3, 0xf0, 0xf1, 0xf2]);
        expected.extend_from_slice(rai.to_bytes().as_slice());
        expected.extend_from_slice(ROUTING_AREA_UPDATE_STATUS_NOT_UPDATED.to_bytes().as_slice());

        assert_eq!(psloci.to_bytes(), expected);
    }
}

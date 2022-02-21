//! RoutingAreaUpdateStatus.
//! ref: 4.2.23 / 3GPP TS 31.102 version 12.8.1 Release 12

use crate::binaryable::Binaryable;

#[derive(Copy, Clone)]
pub struct RoutingAreaUpdateStatus {
    value: u8,
}

pub const ROUTING_AREA_UPDATE_STATUS_UPDATED: RoutingAreaUpdateStatus =
    RoutingAreaUpdateStatus { value: 0b00000000 };
pub const ROUTING_AREA_UPDATE_STATUS_NOT_UPDATED: RoutingAreaUpdateStatus =
    RoutingAreaUpdateStatus { value: 0b00000001 };
pub const ROUTING_AREA_UPDATE_STATUS_PLMN_NOT_ALLOWED: RoutingAreaUpdateStatus =
    RoutingAreaUpdateStatus { value: 0b00000010 };
pub const ROUTING_AREA_UPDATE_STATUS_ROUTING_AREA_NOT_ALLOWED: RoutingAreaUpdateStatus =
    RoutingAreaUpdateStatus { value: 0b00000011 };
pub const ROUTING_AREA_UPDATE_STATUS_RESERVED: RoutingAreaUpdateStatus =
    RoutingAreaUpdateStatus { value: 0b00000111 };

impl Binaryable for RoutingAreaUpdateStatus {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::from([self.value])
    }
}

//! LocationUpdateStatus.
//! ref: 4.2.17 / 3GPP TS 31.102 version 12.8.1 Release 12

use crate::binaryable::Binaryable;

#[derive(Copy, Clone)]
pub struct LocationUpdateStatus {
    value: u8,
}

pub const LOCATION_UPDATE_STATUS_UPDATED: LocationUpdateStatus =
    LocationUpdateStatus { value: 0b00000000 };
pub const LOCATION_UPDATE_STATUS_NOT_UPDATED: LocationUpdateStatus =
    LocationUpdateStatus { value: 0b00000001 };
pub const LOCATION_UPDATE_STATUS_PLMN_NOT_ALLOWED: LocationUpdateStatus =
    LocationUpdateStatus { value: 0b00000010 };
pub const LOCATION_UPDATE_STATUS_LOCATION_AREA_NOT_ALLOWED: LocationUpdateStatus =
    LocationUpdateStatus { value: 0b00000011 };
pub const LOCATION_UPDATE_STATUS_RESERVED: LocationUpdateStatus =
    LocationUpdateStatus { value: 0b00000111 };

impl Binaryable for LocationUpdateStatus {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::from([self.value])
    }
}

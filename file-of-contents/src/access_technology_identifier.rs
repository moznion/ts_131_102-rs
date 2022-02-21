//! Access Technology Identifier contents processor.
//! ref: 4.2.5 / 3GPP TS 31.102 version 12.8.1 Release 12

use crate::binaryable::Binaryable;

pub struct AccessTechnologyIdentifier {
    architecture: u8,
    technology: u8,
}

#[repr(u8)]
pub enum RadioAccessNetworkArchitecture {
    UTRAN = 0b10000000,
    EUTRAN = 0b01000000,
}

#[repr(u8)]
pub enum AccessNetworkTechnology {
    CDMA2000_1xRTT = 0b00010000,
    CDMA2000HRPD = 0b00100000,
    GSMCompact = 0b01000000,
    GSM = 0b10000000,
}

pub fn new_access_technology_identifier(
    architectures: Vec<RadioAccessNetworkArchitecture>,
    technologies: Vec<AccessNetworkTechnology>,
) -> AccessTechnologyIdentifier {
    let mut architecture_byte: u8 = 0;
    for arch in architectures {
        architecture_byte |= arch as u8;
    }

    let mut technology_byte: u8 = 0;
    for tech in technologies {
        technology_byte |= tech as u8;
    }

    AccessTechnologyIdentifier {
        architecture: architecture_byte,
        technology: technology_byte,
    }
}

impl Binaryable for AccessTechnologyIdentifier {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::from([self.architecture, self.technology])
    }
}

#[cfg(test)]
mod test {
    use crate::access_technology_identifier::AccessNetworkTechnology::{
        GSMCompact, CDMA2000HRPD, GSM,
    };
    use crate::access_technology_identifier::RadioAccessNetworkArchitecture::EUTRAN;
    use crate::access_technology_identifier::{
        new_access_technology_identifier, AccessNetworkTechnology, RadioAccessNetworkArchitecture,
    };
    use crate::binaryable::Binaryable;
    use AccessNetworkTechnology::CDMA2000_1xRTT;
    use RadioAccessNetworkArchitecture::UTRAN;

    #[test]
    pub fn should_transform_struct_to_bytes_successfully() {
        let identifier = new_access_technology_identifier(Vec::new(), Vec::new());
        assert_eq!(identifier.to_bytes(), [0b00000000, 0b00000000]);

        let identifier = new_access_technology_identifier(
            Vec::from([UTRAN, EUTRAN]),
            Vec::from([CDMA2000_1xRTT, CDMA2000HRPD, GSMCompact, GSM]),
        );
        assert_eq!(identifier.to_bytes(), [0b11000000, 0b11110000]);
    }
}

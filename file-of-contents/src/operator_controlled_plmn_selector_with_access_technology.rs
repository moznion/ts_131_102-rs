//! Operator controlled PLMN selector with Access Technology (EF OPLMNwACT) contents processor.

use crate::access_technology_identifier::AccessTechnologyIdentifier;
use crate::binaryable::Binaryable;
use crate::identifier_providable::IdentifierProvidable;
use crate::plmn::PLMN;

/// OperatorControlledPLMNSelectorWithAccessTechnology: ref 4.2.53 / 3GPP TS 31.102 version 12.8.1 Release 12.
pub struct OperatorControlledPLMNSelectorWithAccessTechnology<'a> {
    plmns_info: &'a [(PLMN, AccessTechnologyIdentifier)],
}

pub fn new_operator_controlled_plmn_selector_with_access_technology(
    plmns_info: &[(PLMN, AccessTechnologyIdentifier)],
) -> OperatorControlledPLMNSelectorWithAccessTechnology {
    OperatorControlledPLMNSelectorWithAccessTechnology { plmns_info }
}

impl<'a> Binaryable for OperatorControlledPLMNSelectorWithAccessTechnology<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for p in self.plmns_info {
            bytes.extend_from_slice(p.0.to_bytes().as_slice());
            bytes.extend_from_slice(p.1.to_bytes().as_slice());
        }
        bytes
    }
}

impl<'a> IdentifierProvidable for OperatorControlledPLMNSelectorWithAccessTechnology<'a> {
    fn get_identifier(&self) -> [u8; 2] {
        [0x6f, 0x61]
    }
}

#[cfg(test)]
mod test {
    use crate::access_technology_identifier::{
        new_access_technology_identifier,
        AccessNetworkTechnology::GSM,
        RadioAccessNetworkArchitecture::{EUTRAN, UTRAN},
    };
    use crate::binaryable::Binaryable;
    use crate::operator_controlled_plmn_selector_with_access_technology::new_operator_controlled_plmn_selector_with_access_technology;
    use crate::plmn::new_plmn;

    #[test]
    fn should_transform_struct_to_bytes_successfully() {
        let plmns_info = [
            (
                new_plmn(310, 410).unwrap(),
                new_access_technology_identifier(Vec::from([UTRAN, EUTRAN]), Vec::from([])),
            ),
            (
                new_plmn(310, 260).unwrap(),
                new_access_technology_identifier(Vec::from([EUTRAN]), Vec::from([GSM])),
            ),
        ];
        let oplmnwact = new_operator_controlled_plmn_selector_with_access_technology(&plmns_info);

        assert_eq!(
            oplmnwact.to_bytes(),
            Vec::from([0x13, 0x00, 0x14, 0xC0, 0x00, 0x13, 0x00, 0x62, 0x40, 0x80,])
        );
    }
}

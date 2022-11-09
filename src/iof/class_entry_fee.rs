use xmltree::Element;
use std::convert::TryFrom;
use crate::iof::{numeric_contents,IOFXMLError,ClassEntryFee};

impl TryFrom<&Element> for ClassEntryFee {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id: u64 = numeric_contents(element, "EntryFeeId")
            .ok_or("Entry fee id missing or malformed for class entry fee!")?;
        let sequence: u64 = numeric_contents(element, "Sequence")
            .ok_or("Sequence number missing or malformed for class entry fee!")?;

        Ok( ClassEntryFee { id, sequence } )
    }
}
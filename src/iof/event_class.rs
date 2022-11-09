use xmltree::Element;
use std::convert::TryFrom;
use crate::iof::{numeric_contents,subelements,IOFXMLError,EventClass,ClassEntryFee,EntryFee};

impl TryFrom<&Element> for EventClass {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id: u64 = numeric_contents(element, "EventClassId")
            .ok_or("Event class id missing or malformed!")?;

        let mut fees: Vec<ClassEntryFee> = subelements(element, "ClassEntryFee")?;
        fees.sort_by(|a, b| a.sequence.cmp(&b.sequence));

        Ok( EventClass { id, fee_ids: fees.into_iter().map(|f| f.id).collect() } )
    }
}

impl EventClass {

    pub fn paid_direct_entry_fees(&self, birth_year: &u64, entry_fees: &Vec<EntryFee>) -> (f64, f64) {
        let applicable_fee_ids = self.fee_ids.iter()
            .map(|fee_id| entry_fees.iter().find(|fee| fee.id == *fee_id).expect("Invalid entry fee!"))
            .filter(|fee| match (fee.from_year_of_birth, fee.to_year_of_birth) {
                (Some(from_year), _) if birth_year < &from_year => false,
                (_, Some(to_year)) if birth_year > &to_year => false,
                _ => true,
            })
            .map(|fee| fee.id)
            .collect();
        EntryFee::paid_fees_from_fee_ids(&applicable_fee_ids, entry_fees)
    }
}
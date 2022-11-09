use crate::iof::{subelements, IOFXMLError};
use crate::iof::{ClassEntryFee, Entrant, Entry, EntryFee};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for Entry {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let entrant = element.try_into()?;
        let mut fees: Vec<ClassEntryFee> = subelements(element, "EntryEntryFee")?;
        fees.sort_by(|a, b| a.sequence.cmp(&b.sequence));

        Ok(Entry {
            entrant,
            fee_ids: fees.into_iter().map(|f| f.id).collect(),
        })
    }
}

impl Entry {
    pub fn is_for_person(&self, person_id: &Option<u64>) -> bool {
        if let Some(person_id) = person_id {
            match self.entrant {
                Entrant::Individual(id) if id == *person_id => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn paid_fees(&self, entry_fees: &Vec<EntryFee>) -> (f64, f64) {
        EntryFee::paid_fees_from_fee_ids(&self.fee_ids, entry_fees)
    }
}

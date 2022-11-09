use crate::iof::{numeric_contents, textual_contents, year_from_date_string, IOFXMLError};
use crate::iof::{EntryFee, ValueOperator};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for EntryFee {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id: u64 =
            numeric_contents(element, "EntryFeeId").ok_or("Entry fee id missing or malformed!")?;
        let name = textual_contents(element, "Name").ok_or("Entry fee name missing!")?;
        let amount: f64 =
            numeric_contents(element, "Amount").ok_or("Entry fee amount missing or malformed!")?;
        let operator = match element.attributes.get("valueOperator") {
            Some(value) => match value.as_str() {
                "fixed" => Ok(ValueOperator::Fixed),
                "percent" => Ok(ValueOperator::Percent),
                _ => Err("Unrecognized value operator for entry fee."),
            },
            None => Err("No value operator specified for entry fee."),
        }?;

        let from_year_of_birth: Option<u64> =
            if let Some(from_date_of_birth_element) = element.get_child("FromDateOfBirth") {
                if let Some(text) = textual_contents(from_date_of_birth_element, "Date") {
                    year_from_date_string(&text)
                } else {
                    None
                }
            } else {
                None
            };
        let to_year_of_birth: Option<u64> =
            if let Some(to_date_of_birth_element) = element.get_child("ToDateOfBirth") {
                if let Some(text) = textual_contents(to_date_of_birth_element, "Date") {
                    year_from_date_string(&text)
                } else {
                    None
                }
            } else {
                None
            };

        Ok(EntryFee {
            id,
            name,
            amount,
            operator,
            from_year_of_birth,
            to_year_of_birth,
        })
    }
}

impl EntryFee {
    pub fn paid_fees_from_fee_ids(
        applicable_fee_ids: &Vec<u64>,
        event_fees: &Vec<EntryFee>,
    ) -> (f64, f64) {
        applicable_fee_ids
            .iter()
            .fold((0f64, 0f64), |acc, fee_id| -> (f64, f64) {
                let fee = event_fees
                    .iter()
                    .find(|event_fee| event_fee.id == *fee_id)
                    .expect("Event fee id not found!");
                // This code makes some assumptions on how fee types are usually applied, since the division
                // between a normal fee and a late fee is not present in the Eventor data model.
                match fee.operator {
                    ValueOperator::Fixed => (acc.0 + acc.1 + fee.amount, acc.1),
                    ValueOperator::Percent => (acc.0, (acc.1 + acc.0 * fee.amount / 100f64)),
                }
            })
    }
}

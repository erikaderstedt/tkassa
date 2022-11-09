use crate::iof::Race;
use crate::iof::{numeric_contents, EventorTime, IOFXMLError};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for Race {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id: u64 = numeric_contents(element, "EventRaceId")
            .ok_or("Event race id missing or malformed!")?;
        let date: EventorTime = element
            .get_child("RaceDate")
            .ok_or("Event race missing race date!")?
            .try_into()?;

        Ok(Race { id, date })
    }
}

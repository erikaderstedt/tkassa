use crate::iof::{Competitor, IOFXMLError, PersonResult};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for PersonResult {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let competitor: Competitor = element
            .get_child("Person")
            .ok_or("No person was specified for person result.")?
            .try_into()?;

        // If this is a multi-day event, the Result element is buried within
        // a RaceResult element.
        let parent = element.get_child("RaceResult").unwrap_or(element);
        let status = parent
            .get_child("Result")
            .ok_or("Missing result element from person result")?
            .get_child("CompetitorStatus")
            .ok_or("Missing competitor status from result")?;

        let dns = match status.attributes.get("value") {
            Some(v) => v == "DidNotStart",
            _ => false,
        };

        Ok(PersonResult { competitor, dns })
    }
}

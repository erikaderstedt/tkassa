use xmltree::Element;
use std::convert::TryFrom;
use crate::iof::{numeric_contents,IOFXMLError};
use crate::iof::Entrant;

impl TryFrom<&Element> for Entrant {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.get_child("Competitor") {
            Some(element) => Ok(match numeric_contents(element, "PersonId") {
                Some(id) => Entrant::Individual(id),
                None => Entrant::Unknown,
            }),
            None => Ok(Entrant::Team),
        }
    }
}
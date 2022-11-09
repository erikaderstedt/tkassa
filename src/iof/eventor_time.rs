use crate::iof::{textual_contents, EventorTime, IOFXMLError};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for EventorTime {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let date = textual_contents(element, "Date")
            .ok_or("Eventor timestamp object is missing date!")?
            .replace("-", "")
            .parse::<u64>()
            .map_err(|_| "Bad date in eventor timestamp object")?;
        Ok(EventorTime { date })
    }
}

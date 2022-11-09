use xmltree::Element;
use std::convert::TryFrom;
use crate::iof::{numeric_contents,subelements,IOFXMLError,ClassResult};

impl TryFrom<&Element> for ClassResult {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let event_class_element = element
            .get_child("EventClass")
            .ok_or("No event class was specified for class result.")?;
        let event_class_id = numeric_contents(event_class_element, "EventClassId")
            .ok_or("Missing event class id in class result.")?;
        let class_race_info_element = event_class_element
            .get_child("ClassRaceInfo")
            .ok_or("Event class is missing class race info")?;
        let event_race_id = numeric_contents(class_race_info_element, "EventRaceId")
            .ok_or("Class race info is missing the event race id (or it is malformed)")?;

        let person_results = subelements(element, "PersonResult")?;

        Ok( ClassResult { event_class_id, event_race_id, person_results })
    }
}
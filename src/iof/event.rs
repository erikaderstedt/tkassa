use crate::iof::{numeric_contents, subelements, textual_contents, IOFXMLError};
use crate::iof::{Event, EventorTime};
use std::convert::TryFrom;
use xmltree::Element;

impl TryFrom<&Element> for Event {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id: u64 =
            numeric_contents(element, "EventId").ok_or("Event id missing or malformed!")?;
        let name = textual_contents(element, "Name").ok_or("Event name missing!")?;
        let races = subelements(element, "EventRace")?;

        Ok(Event { id, name, races })
    }
}

impl Event {
    pub fn first_race_date(&self) -> u64 {
        self.races.first().map(|race| race.date.date).unwrap_or(0)
    }

    pub fn date_for_race(&self, event_race_id: &u64) -> EventorTime {
        self.races
            .iter()
            .find(|race| race.id == *event_race_id)
            .expect("Unknown race id!")
            .date
            .clone()
    }
}

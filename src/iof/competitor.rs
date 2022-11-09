use xmltree::Element;
use std::convert::TryFrom;
use crate::iof::{numeric_contents,textual_contents,IOFXMLError,Competitor};

impl TryFrom<&Element> for Competitor {
    type Error = IOFXMLError;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id = numeric_contents(element, "PersonId");
        let name_element = element.get_child("PersonName")
                .ok_or("No name given for person")?;
        let family = textual_contents(name_element, "Family").ok_or("No family name specified for competitor.")?;
        let given = textual_contents(name_element, "Given").ok_or("No given name specified for competitor.")?;
        let mut birth_year: Option<u64> = None;
        if let Some(birth_year_element) = element.get_child("BirthDate") {
            birth_year = textual_contents(birth_year_element, "Date")
            .map(|d| if d.len() > 4 { d[0..4].parse::<u64>().ok() } else { None })
            .flatten();
        }

        Ok( Competitor { id, given, family, birth_year } )
    }
}

impl Competitor {
    pub fn probably_the_same_as(&self, other: &Competitor) -> bool {
        self.given == other.given && self.family == other.family
    }
}
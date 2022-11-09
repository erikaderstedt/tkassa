use std::convert::TryFrom;

pub type IOFXMLError = &'static str;

mod class_entry_fee;
mod class_result;
mod competitor;
mod entrant;
mod entry;
mod entry_fee;
mod event;
mod event_class;
mod eventor_time;
mod person_result;
mod race;

#[derive(Debug)]
struct Race {
    id: u64,
    date: EventorTime,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Competitor {
    pub id: Option<u64>,
    pub given: String,
    pub family: String,
    pub birth_year: Option<u64>,
}

#[derive(Debug)]
pub struct ClassEntryFee {
    id: u64,
    sequence: u64,
}

#[derive(Debug)]
pub struct EventClass {
    pub id: u64,
    pub fee_ids: Vec<u64>,
}

#[derive(Debug)]
pub struct PersonResult {
    pub competitor: Competitor,
    pub dns: bool,
}

#[derive(Debug)]
pub struct ClassResult {
    pub event_class_id: u64,
    pub event_race_id: u64,
    pub person_results: Vec<PersonResult>,
}

#[derive(Debug)]
pub struct Event {
    pub id: u64,
    pub name: String,
    races: Vec<Race>,
}

#[derive(Debug)]
pub enum Entrant {
    Unknown,
    Individual(u64),
    Team,
}

#[derive(Debug)]
pub struct Entry {
    pub entrant: Entrant,
    fee_ids: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct EventorTime {
    pub date: u64,
}

#[derive(Debug)]
enum ValueOperator {
    Fixed,
    Percent,
}

#[derive(Debug)]
pub struct EntryFee {
    id: u64,
    pub name: String,
    amount: f64,
    operator: ValueOperator,
    from_year_of_birth: Option<u64>,
    to_year_of_birth: Option<u64>,
}

fn textual_contents(element: &xmltree::Element, child_name: &str) -> Option<String> {
    element
        .get_child(child_name)
        .map(|element| element.get_text())
        .flatten()
        .map(|c| c.to_string())
}

pub fn year_from_date_string(date_string: &String) -> Option<u64> {
    if date_string.len() < 4 {
        None
    } else {
        date_string[0..4].parse::<u64>().ok()
    }
}

fn numeric_contents<T: std::str::FromStr>(
    element: &xmltree::Element,
    child_name: &str,
) -> Option<T> {
    textual_contents(element, child_name)?.parse::<T>().ok()
}

pub fn subelements<'a, T: TryFrom<&'a xmltree::Element, Error = IOFXMLError>>(
    element: &'a xmltree::Element,
    child_name: &str,
) -> Result<Vec<T>, IOFXMLError> {
    element
        .children
        .iter()
        .filter_map(|node| -> Option<Result<T, IOFXMLError>> {
            match node {
                xmltree::XMLNode::Element(element) if element.name == child_name => {
                    Some(element.try_into())
                }
                _ => None,
            }
        })
        .collect::<Result<Vec<T>, IOFXMLError>>()
}

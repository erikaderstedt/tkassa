use std::env;
use getopts::Options;
use iof::year_from_date_string;
use xmltree;
use crate::iof::subelements;

mod eventor;
mod iof;

const EVENTS: &str = "https://eventor.orientering.se/api/events";
const ORGANISATION_RESULTS: &str = "https://eventor.orientering.se/api/results/organisation";
const EVENT_CLASSES: &str = "https://eventor.orientering.se/api/eventclasses";
const ENTRIES: &str = "https://eventor.orientering.se/api/entries";
const ENTRY_FEES: &str = "https://eventor.orientering.se/api/entryfees/events/";

fn print_usage(opts: Options) {
    let brief = format!("Usage: tkassa [options] <API key> <from date YYYY-MM-DD> <to date YYYY-MM-DD>");
    print!("{}", opts.usage(&brief));
}

#[derive(Debug)]
struct BillableEvent {
    race_date: u64,
    event_name: String,
    normal_fee: f64,
    late_fee: f64,
    dns: bool
}

#[derive(Debug)]
struct Person {
    person: iof::Competitor,
    billable: Vec<BillableEvent>,
}

struct DataExtractor {
    verbose: bool,
    organisation_id: u64,
    api_key: String,
    cache_folder: String,
    ignore_events: Vec<u64>,
    current_year: u64,
    from_date: String,
    to_date: String,
}

impl DataExtractor {

    fn from(opts: &Options) -> Result<Self, Option<String>> {
        let args: Vec<String> = env::args().collect();

        match opts.parse(&args[1..]) {
            Ok(matches) => { 
                if matches.opt_present("h") {
                    Err(None)
                } else if matches.free.len() < 3 {
                    Err(Some("Too few arguments.".to_string()))
                } else if matches.free[1].len() < 4 {
                    Err(Some("Starting date is too short.".to_string()))
                } else {
                    let verbose = !matches.opt_present("q");
                    let cache_folder = matches.opt_str("c").unwrap_or(".".to_string());
                    let organisation_id = matches
                        .opt_str("o")
                        .map(|v| v.parse::<u64>().ok())
                        .flatten()
                        .unwrap_or(224); // Kungälvs OK
                    let api_key = (&matches.free[0]).to_string();
                    let from_date = (&matches.free[1]).to_string();
                    let to_date = (&matches.free[2]).to_string();
                    let ignore_events: Vec<u64> = matches
                        .opt_str("i")
                        .unwrap_or("".to_string())
                        .split(",")
                        .filter_map(|p| p.parse::<u64>().ok())
                        .collect();
                    match year_from_date_string(&from_date) {
                        None => Err(Some("Invalid starting year.".to_string())),
                        Some(current_year) => Ok( DataExtractor {
                            verbose, organisation_id, api_key,
                            cache_folder, ignore_events,
                            current_year, from_date, to_date
                        }),
                    }
                }
            },
            Err(f) => { return Err(Some(format!("Unable to parse command-line options: {:?}", f.to_string()))) }
        }
    }

    fn run(&self) {
        let eventor_client = eventor::EventorClient::new(&self.api_key, &self.cache_folder, self.verbose); 
            
        let event_list = eventor_client.request(EVENTS, 
            &[("fromDate", self.from_date.as_str()), 
                        ("toDate", self.to_date.as_str())]);

        let mut events: Vec<iof::Event> = subelements(&event_list, "Event")
            .expect("XML parsing error when reading event list");

        events.sort_by_key(|e| e.first_race_date());

        let mut persons: Vec<Person> = vec![];
        for event in events.iter_mut() {
            if self.ignore_events.contains(&event.id) {
                continue;
            }
            // Get the result list. Will be read in more detail later. 
            let result_list = eventor_client.request(ORGANISATION_RESULTS, 
                &[("organisationIds", &self.organisation_id.to_string()), 
                            ("eventId", &event.id.to_string())]);

            // First we just check that it contains any ClassResult. If not, then noone from our club was at
            // the event (and were not pre-entered either).
            if result_list.get_child("ClassResult").is_none() {
                continue
            }

            if self.verbose {
                println!("Event '{}'", event.name);
            }

            // Get entry fees.
            let entry_fee_url: String = ENTRY_FEES.to_owned() + &event.id.to_string(); // & format!(ENTRY_FEES, event.id);
            let entry_fee_list: xmltree::Element = eventor_client.request(entry_fee_url, &[("eventId", &event.id.to_string())]);
            let entry_fees: Vec<iof::EntryFee> = iof::subelements(&entry_fee_list, "EntryFee")
                .expect("XML parsing error when reading entry fee list");

            // Get event classes
            let class_list: xmltree::Element = eventor_client.request(EVENT_CLASSES, 
            &[("includeEntryFees", "true"), ("eventId", &event.id.to_string())]);
            let event_classes: Vec<iof::EventClass> = iof::subelements(&class_list, "EventClass")
                .expect("XML parsing error when reading event classes");
            
            // Get pre-entries
            let entry_list: xmltree::Element = eventor_client.request(ENTRIES, 
            &[("includeEntryFees", "true"), 
                        ("organisationIds", &self.organisation_id.to_string()), 
                        ("eventIds", &event.id.to_string())]);
            let entries: Vec<iof::Entry> = iof::subelements(&entry_list, "Entry")
                .expect("XML parsing error while reading entry list");

            let class_results: Vec<iof::ClassResult> = subelements(&result_list, "ClassResult")
                .expect("XML parsing error when reading result list");

            // for each result
            for class in class_results.iter() {
                let race_date = event.date_for_race(&class.event_race_id);
                // We are not guaranteed to find the event class, if the entry classes are different from the race
                // classes (such as for elite events with qualifications).
                let event_class = event_classes.iter().find(|event_class| event_class.id == class.event_class_id);

                for person_result in class.person_results.iter() {
                    // find reference to person in persons, or create new.
                    let mut existing_person = persons
                        .iter_mut()
                        .find(|x| x.person == person_result.competitor);
                    if existing_person.is_none() {
                        existing_person = match persons
                            .iter_mut()
                            .find(|x| x.person.probably_the_same_as(&person_result.competitor)) 
                        {
                            Some(p) => Some(p),
                            None => {
                                persons.push(Person { person: person_result.competitor.clone(), billable: vec![] });
                                persons.last_mut()
                            },
                        };
                    }
                    let existing_person = existing_person.unwrap();
                    
                    let paid = 
                    // Is this person pre-registered?
                    if let Some(entry) = entries.iter().find(|entry| entry.is_for_person(&existing_person.person.id)){
                        // Yes.
                        entry.paid_fees(&entry_fees)
                    } else if let Some(event_class) = event_class {
                        // No? Ok. Then we get the class id, and the fees from there.
                    event_class.paid_direct_entry_fees(
                            & person_result.competitor.birth_year.unwrap_or(self.current_year), 
                            &entry_fees)
                    } else {
                        panic!("The person was not pre-registered and the class id is unknown.");
                    };

                    existing_person.billable.push(
                        BillableEvent { 
                            race_date: race_date.date , 
                            event_name: event.name.clone(), 
                            normal_fee: paid.0, 
                            late_fee: paid.1, 
                            dns: person_result.dns 
                        }
                    );
                }
            }
        }

        // Present the results, sorted by last name.
        persons.sort_by_key(|person| person.person.family.clone());

        for p in persons.iter_mut() {
            let id: String = match p.person.id {
                Some(i) => i.to_string(),
                None => "????".to_string(),
            };
            let byear = match p.person.birth_year {
                Some(y) => y.to_string(),
                None => "????".to_string(),
            };
            println!("{}\t{}\t{}\t{}", id, p.person.given, p.person.family, byear);
            p.billable.sort_by_key(|b| b.race_date);
            for b in p.billable.iter() {
                println!("\t{}\t{}\t{}\t{}\t{}",
                    b.race_date, b.event_name, b.normal_fee as u64, b.late_fee as u64, if b.dns { "DNS" } else { "" })
            }
        }
    }
}

fn main() {
    let mut opts = Options::new();

    opts.optflag("q", "quiet", "hide additional information while running");
    opts.optopt("i", "ignore", "comma-separated list of event IDs to ignore", "34567,35112");
    opts.optopt("c", "cache", "cache folder for requests", "caches/");
    opts.optopt("o", "org_id", "organisation id", "224");
    opts.optflag("h", "help", "show this help menu");
    
    match DataExtractor::from(&opts) {
        Err(problem) => {
            if let Some(problem) = problem {
                println!("ERROR: {}", problem);
            }
            print_usage(opts);
        },
        Ok(extractor) => {
            extractor.run();
        }
    };
    
    
}

//! Next Bus Predictions Command

use error::Error;
use request::{Command, Request};
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};


/// Predictions for a route. Maps directly from Nextbus response.
#[derive(Debug, PartialEq)]
pub struct Predictions {
    agency_title: String,
    route_tag: String,
    route_code: Option<String>,
    route_title: String,
    stop_title: String,
    dir_title_because_no_predictions: Option<String>,
    directions: Vec<Direction>,
    messages: Vec<Message>,
}

impl Predictions {
    pub fn new(agency_title: String,
               route_tag: String,
               route_code: Option<String>,
               route_title: String,
               stop_title: String,
               dir_title_because_no_predictions: Option<String>,
               directions: Vec<Direction>,
               messages: Vec<Message>,
              ) -> Self {
        Predictions {
            agency_title: agency_title,
            route_tag: route_tag,
            route_code: route_code,
            route_title: route_title,
            stop_title: stop_title,
            dir_title_because_no_predictions: dir_title_because_no_predictions,
            directions: directions,
            messages: messages,
        }
    }
}

// Builder
// ===============================================================

pub struct PredictionsBuilder<'a> {
    agency: Option<&'a str>,
    route: Option<&'a str>,
    stop: Option<&'a str>,
}

impl<'a> PredictionsBuilder<'a> {
    pub fn new() -> Self {
        PredictionsBuilder {
            agency: None,
            route: None,
            stop: None,
        }
    }

    /// Builder to set agency
    pub fn agency(&mut self, agency: &'a str) -> &mut Self {
        self.agency = Some(agency);
        self
    }

    /// Builder to set route
    pub fn route(&mut self, route: &'a str) -> &mut Self {
        self.route = Some(route);
        self
    }

    /// Builder to set route
    pub fn stop(&mut self, stop: &'a str) -> &mut Self {
        self.stop = Some(stop);
        self
    }

    pub fn get(&self) -> ::Result<Predictions> {
        // Check if agency or route is none. If so, send error.
        let agency = try!(self.agency.ok_or(Error::BuildCommandError));
        let route = try!(self.route.ok_or(Error::BuildCommandError));
        let stop = try!(self.stop.ok_or(Error::BuildCommandError));

        // Request and get back
        let res = try!(Request::new()
            .command(Command::Predictions)
            .agency(agency)
            .route(route)
            .stop(stop)
            .send());

        // Parse xml into predictions struct
        Self::from_xml(res)
    }

    fn from_xml<R: Read>(input: R) -> ::Result<Predictions> {
        // initializing vars for Predictions attributes
        let mut agency_title = None ;
        let mut route_tag = None ;
        let mut route_code = None ;
        let mut route_title = None ;
        let mut stop_title = None ;
        let mut dir_title_because_no_predictions = None ;

        // Vec for collecting direction and messages
        let mut directions = vec![];
        let mut messages = vec![];

        let mut parser = EventReader::new(input);

        loop {
            match parser.next() {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                    let name = name.borrow().local_name;

                    if name == "body" { continue };

                    if name == "predictions" {

                        for attribute in attributes {
                            let attribute = attribute.borrow();
                            let name = attribute.name.local_name;
                            let value = attribute.value;

                            match name {
                                "agencyTitle" => agency_title = Some(value.to_owned()),
                                "routeTag" => route_tag = Some(value.to_owned()),
                                "routeCode" => route_code = Some(value.to_owned()),
                                "routeTitle" => route_title = Some(value.to_owned()),
                                "stopTitle" => stop_title = Some(value.to_owned()),
                                "dirTitleBecauseNoPredictions" =>
                                    dir_title_because_no_predictions = Some(value.to_owned()),
                                _ => (),
                            };
                        }
                    } else if name == "direction" {
                        try!(add_direction_to_directions(&mut parser,
                                                         attributes,
                                                         &mut directions,
                                                         ));
                    } else if name == "message" {
                        try!(add_message_to_messages(attributes,
                                                     &mut messages
                                                     ));
                    }
                },
                Ok(XmlEvent::EndDocument) => break,
                Ok(_) => continue,
                Err(_) => break, // Later cover Err
            }
        }
        Ok(Predictions {
            agency_title: try!(agency_title.ok_or(Error::ParseError)),
            route_tag: try!(route_tag.ok_or(Error::ParseError)),
            route_code: route_code,
            route_title: try!(route_title.ok_or(Error::ParseError)),
            stop_title: try!(stop_title.ok_or(Error::ParseError)),
            dir_title_because_no_predictions:
                dir_title_because_no_predictions,
            directions: directions,
            messages: messages,
        })
    }
}

// Components of Predictions
// ===============================================================

/// Used for parsing RouteList Nextbus response.

#[derive(Debug, PartialEq)]
pub struct Direction {
    title: String,
    predictions: Vec<Prediction>,
}

impl Direction {
    pub fn new(title: String, predictions: Vec<Prediction>) -> Self {
        Direction {
            title: title,
            predictions: predictions,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Prediction {
    seconds: usize,
    minutes: usize,
    epoch_time: usize,
    is_departure: bool,
    block: String,
    dir_tag: String,
    trip_tag: Option<String>,
    branch: Option<String>, // only for Toronto TTC agency
    affected_by_layover: Option<bool>,
    is_schedule_based: Option<bool>, // Only exists when true
    delayed: Option<bool>, // only for certain agencies
}

impl Prediction {
    pub fn new(seconds: usize,
               minutes: usize,
               epoch_time: usize,
               is_departure: bool,
               block: String,
               dir_tag: String,
               trip_tag: Option<String>,
               branch: Option<String>,
               affected_by_layover: Option<bool>,
               is_schedule_based: Option<bool>,
               delayed: Option<bool>) -> Self {
        Prediction {
            seconds: seconds,
            minutes: minutes,
            epoch_time: epoch_time,
            is_departure: is_departure,
            block: block,
            dir_tag: dir_tag,
            trip_tag: trip_tag,
            branch: branch,
            affected_by_layover: affected_by_layover,
            is_schedule_based: is_schedule_based,
            delayed: delayed,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    text: String,
    priority: Option<String>,
}

impl Message {
    pub fn new(text: String, priority: Option<String>) -> Self {
        Message {
            text: text,
            priority: priority
        }
    }
}

// ===============================================================
// Helpers for parsing
// ===============================================================

// Adding a direction
// ===============================================================

fn add_direction_to_directions<R: Read>(mut parser: &mut EventReader<R>,
                                        attributes: Vec<OwnedAttribute>,
                                        mut directions: &mut Vec<Direction>) -> ::Result<()> {
    let mut title = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "title" => title = Some(value.to_owned()),
            _ => (),
        };
    }

    // Now get predictions
    let mut predictions = Vec::new();
    try!(parse_prediction_elements(&mut parser, &mut predictions));

    // Set Direction
    directions.push(Direction {
        title: try!(title.ok_or(Error::ParseError)),
        predictions: predictions,
    });

    Ok(())
}

// Parsing a prediction
// ===============================================================

fn parse_prediction_elements<R: Read>(mut parser: &mut EventReader<R>,
                                      mut predictions: &mut Vec<Prediction>) -> ::Result<()> {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                let name = name.borrow().local_name;
                if name == "prediction" {
                    try!(add_prediction_to_predictions(attributes, &mut predictions));
                }
            },
            Ok(XmlEvent::EndElement {name, ..}) => {
                if name.borrow().local_name == "direction" {
                    break;
                }
            },
            Err(_) => break,
            _ => continue,
        }
    }
    Ok(())
}

fn add_prediction_to_predictions(attributes: Vec<OwnedAttribute>,
                                 mut predictions: &mut Vec<Prediction>) -> ::Result<()> {
    let mut seconds: Option<usize> = None;
    let mut minutes: Option<usize> = None;
    let mut epoch_time: Option<usize> = None;
    let mut is_departure: Option<bool> = None;
    let mut block: Option<String> = None;
    let mut dir_tag: Option<String> = None;
    let mut trip_tag: Option<String> = None;
    let mut branch: Option<String> = None;
    let mut affected_by_layover: Option<bool> = None;
    let mut is_schedule_based: Option<bool> = None;
    let mut delayed: Option<bool> = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "seconds" => seconds = Some(value.parse().unwrap()),
            "minutes" => minutes = Some(value.parse().unwrap()),
            "epochTime" => epoch_time = Some(value.parse().unwrap()),
            "isDeparture" => is_departure = Some(value.parse().unwrap()),
            "block" => block = Some(value.to_owned()),
            "dirTag" => dir_tag = Some(value.to_owned()),
            "tripTag" => trip_tag = Some(value.to_owned()),
            "branch" => branch = Some(value.to_owned()),
            "affecteByLayover" => affected_by_layover = Some(value.parse().unwrap()),
            "isScheduleBased" => is_schedule_based = Some(value.parse().unwrap()),
            "delayed" => delayed = Some(value.parse().unwrap()),
            _ => (),
        }
    }
    predictions.push(Prediction {
        seconds: try!(seconds.ok_or(Error::ParseError)),
        minutes: try!(minutes.ok_or(Error::ParseError)),
        epoch_time: try!(epoch_time.ok_or(Error::ParseError)),
        is_departure: try!(is_departure.ok_or(Error::ParseError)),
        block: try!(block.ok_or(Error::ParseError)),
        dir_tag: try!(dir_tag.ok_or(Error::ParseError)),
        trip_tag: trip_tag,
        branch: branch,
        affected_by_layover: affected_by_layover,
        is_schedule_based: is_schedule_based,
        delayed: delayed,
    });
    Ok(())
}
fn add_message_to_messages(attributes: Vec<OwnedAttribute>,
                                 mut messages: &mut Vec<Message>) -> ::Result<()> {
    let mut text: Option<String> = None;
    let mut priority: Option<String> = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "text" => text = Some(value.to_owned()),
            "priority" => priority = Some(value.to_owned()),
            _ => (),
        }
    }
    messages.push(Message {
        text: try!(text.ok_or(Error::ParseError)),
        priority: priority,
    });
    Ok(())
}

// Tests
// ===============================================================

#[cfg(test)]
mod test {
//    use std::io::Cursor;
    use super::*;

//    #[test]
//    fn parse_good_xml() {
//        let buffer = Cursor::new(GOOD_ROUTE_XML);
//        let routes = RouteListBuilder::from_xml(buffer).unwrap();
//
//        let test_boston = Route::new("boston".to_owned(),
//                                     "Boston Daytime".to_owned(),
//                                     None);
//        let test_kendchar = Route::new("kendchar".to_owned(),
//                                       "Kendall to Charles Park".to_owned(),
//                                       Some("Kendall-Charles".to_owned()));
//        let test_routes = RouteList::new(vec![test_boston, test_kendchar]);
//
//        assert_eq!(routes, test_routes);
//    }
//
//    #[test]
//    #[should_panic]
//    fn parse_bad_xml_missing_tag() {
//        let buffer = Cursor::new(MISSING_TAG_ROUTE_XML);
//        RouteListBuilder::from_xml(buffer).unwrap();
//    }
//
//    #[test]
//    #[should_panic]
//    fn parse_bad_xml_missing_title() {
//        let buffer = Cursor::new(MISSING_TITLE_ROUTE_XML);
//        RouteListBuilder::from_xml(buffer).unwrap();
//    }
//
//    // Should simply skip over any extra attributes, no panic.
//    #[test]
//    fn parse_bad_xml_extra_attribute() {
//        let buffer = Cursor::new(EXTRA_ATTRIBUTE_ROUTE_XML);
//        let routes = RouteListBuilder::from_xml(buffer).unwrap();
//
//        let test_kendchar = Route::new("kendchar".to_owned(),
//                                       "Kendall to Charles Park".to_owned(),
//                                       Some("Kendall-Charles".to_owned()));
//        let test_routes = RouteList::new(vec![test_kendchar]);
//
//        assert_eq!(routes, test_routes);
//    }

    #[test]
//    #[ignore]
    fn should_get_predictions() {
        let predictions = PredictionsBuilder::new()
            .agency("mit")
            .route("saferidecampshut")
            .stop("mass84_d")
            .get()
            .unwrap();
        println!("{:?}", predictions);
        assert!(false);
    }
}


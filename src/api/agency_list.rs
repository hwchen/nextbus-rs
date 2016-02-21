//! Module for handling agency functions

//TODO: implement iterator for AgencyList
//TODO: unit testing for parser

use error::Error;
use request::{Command, Request};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;

/// List of Agencies. Maps directly from Nextbus
/// response. Contains vec of routes.
#[derive(Debug)]
pub struct AgencyList(Vec<Agency>);

impl AgencyList {
    pub fn new(agencies: Vec<Agency>) -> Self {
        AgencyList(agencies)
    }
}

/// Builds a request for AgencyList.
/// Since there's no config, it's empty,
/// but it's a consistent API with other commands.
pub struct AgencyListBuilder;

impl AgencyListBuilder {

    pub fn new() -> Self {
        AgencyListBuilder
    }

    pub fn get(self) -> ::Result<AgencyList> {

        // Make the request
        let mut res = try!(Request::new()
            .command(Command::AgencyList)
            .send());

        // Parse xml into agency list struct
        xml_to_agency_list(res)
    }
}

#[derive(Debug)]
pub struct Agency {
    tag: String,
    title: String,
    short_title: Option<String>,
    region_title: String,
}

fn xml_to_agency_list<R: Read>(input: R) -> ::Result<AgencyList> {
    // Vec for collecting agencies
    let mut agencies = vec![];

    let mut parser = EventReader::new(input);

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {name: name, attributes: attributes, ..}) => {
                if name.borrow().local_name == "agency" {
                    let mut tag = None ;
                    let mut title = None;
                    let mut short_title = None;
                    let mut region_title = None;

                    for attribute in attributes {
                        let attribute = attribute.borrow();
                        let name = attribute.name.local_name;
                        let value = attribute.value;

                        match name {
                            "tag" => tag = Some(value.to_owned()),
                            "title" => title = Some(value.to_owned()),
                            "shortTitle" => short_title = Some(value.to_owned()),
                            "regionTitle" =>  region_title = Some(value.to_owned()),
                            _ => (),
                        };
                    }

                    agencies.push(Agency{
                        tag: try!(tag.ok_or(Error::ParseError)),
                        title: try!(title.ok_or(Error::ParseError)),
                        short_title: short_title,
                        region_title: try!(region_title.ok_or(Error::ParseError)),

                    });
                }
            },
            _ => (),
        }
    }

    Ok(AgencyList(agencies))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
//    #[ignore]
    fn should_get_agencies() {
        let agencies = AgencyListBuilder::new().get().unwrap();
        let AgencyList(agencies) = agencies;
        for agency in agencies {
            println!("{:?}\n", agency);
        }
        assert!(false);
    }
}


//! Module for handling agency functions

use error::Error;
use request::{Command, Request};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

/// List of Agencies. Maps directly from Nextbus
/// response. Contains vec of routes.
#[derive(Debug, PartialEq)]
pub struct AgencyList(Vec<Agency>);

impl AgencyList {
    pub fn new(agencies: Vec<Agency>) -> Self {
        AgencyList(agencies)
    }
}

impl IntoIterator for AgencyList {
    type Item = Agency;
    type IntoIter = ::std::vec::IntoIter<Agency>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a AgencyList {
    type Item = &'a Agency;
    type IntoIter = ::std::slice::Iter<'a, Agency>;

    fn into_iter(self) -> Self::IntoIter {
        let &AgencyList(ref agencies) = self;
        agencies.iter()
    }
}

// Builder
// ===============================================================

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
        let res = try!(Request::new()
            .command(Command::AgencyList)
            .send());

        // Parse xml into agency list struct
        Self::from_xml(res)
    }

    fn from_xml<R: Read>(input: R) -> ::Result<AgencyList> {
        // Vec for collecting agencies
        let mut agencies = vec![];

        let parser = EventReader::new(input);

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
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
}

// Components of AgencyList
// ===============================================================

#[derive(Debug, PartialEq)]
pub struct Agency {
    tag: String,
    title: String,
    short_title: Option<String>,
    region_title: String,
}

impl Agency {
    pub fn new(tag: String,
               title: String,
               short_title: Option<String>,
               region_title: String) -> Self {
        Agency {
            tag: tag,
            title: title,
            short_title: short_title,
            region_title: region_title,
        }
    }
}

// Tests
// ===============================================================

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    // test xml
    const GOOD_AGENCY_XML: &'static str = "
        <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright agencies listed below and NextBus Inc 2016.\">
        <agency tag=\"jhu-apl\" title=\"APL\" regionTitle=\"Maryland\"/>
        <agency tag=\"camarillo\" title=\"Camarillo Area (CAT)\" shortTitle=\"Camarillo (CAT)\" regionTitle=\"California-Southern\"/>
        </body>";

    // Missing tag, title, regionTitle. Extra attribute
    const MISSING_TAG_AGENCY_XML: &'static str = "
        <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright agencies listed below and NextBus Inc 2016.\">
        <agency title=\"Camarillo Area (CAT)\" shortTitle=\"Camarillo (CAT)\" regionTitle=\"California-Southern\"/>
        </body>";

    const MISSING_TITLE_AGENCY_XML: &'static str = "
        <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright agencies listed below and NextBus Inc 2016.\">
        <agency tag=\"camarillo\" shortTitle=\"Camarillo (CAT)\" regionTitle=\"California-Southern\"/>
        </body>";

    const MISSING_REGION_TITLE_AGENCY_XML: &'static str = "
        <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright agencies listed below and NextBus Inc 2016.\">
        <agency tag=\"camarillo\" title=\"Camarillo Area (CAT)\" shortTitle=\"Camarillo (CAT)\"/>
        </body>";

    const EXTRA_ATTRIBUTE_AGENCY_XML: &'static str = "
        <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright agencies listed below and NextBus Inc 2016.\">
        <agency extra=\"extra\" tag=\"camarillo\" title=\"Camarillo Area (CAT)\" shortTitle=\"Camarillo (CAT)\" regionTitle=\"California-Southern\"/>
        </body>";

    #[test]
    fn parse_good_xml() {
        let buffer = Cursor::new(GOOD_AGENCY_XML);
        let agencies = AgencyListBuilder::from_xml(buffer).unwrap();

        let test_jhu = Agency::new("jhu-apl".to_owned(),
                                   "APL".to_owned(),
                                   None,
                                   "Maryland".to_owned());
        let test_camarillo = Agency::new("camarillo".to_owned(),
                                         "Camarillo Area (CAT)".to_owned(),
                                         Some("Camarillo (CAT)".to_owned()),
                                         "California-Southern".to_owned());
        let test_agencies = AgencyList::new(vec![test_jhu, test_camarillo]);

        assert_eq!(agencies, test_agencies);
    }

    #[test]
    #[should_panic]
    fn parse_bad_xml_missing_tag() {
        let buffer = Cursor::new(MISSING_TAG_AGENCY_XML);
        AgencyListBuilder::from_xml(buffer).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_bad_xml_missing_title() {
        let buffer = Cursor::new(MISSING_TITLE_AGENCY_XML);
        AgencyListBuilder::from_xml(buffer).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_bad_xml_missing_region_title() {
        let buffer = Cursor::new(MISSING_REGION_TITLE_AGENCY_XML);
        AgencyListBuilder::from_xml(buffer).unwrap();
    }

    // Should simply skip over any extra attributes, no panic.
    #[test]
    fn parse_bad_xml_extra_attribute() {
        let buffer = Cursor::new(EXTRA_ATTRIBUTE_AGENCY_XML);
        let agencies = AgencyListBuilder::from_xml(buffer).unwrap();

        let test_camarillo = Agency::new("camarillo".to_owned(),
                                         "Camarillo Area (CAT)".to_owned(),
                                         Some("Camarillo (CAT)".to_owned()),
                                         "California-Southern".to_owned());
        let test_agencies = AgencyList::new(vec![test_camarillo]);

        assert_eq!(agencies, test_agencies);
    }

    #[test]
    #[ignore]
    fn should_get_agencies() {
        let agencies = AgencyListBuilder::new().get().unwrap();
        for agency in agencies {
            println!("agency");
            println!("{:?}\n", agency);
        }
        assert!(false);
    }
}


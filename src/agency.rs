//! Module for handling agency functions

use ::{Command, Request};
use kuchiki;
use kuchiki::traits::*;
use std::io::Read;

/// List of Agencies. Maps directly from Nextbus
/// response. Contains vec of stub information
/// for each route.
#[derive(Debug)]
pub struct AgencyList(Vec<AgencyStub>);

impl AgencyList {
    // Can't seem to do without allocations :(
    pub fn new() -> ::Result<AgencyList> {
        let mut res = try!(Request::new()
            .command(Command::AgencyList)
            .send());
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let xml = kuchiki::parse_html().one(body);
        let mut agencies = vec![];
        for agencies_xml in xml.descendants().select("agency") {
            for agency in agencies_xml {

                agencies.push(AgencyStub{
                    tag: agency.attributes.borrow()
                        .get("tag").to_owned().map(|s| s.to_owned()).unwrap(),

                    title: agency.attributes.borrow()
                        .get("title").map(|s| s.to_owned()).unwrap(),

                    short_title: agency.attributes.borrow()
                        .get("shorttitle").map(|s| s.to_owned()),

                    region_title: agency.attributes.borrow()
                        .get("regiontitle").map(|s| s.to_owned()),
                })
            }
        }
        Ok(AgencyList(agencies))
    }
}

/// Used for parsing AgencyList Nextbus response.
/// Contains a stub of route data.
#[derive(Debug)]
pub struct AgencyStub {
    tag: String,
    title: String,
    short_title: Option<String>,
    region_title: Option<String>,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    //#[ignore]
    fn should_get_agencies() {
        let agencies = AgencyList::new().unwrap();
        println!("{:?}", agencies);
        assert!(false);
    }
}

// Maybe this one is too complicated
///// Entry point for working with Agencies.
//#[derive(Debug)]
//pub struct Agency<'a> {
//    tag: String,
//    title: String,
//    short_title: Option<String>,
//    region_title: Option<String>,
//    routes: HashMap<String, Route>,
//}



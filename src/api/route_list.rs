//! Next Bus Route List Command

use error::Error;
use request::{Command, Request};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};


/// List of routes. Maps directly from Nextbus response.
/// Contains vec of stub information for each route.
#[derive(Debug, PartialEq)]
pub struct RouteList(Vec<Route>);

impl RouteList {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteList(routes)
    }
}

impl IntoIterator for RouteList {
    type Item = Route;
    type IntoIter = ::std::vec::IntoIter<Route>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a RouteList {
    type Item = &'a Route;
    type IntoIter = ::std::slice::Iter<'a, Route>;

    fn into_iter(self) -> Self::IntoIter {
        let &RouteList(ref routes) = self;
        routes.iter()
    }
}

// Builder
// ===============================================================

pub struct RouteListBuilder<'a> {
    agency: Option<&'a str>,
}

impl<'a> RouteListBuilder<'a> {
    pub fn new() -> Self {
        RouteListBuilder { agency: None }
    }

    /// Builder to set agency
    pub fn agency(&mut self, agency: &'a str) -> &Self {
        self.agency = Some(agency);
        self
    }

    pub fn get(&self) -> ::Result<RouteList> {
        // Check if agency is none. If so, send error.
        let agency = try!(self.agency.ok_or(Error::BuildCommandError));

        // Request and get back
        let res = try!(Request::new()
            .command(Command::RouteList)
            .agency(agency)
            .send());

        // Parse xml into route list struct
        Self::from_xml(res)
    }

    fn from_xml<R: Read>(input: R) -> ::Result<RouteList> {
        // Vec for collecting routes
        let mut routes = vec![];

        let parser = EventReader::new(input);

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                    if name.borrow().local_name == "route" {
                        let mut tag = None ;
                        let mut title = None;
                        let mut short_title = None;

                        for attribute in attributes {
                            let attribute = attribute.borrow();
                            let name = attribute.name.local_name;
                            let value = attribute.value;

                            match name {
                                "tag" => tag = Some(value.to_owned()),
                                "title" => title = Some(value.to_owned()),
                                "shortTitle" => short_title = Some(value.to_owned()),
                                _ => (),
                            };
                        }

                        routes.push(Route{
                            tag: try!(tag.ok_or(Error::ParseError)),
                            title: try!(title.ok_or(Error::ParseError)),
                            short_title: short_title,
                        });
                    }
                },
                _ => (),
            }
        }

        Ok(RouteList(routes))
    }
}

// Components of RouteList
// ===============================================================

/// Used for parsing RouteList Nextbus response.
#[derive(Debug, PartialEq)]
pub struct Route {
    tag: String,
    title: String,
    short_title: Option<String>,
}

impl Route {
    pub fn new(tag: String, title: String, short_title: Option<String>) -> Self {
        Route {
            tag: tag,
            title: title,
            short_title: short_title,
        }
    }
}

// Tests
// ===============================================================

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

const GOOD_ROUTE_XML: &'static str = "
    <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright Massachusetts Institute of Technology 2016.\">
        <route tag=\"boston\" title=\"Boston Daytime\"/>
        <route tag=\"kendchar\" title=\"Kendall to Charles Park\" shortTitle=\"Kendall-Charles\"/>
        </body>";

// Bad Route XML: missing tag, missing title, extra attribute;
const MISSING_TAG_ROUTE_XML: &'static str = "
    <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright Massachusetts Institute of Technology 2016.\">
        <route title=\"Kendall to Charles Park\" shortTitle=\"Kendall-Charles\"/>
        </body>";

const MISSING_TITLE_ROUTE_XML: &'static str = "
    <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright Massachusetts Institute of Technology 2016.\">
        <route tag=\"kendchar\" shortTitle=\"Kendall-Charles\"/>
        </body>";

const EXTRA_ATTRIBUTE_ROUTE_XML: &'static str = "
    <?xml version=\"1.0\" encoding=\"utf-8\" ?> 
        <body copyright=\"All data copyright Massachusetts Institute of Technology 2016.\">
        <route extra=\"extra\" tag=\"kendchar\"
            title=\"Kendall to Charles Park\" shortTitle=\"Kendall-Charles\"/>
        </body>";

    #[test]
    fn parse_good_xml() {
        let buffer = Cursor::new(GOOD_ROUTE_XML);
        let routes = RouteListBuilder::from_xml(buffer).unwrap();

        let test_boston = Route::new("boston".to_owned(),
                                     "Boston Daytime".to_owned(),
                                     None);
        let test_kendchar = Route::new("kendchar".to_owned(),
                                       "Kendall to Charles Park".to_owned(),
                                       Some("Kendall-Charles".to_owned()));
        let test_routes = RouteList::new(vec![test_boston, test_kendchar]);

        assert_eq!(routes, test_routes);
    }

    #[test]
    #[should_panic]
    fn parse_bad_xml_missing_tag() {
        let buffer = Cursor::new(MISSING_TAG_ROUTE_XML);
        RouteListBuilder::from_xml(buffer).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_bad_xml_missing_title() {
        let buffer = Cursor::new(MISSING_TITLE_ROUTE_XML);
        RouteListBuilder::from_xml(buffer).unwrap();
    }

    // Should simply skip over any extra attributes, no panic.
    #[test]
    fn parse_bad_xml_extra_attribute() {
        let buffer = Cursor::new(EXTRA_ATTRIBUTE_ROUTE_XML);
        let routes = RouteListBuilder::from_xml(buffer).unwrap();

        let test_kendchar = Route::new("kendchar".to_owned(),
                                       "Kendall to Charles Park".to_owned(),
                                       Some("Kendall-Charles".to_owned()));
        let test_routes = RouteList::new(vec![test_kendchar]);

        assert_eq!(routes, test_routes);
    }

    #[test]
    #[ignore]
    fn should_get_routes() {
        let routes = RouteListBuilder::new()
            .agency("mit")
            .get()
            .unwrap();
        for route in routes {
            println!("route");
            println!("{:?}\n", route);
        }
        assert!(false);
    }
}


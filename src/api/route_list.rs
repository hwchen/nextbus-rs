//! Next Bus Route List Command
//!

use ::{Command, Request};
use ::Error;
use kuchiki;
use kuchiki::traits::*;
use std::io::Read;

/// List of routes. Maps directly from Nextbus response.
/// Contains vec of stub information for each route.
#[derive(Debug)]
pub struct RouteList(Vec<Route>);

impl RouteList {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteList(routes)
    }
}

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
        let agency = try!(self.agency.ok_or(Error::BuildCommandError));

        let mut res = try!(Request::new()
            .command(Command::RouteList)
            .agency(agency)
            .send());
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{}", body);
        let xml = kuchiki::parse_html().one(body);
        let mut routes = vec![];
        for routes_xml in xml.descendants().select("route") {
            for route in routes_xml {

                routes.push(Route{
                    tag: route.attributes.borrow()
                        .get("tag").map(|s| s.to_owned()).unwrap(),

                    title: route.attributes.borrow()
                        .get("title").map(|s| s.to_owned()).unwrap(),
                })
            }
        }
        Ok(RouteList(routes))
    }
}

/// Used for parsing RouteList Nextbus response.
/// Contains a stub of route data.
#[derive(Debug)]
pub struct Route {
    tag: String,
    title: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn should_get_routes() {
        let routes = RouteListBuilder::new()
            .agency("mit")
            .get()
            .unwrap();
        println!("{:?}", routes);
        assert!(false);
    }
}


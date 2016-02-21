//! Next Bus Route List Command
//!

use request::{Command, Request};
use ::Error;
use std::io::Read;

#[derive(Debug)]
pub struct RouteConfig(Vec<Route>);

impl RouteConfig {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteConfig(routes)
    }
}

pub struct RouteConfigBuilder<'a> {
    agency: Option<&'a str>,
    route: Option<&'a str>,
}

impl<'a> RouteConfigBuilder<'a> {
    pub fn new() -> Self {
        RouteConfigBuilder {
            agency: None,
            route: None,
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

    pub fn get(&self) -> ::Result<RouteConfig> {
        // Check if agency or route is none. If so, send error.
        let agency = try!(self.agency.ok_or(Error::BuildCommandError));

        // Request. Allow one route or no route param (returns all routes)
        let mut res = match self.route {
            Some(route) => {
                try!(Request::new()
                    .command(Command::RouteConfig)
                    .agency(agency)
                    .route(route)
                    .send())
            },
            None => {
                try!(Request::new()
                    .command(Command::RouteConfig)
                    .agency(agency)
                    .send())
            }
        };

        // Read response
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("Body: {}", body);
        // remove \n, it's mucking up the parsing
        let body_cleaned = body.replace("\n", "");
        //println!("Body Cleaned: {:?}", body_cleaned);

        // TODO: parse xml into structs.

        Ok(RouteConfig(vec![]))
    }
}

// Components of RouteConfig

#[derive(Debug)]
pub struct Route {
    tag: String,
    title: String,
    color: String,
    opposite_color: String,
    lat_min: f32,
    lat_max: f32,
    lon_min: f32,
    lon_max: f32,
    directions: Vec<Direction>,
    paths: Vec<Path>,
}

/// A stop along a Route
#[derive(Debug)]
pub struct Stop {
    tag: String,
    title: String,
    lat: String,
    long: String,
    stop_id: String,
}

/// An itinerary along a route
#[derive(Debug)]
pub struct Direction {
    tag: String,
    title: String,
    name: String,
    use_for_ui: bool,
    // stops is an ordered vector of stop tags
    // References are kept because stops will
    // be referenced many times among the many
    // directions
    stops: Vec<String>,
}

/// The coordinates tracing a Route
#[derive(Debug)]
pub struct Path(Vec<Point>);

/// A coordinate in a Path
#[derive(Debug)]
pub struct Point {
    lat: f32,
    lon: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn should_get_route_config() {
        let mut routes = RouteConfigBuilder::new()
            .agency("mit")
            .route("saferidecambwest")
            .get()
            .unwrap();
        //println!("{:?}", routes);
        assert!(false);
    }
}


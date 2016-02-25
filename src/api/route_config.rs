//! Next Bus Route List Command
//!

use error::Error;
use request::{Command, Request};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub struct RouteConfig(Vec<Route>);

impl RouteConfig {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteConfig(routes)
    }
}

impl IntoIterator for RouteConfig {
    type Item = Route;
    type IntoIter = ::std::vec::IntoIter<Route>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
        let res = match self.route {
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

        Self::from_xml(res)
    }

    fn from_xml<R: Read>(input: R) -> ::Result<RouteConfig> {
        // Vec for collecting routes
        let mut routes = vec![];

        let mut parser = EventReader::new(input);

        loop {
            // This should match the route tag
            match parser.next() {
                Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                    if name.borrow().local_name == "body" { continue };

                    if name.borrow().local_name == "route" {
                        try!(add_route_to_routes(&mut parser, attributes, &mut routes));
                    }
                },
                Ok(XmlEvent::EndDocument) => break,
                Ok(_) => continue,
                Err(_) => break, // Later cover Err
            }
        }

        Ok(RouteConfig(routes))
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
    stops: Vec<Stop>,
    directions: Vec<Direction>,
    paths: Vec<Path>,
}

/// A stop along a Route
#[derive(Debug)]
pub struct Stop {
    tag: String,
    title: String,
    lat: String,
    lon: String,
    short_title: Option<String>,
    stop_id: Option<String>,
}

#[derive(Debug)]
pub struct StopStub {
    tag: String,
}

/// An itinerary along a route
#[derive(Debug)]
pub struct Direction {
    tag: String,
    title: String,
    name: String,
    use_for_ui: bool,
    stops: Vec<StopStub>,
}

/// The coordinates tracing a Route
#[derive(Debug)]
pub struct Path {
    tag: String,
    points: Vec<Point>,
}

/// A coordinate in a Path
#[derive(Debug)]
pub struct Point {
    lat: f32,
    lon: f32,
}

// ===============================================================
// Helpers for parsing
// ===============================================================

// Parsing a Route
// ===============================================================

fn add_route_to_routes<R: Read>(mut parser: &mut EventReader<R>,
                                attributes: Vec<OwnedAttribute>,
                                mut routes: &mut Vec<Route>) -> ::Result<()> {
    let mut tag = None ;
    let mut title = None;
    let mut color = None;
    let mut opposite_color = None;
    let mut lat_min: Option<f32> = None;
    let mut lat_max: Option<f32> = None;
    let mut lon_min: Option<f32> = None;
    let mut lon_max: Option<f32> = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "tag" => tag = Some(value.to_owned()),
            "title" => title = Some(value.to_owned()),
            "color" => color = Some(value.to_owned()),
            "oppositeColor" => opposite_color = Some(value.to_owned()),
            "latMin" => lat_min = Some(value.parse().unwrap()),
            "latMax" => lat_max = Some(value.parse().unwrap()),
            "lonMin" => lon_min = Some(value.parse().unwrap()),
            "lonMax" => lon_max = Some(value.parse().unwrap()),
            _ => (),
        };
    }

    // Now get stops for this route
    let mut stops = Vec::new();
    let mut directions = Vec::new();
    let mut paths = Vec::new();
    try!(parse_route_elements(&mut parser, &mut stops, &mut directions, &mut paths));

    // Set Route
    routes.push(Route{
        tag: try!(tag.ok_or(Error::ParseError)),
        title: try!(title.ok_or(Error::ParseError)),
        color: try!(color.ok_or(Error::ParseError)),
        opposite_color: try!(opposite_color.ok_or(Error::ParseError)),
        lat_min: try!(lat_min.ok_or(Error::ParseError)),
        lat_max: try!(lat_max.ok_or(Error::ParseError)),
        lon_min: try!(lon_min.ok_or(Error::ParseError)),
        lon_max: try!(lon_max.ok_or(Error::ParseError)),
        stops: stops,
        directions: directions,
        paths: paths,
    });

    Ok(())
}

// Parsing route elements
//
// The reason all the elements (stops, directions, paths) are all
// put together is that the parser doesn't have lookahead and there
// is no end element for stops. So stops won't know when to end until
// it hits a direction start tag.
// ===============================================================
fn parse_route_elements<R: Read>(mut parser: &mut EventReader<R>,
                        mut stops: &mut Vec<Stop>,
                        mut directions: &mut Vec<Direction>,
                        mut paths: &mut Vec<Path>) -> ::Result<()> {
    // The logic for matching:
    // - continue if it's a start element that's a stop
    // - break if it's a start element that's not a stop (and move on)
    // - break on error
    // - break when hit end route
    // - continue if it's any other event (whitespace, etc.)
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                let name = name.borrow().local_name;
                if name == "stop" {
                    // Stops are not nested inside a stop start/end,
                    // so can directly add them to the vec here
                    try!(add_stop_to_stops(attributes, &mut stops));
                } else if name == "direction" {
                    // Direction contains a start/end tag, so the stops
                    // in direction are nested, so can't add here directly.
                    // Need to parse all Direction struct, like parsing a 
                    // route.
                    try!(parse_direction(&mut parser, attributes, &mut directions));
                } else if name == "path" {
                    // path element has no attributes
                    // tag is contained in inner element
                    try!(parse_path(&mut parser, &mut paths));
                } else { // This shouldn't be needed.
                    break;
                }
            },
            Ok(XmlEvent::EndElement {name, ..}) => {
                if name.borrow().local_name == "route" {
                    break;
                }
            },
            Err(_) => break,
            _ => continue,
        }
    }
    Ok(())
}

fn add_stop_to_stops(attributes: Vec<OwnedAttribute>,
                     stops: &mut Vec<Stop>) -> ::Result<()> {
    let mut tag = None ;
    let mut title = None;
    let mut lat = None;
    let mut lon = None;
    let mut short_title = None;
    let mut stop_id = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "tag" => tag = Some(value.to_owned()),
            "title" => title = Some(value.to_owned()),
            "lat" => lat = Some(value.parse().unwrap()),
            "lon" => lon = Some(value.parse().unwrap()),
            "shortTitle" => short_title = Some(value.to_owned()),
            "stopId" => stop_id = Some(value.to_owned()),
            _ => (),
        }
    }
    stops.push(Stop{
        tag: try!(tag.ok_or(Error::ParseError)),
        title: try!(title.ok_or(Error::ParseError)),
        lat: try!(lat.ok_or(Error::ParseError)),
        lon: try!(lon.ok_or(Error::ParseError)),
        short_title: short_title,
        stop_id: stop_id,
    });

    Ok(())
}

// Parsing Direction
// ===============================================================
fn parse_direction<R: Read>(parser: &mut EventReader<R>,
                             attributes: Vec<OwnedAttribute>,
                             mut directions: &mut Vec<Direction>) -> ::Result<()> {
    // put attributes of direction first
    let mut tag = None;
    let mut title = None;
    let mut name = None;
    let mut use_for_ui: Option<bool> = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let attr_name = attribute.name.local_name;
        let value = attribute.value;

        match attr_name {
            "tag" => tag = Some(value.to_owned()),
            "title" => title = Some(value.to_owned()),
            "name" => name = Some(value.to_owned()),
            "useForUI" => use_for_ui = Some(value.parse().unwrap()),
            _ => (),
        };
    }

    // Now iterate through inner Stop tags
    // The logic for matching:
    // - continue if it's a start element that's a stop
    // - break if it's an end element for direction (The next one should be matching)
    // - break on error
    // - continue if it's any other event (whitespace, etc.)
    let mut stops = Vec::new();
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                if name.borrow().local_name == "stop" {
                    //println!("{:?}", name);
                    //println!("{:?}", attributes);
                    try!(add_stop_to_direction(attributes, &mut stops));
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
    directions.push(Direction {
        tag: try!(tag.ok_or(Error::ParseError)),
        title: try!(title.ok_or(Error::ParseError)),
        name: try!(name.ok_or(Error::ParseError)),
        use_for_ui: try!(use_for_ui.ok_or(Error::ParseError)),
        stops: stops,
    });
    Ok(())
}

fn add_stop_to_direction(attributes: Vec<OwnedAttribute>,
                         stops: &mut Vec<StopStub>) -> ::Result<()> {
    let mut tag = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "tag" => tag = Some(value.to_owned()),
            _ => (),
        };
    }
    stops.push(StopStub {
        tag: try!(tag.ok_or(Error::ParseError)),
    });
    Ok(())
}

// Parsing Path
// ===============================================================
fn parse_path<R: Read>(parser: &mut EventReader<R>,
                       mut paths: &mut Vec<Path>) -> ::Result<()> {
    // put attributes of direction first
    let mut tag = None;


    // Now iterate through inner 
    // The logic for matching:
    // - continue if it's a start element that's a tag or point
    // - break if it's an end element for path (The next one should be matching)
    // - break on error
    // - continue if it's any other event (whitespace, etc.)
    let mut points = Vec::new();
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                let name = name.borrow().local_name;

                if name == "tag" {
                    for attribute in attributes {
                        let attribute = attribute.borrow();
                        let attr_name = attribute.name.local_name;
                        let value = attribute.value;

                        match attr_name {
                            "id" => tag = Some(value.to_owned()),
                            _ => (),
                        };
                    }

                } else if name == "point" {
                    try!(add_point_to_path(attributes, &mut points));
                }
            },
            Ok(XmlEvent::EndElement {name, ..}) => {
                if name.borrow().local_name == "path" {
                    break;
                }
            },
            Err(_) => break,
            _ => continue,
        }
    }
    paths.push(Path {
        tag: try!(tag.ok_or(Error::ParseError)),
        points: points,
    });
    Ok(())
}

fn add_point_to_path(attributes: Vec<OwnedAttribute>,
                     points: &mut Vec<Point>) -> ::Result<()> {
    let mut lat: Option<f32> = None;
    let mut lon: Option<f32> = None;

    for attribute in attributes {
        let attribute = attribute.borrow();
        let name = attribute.name.local_name;
        let value = attribute.value;

        match name {
            "lat" => lat = Some(value.parse().unwrap()),
            "lon" => lon = Some(value.parse().unwrap()),
            _ => (),
        };
    }
    points.push(Point {
        lat: try!(lat.ok_or(Error::ParseError)),
        lon: try!(lon.ok_or(Error::ParseError)),
    });
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
//    #[ignore]
    fn should_get_one_route_config() {
        let routes = RouteConfigBuilder::new()
            .agency("mit")
            .route("saferidecampshut")
            .get()
            .unwrap();
        for route in routes {
            println!("{:?}", route);
            println!("\n");
        }
        assert!(false);
    }

    #[test]
    #[ignore]
    fn should_get_many_route_config() {
        let routes = RouteConfigBuilder::new()
            .agency("mit")
            .get()
            .unwrap();
        for route in routes {
            println!("{:?}", route);
            println!("\n");
        }
        assert!(false);
    }
}


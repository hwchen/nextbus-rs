//! Module for building up Next Bus URL from components
//!
//! Builder pattern is used to create a Request
//! from RequestBuilder
//!
//! The Request is guaranteed to be valid!
//!
//! ## Components
//! - Command
//! - Agency
//! - RouteConfig
//! - Predictions
//! - PredictionsForMultiStops
//! - Schedule
//! - Messages
//! - VehicleLocations
//! 
// TODO: Change API to get rid of add_route and add_stop,
// just use route and stop, and always append instead of replace?

use nb::NEXTBUS_URL;
use hyper::client::Client;
use hyper::client::response::Response;
use hyper::Url;
use std::fmt;
use error::Error;

/// A valid next bus url and query
/// Verified on building (dynamic). Because tuple struct constructors
/// with private fields cannot be invoked.
#[derive(Debug, PartialEq)]
pub struct Request(Url);

impl Request {
    pub fn new(builder: &RequestBuilder) -> ::Result<Self> {
        // Check for invalid query combinations

        match builder.command {
            Some(Command::AgencyList) => {
                if builder.agency != None || builder.routes != None ||
                    builder.stops != None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
            },
            Some(Command::RouteList) => {
                if builder.agency == None || builder.routes != None ||
                    builder.stops != None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
            },
            // Can specify one or no routes
            Some(Command::RouteConfig) => {
                if builder.agency == None ||
                    builder.stops != None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() > 1 { return Err(Error::BuildUrlError) }
                }
            },
            Some(Command::Predictions) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops == None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(Error::BuildUrlError) }
                }
                if let Some(ref stops) = builder.stops {
                    if stops.len() != 1 { return Err(Error::BuildUrlError); }
                }
            },
            // No routes allowed?
            Some(Command::PredictionsForMultiStops) => {
                if builder.agency == None || builder.routes != None ||
                    builder.stops == None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    // Shouldn't be able to happen, but double-check
                    if routes.is_empty() { return Err(Error::BuildUrlError) }
                }
            },
            Some(Command::Schedule) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(Error::BuildUrlError) }
                }
            },
            Some(Command::Messages) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time != None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    // Shouldn't be able to happen, but double-check
                    if routes.is_empty() { return Err(Error::BuildUrlError) }
                }
            },
            Some(Command::VehicleLocations) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time == None {

                    return Err(Error::BuildUrlError);
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(Error::BuildUrlError) }
                }
            },
            None => return Err(Error::BuildUrlError),
        }

        // Add queries. Correct combinations checked above,
        // So all we have to do is not panic on a None here.
        let mut queries = vec![];
        queries.push(("command", builder.command.as_ref().unwrap().to_string()));
        if builder.agency.is_some() {
            queries.push(("a", builder.agency.unwrap().to_owned()));
        }

        if builder.routes.is_some() {
            let mut route_queries: Vec<_> = builder.routes.as_ref().unwrap()
                .iter()
                .map(|route| {
                    ("r", route.to_string())
                })
                .collect();
            Vec::append(&mut queries, &mut route_queries);
        }

        if builder.stops.is_some() {
            let mut stop_queries: Vec<_> = builder.stops.as_ref().unwrap()
                .iter()
                .map(|stop| {
                    if builder.command == Some(Command::Predictions) {
                        ("s", stop.to_string())
                    } else {
                        ("stops", stop.to_string())
                    }
                })
                .collect();
            Vec::append(&mut queries, &mut stop_queries);
        }

        if builder.time.is_some() {
            queries.push(("t", builder.time.unwrap().to_string()));
        }

        // Create url
        let mut url = Url::parse(NEXTBUS_URL).unwrap();
        url.set_query_from_pairs(queries);

        Ok(Request(url))
    }

    pub fn send(self) -> ::Result<Response> {
        let client = Client::new();
        let Request(url) = self;
        let res = try!(client.get(url).send());
        Ok(res)
    }
}

// Request Builder

#[derive(Debug, PartialEq)]
pub struct RequestBuilder<'a> {
    command: Option<Command>,
    agency: Option<&'a str>,
    routes: Option<Vec<&'a str>>,
    stops:Option< Vec<&'a str>>,
    time: Option<usize>,
}

/// Build a Next Bus Request!
/// Last invocation of each method is the
/// one that "sticks"
// TODO: pass args as reference?
impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        RequestBuilder {
            command: None,
            agency: None,
            routes: None,
            stops: None,
            time: None,
        }
    }

    /// Command for Next Bus. Last invocation is the one that
    /// will be built.
    pub fn command(&mut self, command: Command) -> &mut Self {
        self.command = Some(command);
        self
    }

    /// Chose an agency. Last invocation is the one that will be
    /// built.
    pub fn agency(&mut self, agency: &'a str) -> &mut Self {
        self.agency = Some(agency);
        self
    }

    /// Chose a route. Last invocation is the one that will be
    /// built.
    pub fn route(&mut self, route: &'a str) -> &mut Self {
        self.routes = Some(vec![route]);
        self
    }

    /// Append a route. Last invocation is the one that will be
    /// built.
    pub fn add_route(&mut self, route: &'a str) -> &mut Self {
        //TODO: Better way to initialize?
        if self.routes.is_none() { self.routes = Some(vec![]); }
        self.routes.as_mut().map(|routes| routes.push(route));
        self
    }

    /// Chose a list of routes. Reaplces any and all previous
    /// routes in the list
    pub fn routes(&mut self, routes: Vec<&'a str>) -> &mut Self {
        self.routes = Some(routes);
        self
    }

    /// Append a list of routes to the current list of routes
    pub fn append_routes(&mut self, mut routes: Vec<&'a str>) -> &mut Self {
        self.routes.as_mut().map(|mut current_routes| {
            Vec::append(&mut current_routes, &mut routes);
        });
        self
    }

    /// Chose a stop. Last invocation is the one that will be
    /// built. Replaces any and all previous stops in the list.
    pub fn stop(&mut self, stop: &'a str) -> &mut Self {
        self.stops = Some(vec![stop]);
        self
    }

    /// Append a stop to the current list of stops.
    pub fn add_stop(&mut self, stop: &'a str) -> &mut Self {
        //TODO: Better way to initialize?
        if self.stops.is_none() {self.stops = Some(vec![]); }
        self.stops.as_mut().map(|stops| stops.push(stop));
        self
    }

    /// Chose a list of stops. Replaces any and all previous
    /// stops in the list.
    pub fn stops(&mut self, stops: Vec<&'a str>) -> &mut Self {
        self.stops = Some(stops);
        self
    }

    /// Append a list of stops to the current list of stops.
    // Not entirely clear why this can't be &mut in fn defn
    pub fn append_stops(&mut self, mut stops: Vec<&'a str>) -> &mut Self {
        self.stops.as_mut().map(|mut current_stops| {
            Vec::append(&mut current_stops, &mut stops);
        });
        self
    }

    /// Chose a time. Replaces previous any previous time.
    pub fn time(&mut self, time: usize) -> &mut Self {
        self.time = Some(time);
        self
    }

    pub fn create_request(&self) -> ::Result<Request> {
        Request::new(&self)
    }
}

// Components for building the NextBus Url

#[derive(Debug, PartialEq)]
pub enum Command {
    AgencyList,
    RouteList,
    RouteConfig,
    Predictions,
    PredictionsForMultiStops,
    Schedule,
    Messages,
    VehicleLocations,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::AgencyList => write!(f, "agencyList"),
            Command::RouteList => write!(f, "routeList"),
            Command::RouteConfig => write!(f, "routeConfig"),
            Command::Predictions => write!(f, "predictions"),
            Command::PredictionsForMultiStops => write!(f, "predictionsForMultiStops"),
            Command::Schedule => write!(f, "schedule"),
            Command::Messages => write!(f, "messages"),
            Command::VehicleLocations => write!(f, "vehicleLocations"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use super::*;
    use hyper::Url;

    #[test]
    // tuple constructor works inside module, but not outside
    fn builds_agency_list() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::AgencyList)
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?command=agencyList";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_routes_list() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::RouteList)
            .agency("test_agency")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=routeList&a=test_agency";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_route_config() {
        // for one route
        let built_request = RequestBuilder::new()
            .command(Command::RouteConfig)
            .agency("test_agency")
            .route("one")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=routeConfig&a=test_agency&r=one";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);

        // for many routes
        let built_request = RequestBuilder::new()
            .command(Command::RouteConfig)
            .agency("test_agency")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=routeConfig&a=test_agency";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_predictions() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::Predictions)
            .agency("test_agency")
            .route("one")
            .stop("stop_1")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=predictions&a=test_agency&r=one&s=stop_1";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_predictions_for_multi_stops() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::PredictionsForMultiStops)
            .agency("test_agency")
            .stop("stop_1")
            .add_stop("stop_2")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=predictionsForMultiStops&a=test_agency\
                   &stops=stop_1&stops=stop_2";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);

        // using add_stop first
        let built_request = RequestBuilder::new()
            .command(Command::PredictionsForMultiStops)
            .agency("test_agency")
            .add_stop("stop_1")
            .add_stop("stop_2")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=predictionsForMultiStops&a=test_agency\
                   &stops=stop_1&stops=stop_2";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_schedule() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::Schedule)
            .agency("test_agency")
            .route("one")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=schedule&a=test_agency&r=one";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_messages() {
        //normal, one route
        let built_request = RequestBuilder::new()
            .command(Command::Messages)
            .agency("test_agency")
            .route("one")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=messages&a=test_agency&r=one";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);

        // multiple routes
        let built_request = RequestBuilder::new()
            .command(Command::Messages)
            .agency("test_agency")
            .route("one")
            .add_route("two")
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=messages&a=test_agency&r=one&r=two";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    fn builds_vehicle_locations() {
        //normal
        let built_request = RequestBuilder::new()
            .command(Command::VehicleLocations)
            .agency("test_agency")
            .route("one")
            .time(0)
            .create_request()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?\
                   command=vehicleLocations&a=test_agency&r=one&t=0";
        let parsed_url = Request(Url::parse(url).unwrap());
        assert_eq!(built_request, parsed_url);
    }

    #[test]
    #[ignore]
    fn gets_agency_list() {
        let request = RequestBuilder::new()
            .command(Command::AgencyList)
            .create_request()
            .unwrap();
        let mut res = request.send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{:?}", res);
        println!("\n");
        println!("{:?}", body);
        assert!(false);
    }
}

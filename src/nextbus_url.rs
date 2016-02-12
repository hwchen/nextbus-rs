//! Module for building up Next Bus URL from components
//!
//! Builder pattern is used to create a NextBusUrl
//! from NextBusUrlBuilder
//!
//! The NextBusUrl is guaranteed to be valid!
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

use hyper::Url;
use std::fmt;

// Request Url Builder

#[derive(Clone, Debug, PartialEq)]
pub struct NextBusUrlBuilder<'a> {
    command: Option<Command>,
    agency: Option<AgencyTag<'a>>,
    routes: Option<Vec<RouteTag<'a>>>,
    stops:Option< Vec<StopTag<'a>>>,
    time: Option<Time>,
}

/// Build a Next Bus Url!
/// Last invocation of each method is the
/// one that "sticks"
// TODO: pass args as reference?
impl<'a> NextBusUrlBuilder<'a> {
    pub fn new() -> Self {
        NextBusUrlBuilder {
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
    pub fn agency(&mut self, agency: AgencyTag<'a>) -> &mut Self {
        self.agency = Some(agency);
        self
    }

    /// Chose a route. Last invocation is the one that will be
    /// built.
    pub fn route(&mut self, route: RouteTag<'a>) -> &mut Self {
        self.routes = Some(vec![route]);
        self
    }

    /// Append a route. Last invocation is the one that will be
    /// built.
    pub fn add_route(&mut self, route: RouteTag<'a>) -> &mut Self {
        self.routes.as_mut().map(|routes| routes.push(route));
        self
    }

    /// Chose a list of routes. Reaplces any and all previous
    /// routes in the list
    pub fn routes(&mut self, routes: Vec<RouteTag<'a>>) -> &mut Self {
        self.routes = Some(routes);
        self
    }

    /// Append a list of routes to the current list of routes
    pub fn append_routes(&mut self, mut routes: Vec<RouteTag<'a>>) -> &mut Self {
        self.routes.as_mut().map(|mut current_routes| {
            Vec::append(&mut current_routes, &mut routes);
        });
        self
    }

    /// Chose a stop. Last invocation is the one that will be
    /// built. Replaces any and all previous stops in the list.
    pub fn stop(&mut self, stop: StopTag<'a>) -> &mut Self {
        self.stops = Some(vec![stop]);
        self
    }

    /// Append a stop to the current list of stops.
    pub fn add_stop(&mut self, stop: StopTag<'a>) -> &mut Self {
        self.stops.as_mut().map(|stops| stops.push(stop));
        self
    }

    /// Chose a list of stops. Replaces any and all previous
    /// stops in the list.
    // Not entirely clear why this can't be &mut in fn defn
    pub fn stops(&mut self, mut stops: Vec<StopTag<'a>>) -> &mut Self {
        self.stops = Some(stops);
        self
    }

    /// Append a list of stops to the current list of stops.
    // Not entirely clear why this can't be &mut in fn defn
    pub fn append_stops(&mut self, mut stops: Vec<StopTag<'a>>) -> &mut Self {
        self.stops.as_mut().map(|mut current_stops| {
            Vec::append(&mut current_stops, &mut stops);
        });
        self
    }

    /// Chose a time. Replaces previous any previous time.
    pub fn time(&mut self, time: Time) -> &mut Self {
        self.time = Some(time);
        self
    }

    //TODO: make real error
    pub fn create_url(&self) -> Result<NextBusUrl, ()> {
        NextBusUrl::new(&self)
    }
}

/// A valid next bus url and query
/// Verified on building (dynamic). Because tuple struct constructors
/// with private fields cannot be invoked.
#[derive(Debug, PartialEq)]
pub struct NextBusUrl(Url);

impl NextBusUrl {
    pub fn new(builder: &NextBusUrlBuilder) -> Result<Self, ()> {
        // Check for invalid query combinations

        match builder.command {
            Some(Command::AgencyList) => {
                if builder.agency != None || builder.routes != None ||
                    builder.stops != None || builder.time != None {

                    return Err(());
                }
            },
            Some(Command::RouteList) => {
                if builder.agency == None || builder.routes != None ||
                    builder.stops != None || builder.time != None {

                    return Err(());
                }
            },
            // Can specify one or no routes
            Some(Command::RouteConfig) => {
                if builder.agency == None ||
                    builder.stops != None || builder.time != None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() <= 1 { return Err(()) }
                }
            },
            Some(Command::Predictions) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops == None || builder.time != None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(()) }
                }
                if let Some(ref stops) = builder.stops {
                    if stops.len() != 1 { return Err(()); }
                }
            },
            // No routes allowed?
            Some(Command::PredictionsForMultiStops) => {
                if builder.agency == None || builder.routes != None ||
                    builder.stops == None || builder.time != None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    // Shouldn't be able to happen, but double-check
                    if routes.is_empty() { return Err(()) }
                }
            },
            Some(Command::Schedule) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time != None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(()) }
                }
            },
            Some(Command::Messages) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time != None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    // Shouldn't be able to happen, but double-check
                    if routes.is_empty() { return Err(()) }
                }
            },
            Some(Command::VehicleLocations) => {
                if builder.agency == None || builder.routes == None ||
                    builder.stops != None || builder.time == None {

                    return Err(());
                }
                if let Some(ref routes) = builder.routes {
                    if routes.len() != 1 { return Err(()) }
                }
            },
            None => return Err(()),
        }

        // Add queries. Correct combinations checked above,
        // So all we have to do is not panic on a None here.
        let mut queries = vec![];
        queries.push(("command", builder.command.as_ref().unwrap().to_string()));
        if builder.agency.is_some() {
            queries.push(("a", builder.agency.as_ref().unwrap().to_string()));
        }
        if builder.time.is_some() {
            queries.push(("t", builder.time.as_ref().unwrap().to_string()));
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

        // Create url
        let mut url = Url::parse(NEXTBUS_URL).unwrap();
        url.set_query_from_pairs(queries);

        Ok(NextBusUrl(url))
    }
}

// Components for building the NextBus URl

const NEXTBUS_URL: &'static str = "http://webservices.nextbus.com/service/publicXMLFeed";

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct AgencyTag<'a>(pub &'a str);

impl<'a> fmt::Display for AgencyTag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let AgencyTag(ref agency_tag) = *self;
        write!(f, "{}", agency_tag)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RouteTag<'a>(pub &'a str);

impl<'a> fmt::Display for RouteTag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let RouteTag(ref route_tag) = *self;
        write!(f, "{}", route_tag)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopTag<'a>(pub &'a str);

impl<'a> fmt::Display for StopTag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let StopTag(ref stop_tag) = *self;
        write!(f, "{}", stop_tag)
    }
}

/// Parameter for specifying the last time information
/// was recevied for a Vehicle Location command.
#[derive(Clone, Debug, PartialEq)]
pub struct Time(pub usize);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Time(ref time) = *self;
        write!(f, "{}", time)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::Url;

    #[test]
    fn whatever() {
        let nextbus_url_2 = NextBusUrlBuilder::new()
            .command(Command::AgencyList)
            .agency(AgencyTag("something"))
            .create_url()
            .unwrap();
        let url = "http://webservices.nextbus.com/service/publicXMLFeed?command=agencyList";
        // tuple constructor works inside module, but not outside
        let nb_url = NextBusUrl(Url::parse(url).unwrap());
        println!("{:?}", nb_url);
        assert_eq!(nextbus_url_2, nb_url);
        //assert!(false);
    }
}

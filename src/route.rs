//! Module for handling route functions

/// List of routes. Maps directly from Nextbus response.
/// Contains vec of stub information for each route.
#[derive(Debug)]
pub struct RouteList(Vec<RouteStub>);

/// Used for parsing RouteList Nextbus response.
/// Contains a stub of route data.
#[derive(Debug)]
pub struct RouteStub {
    tag: String,
    title: String,
}

// Put routeconfig here
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

// Add prediction, schedule, messages, and vehicle locations


// To complicated?
///// Entry point for working with Routes
//#[derive(Debug)]
//pub struct Route {
//    tag: String,
//    title: String,
//    color: String, // should be valid color?
//    opposite_color: String,
//    lat_min: f32,
//    lat_max: f32,
//    lon_min: f32,
//    lon_max: f32,
//    directions: Vec<Direction>,
//    paths: Vec<Path>,
//    // all stops is a hash map of all stops in the route
//    // with relationship stop_tag: Stop for easy lookup
//    all_stops: HashMap<String, Stop>,
//}


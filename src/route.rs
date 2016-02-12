//! Module for handling route functions

use std::collections::HashMap;

/// Entry point for working with Routes
#[derive(Debug)]
pub struct Route<'a> {
    tag: &'a str,
    title: &'a str,
    color: &'a str, // should be valid color?
    opposite_color: &'a str,
    lat_min: f32,
    lat_max: f32,
    lon_min: f32,
    lon_max: f32,
    directions: Vec<Direction<'a>>,
    paths: Vec<Path>,
    // all stops is a hash map of all stops in the route
    // with relationship stop_tag: Stop for easy lookup
    all_stops: HashMap<&'a str, Stop<'a>>,
}

/// Used for parsing RouteList Nextbus response.
/// Contains a stub of route data.
#[derive(Debug)]
pub struct RouteStub<'a> {
    tag: &'a str,
    title: &'a str,
}

/// List of routes. Maps directly from Nextbus response.
/// Contains vec of stub information for each route.
#[derive(Debug)]
pub struct RouteList<'a>(Vec<RouteStub<'a>>);

/// A stop along a Route
#[derive(Debug)]
pub struct Stop<'a> {
    tag: &'a str,
    title: &'a str,
    lat: f32,
    long: f32,
    stop_id: &'a str,
}

/// An itinerary along a route
#[derive(Debug)]
pub struct Direction<'a> {
    tag: &'a str,
    title: &'a str,
    name: &'a str,
    use_for_ui: bool,
    // stops is an ordered vector of stop tags
    // References are kept because stops will
    // be referenced many times among the many
    // directions
    stops: Vec<&'a str>,
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

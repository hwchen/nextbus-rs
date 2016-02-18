//! Next Bus Route List Command
//!

use request::{Command, Request};
use ::Error;
use kuchiki;
use kuchiki::traits::*;
use std::io::Read;

pub struct RouteConfig(Vec<Route>);

impl RouteConfig {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteList(routes)
    }
}

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


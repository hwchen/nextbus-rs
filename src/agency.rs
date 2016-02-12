//! Module for handling agency functions

use std::collections::HashMap;
use route::Route;

/// Entry point for working with Agencies.
#[derive(Debug)]
pub struct Agency<'a> {
    tag: &'a str,
    title: &'a str,
    region_title: &'a str,
    routes: HashMap<&'a str, Route<'a>>,
}

/// Used for parsing AgencyList Nextbus response.
/// Contains a stub of route data.
#[derive(Debug)]
pub struct AgencyStub<'a> {
    tag: &'a str,
    title: &'a str,
    region_title: &'a str,
}

/// List of Agencies. Maps directly from Nextbus
/// response. Contains vec of stub information
/// for each route.
#[derive(Debug)]
pub struct AgencyList<'a>(Vec<AgencyStub<'a>>);


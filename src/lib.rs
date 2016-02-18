#![allow(dead_code)]
// TODO: Definitely take out verfication from url builder. It no longer needs it,
// since building is already constrained by individual builders.
//
// TODO: Make decisions about below after building routeConfig.
//
// TODO: Maybe move http request out of request, turn it into just a url builder.
// Because I don't want I/O hiding below.
//
// TODO: Maybe take out builder methods later from url builder, and just
// build inline. But don't have to do this until later.
//
// TODO: Review .select() in kuchiki?

//! NextBus
//! Rust API for NextBus API
//!
//! This library makes calls to the NextBus API, then parses
//! the output and serializes it.
//!
//! Features of the CLI:
//!
//! Set alarms on specific routes based on current location!
//!
//! ## Feature set
//!
//! - A convenient builder for an API request which returns a hyper Response.
//! - A convenient builder for an API request which returns a
//!
//! ## Structure
//!
//! ### API calls
//!
//! - AgencyList
//! - RouteList
//! - RouteConfig
//! - Predictions
//! - PredicionsForMultiStops
//! - Schedule
//! - Messages
//! - VehicleLocations

extern crate hyper;
extern crate kuchiki;

mod api;
mod error;
mod nb;
mod request;

use api::agency_list::AgencyListBuilder;
use api::route_list::RouteListBuilder;
pub use error::{Error, Result};
pub use request::{Request, Command};

pub struct NextBus;

impl<'a> NextBus {
    pub fn new() -> Self {
        NextBus
    }

    pub fn agency_list(self) -> AgencyListBuilder {
        AgencyListBuilder::new()
    }

    pub fn route_list(self) -> RouteListBuilder<'a> {
        RouteListBuilder::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn get_agency_list() {
        let agencies = NextBus::new()
            .agency_list()
            .get()
            .unwrap();
        println!("{:?}", agencies);
    }

    #[test]
    #[ignore]
    fn get_route_list() {
        let routes = NextBus::new()
            .route_list()
            .agency("mbta")
            .get()
            .unwrap();
        println!("{:?}", routes);
    }
}

#![allow(dead_code)]

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
//! - List and select agency
//! - List and select route for an agency
//! - For a route, get configuration (stops)
//! - For a route, get predictions
//! - For a route, get multi-stop predictions
//! - for a route, get schedule
//! - for a route, get messages
//! - for a route, get vehicle locations
//!
//! ## Structure
//!
//! Should I map the API to functions, or use objects?
//! Objects might allow for caching! I can have a refresh method too.
//! But maybe the API should be straight calls, and worry about a caching
//! layer separately? I could basically make a query builder and output parser.
//!
//! For example, I would need the following structs: AgencyList, RouteList,
//! RouteConfig, Prediction, PredictionMultiStop, Schedule, Message
//!
//! Then I could build caching layer on top of this.

extern crate hyper;
extern crate kuchiki;

pub mod agency;
mod error;
mod nb;
mod request;
pub mod route;

pub use error::{Error, Result};
pub use request::{Request, Command};

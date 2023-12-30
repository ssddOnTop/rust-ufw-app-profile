/// This lib is used for generating and adding UFW app profiles, so that your app can easily manage ports with UFW.
extern crate log;
mod config;
mod rootcheck;

pub use config::*;

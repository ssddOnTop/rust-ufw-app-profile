/// This lib is used for generating and adding UFW app profiles, so that your app can easily manage ports with UFW.

#[macro_use]
extern crate log;

pub mod config;
mod rootcheck;
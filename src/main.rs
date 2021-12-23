#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use std::time::SystemTime;
use std::net::(IpAddr, Ipv4Addr);

#[derive(serialize)]

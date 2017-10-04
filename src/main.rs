#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused)]

mod api;
mod server;
mod config;
mod consul;

#[cfg(test)]
mod config_test;

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate grpcio;
extern crate futures;
extern crate protobuf;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate reqwest;

extern crate serde;
extern crate url;

use clap::{Arg, App, SubCommand};
use std::process::exit;
use std::sync::Arc;

// use consul::client::Client as ConsulClient;

fn main() {
    let app = App::new("diplomat")
        .version(crate_version!())
        .about(
            "Provides the Envoy v2 API as a gRPC service and CLI application.",
        )
        .author("Timothy Perrett")
        .arg(
            Arg::with_name("c")
                .long("config")
                .value_name("config")
                .help("Path to the configuration for diplomat")
                .required(false)
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("eds")
                .about(
                    "given a service name, resolve the IPs providing that service",
                )
                .arg(
                    Arg::with_name("service-name")
                        .long("service-name")
                        .value_name("service-name")
                        .required(true)
                        .takes_value(true),
                ),
        );

    // TIM: Not sure if cloning here is going to cause problems,
    // but given this is once at the edge of the world it probally isn't
    // too much of a big deal.
    let matches = app.clone().get_matches();

    let config_path: &str = matches.value_of("consul-addr").unwrap_or("diplomat.toml");
    info!(
        "==>> attempting to load configuration from '{}'",
        config_path
    );

    let config = config::load(config_path.to_string());
    if config.is_err() {
        error!("==>> failed loading the specified configuration file... exiting.")
    }

    // TODO: Remove me.
    // let consul = Arc::new(ConsulClient::new("http://127.0.0.1:8500"));

    match matches.subcommand() {
        ("eds", Some(_)) => {
            // let ips = consul.catalog.get_nodes("consul".to_string()).unwrap();
            // consul.catalog.service("consul");
            println!("{:?}", "ips");
        }
        ("serve", Some(_)) => {
            ::server::start(config.unwrap());
        }
        _ => {
            let _ = app.clone().print_help();
            println!("");
        }
    }
}

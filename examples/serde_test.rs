#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_macros;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

use serde::Serialize;

fn setup_log(){
    let format = |record: &LogRecord| {
        format!("{level}:  {args}",
        level = record.level(),
        args  = record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    let log_var ="WEB_LOG";
    if env::var(log_var).is_ok() {
       builder.parse(&env::var(log_var).unwrap());
    }

    builder.init().unwrap();
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    name: String,
    age: u8
}

fn main(){
    setup_log();
    debug!("App started");

    let me = User{
        name: "hendrik".into(),
        age: 29
    };

    debug!("{}", serde_json::to_string(&me).unwrap());
}

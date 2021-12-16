use log::{debug, trace};
use std::convert::TryFrom;
use structopt::StructOpt;

use crate::errors::{Error, HurlResult};

#[derive(StructOpt, Debug)]
#[structopt(name = "hurl")]
pub struct App {
    #[structopt(short, long)]
    pub quiet: bool,

    #[structopt(short, long, parse(from_occurences))]
    pub verbose: u8,

    #[structopt(short, long)]
    pub form: bool,

    #[structopt(short, long)]
    pub auth: Option<String>,

    #[structopt(short, long)]
    pub secure: bool,

    #[structopt(subcommand)]
    pub cmd: Option<Method>,

    pub url: Option<String>,

    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

impl App {
    pub fn validate(&mut self) -> HurlResult<()> {
        if self.is_none() && self.url.is_none() {
            return Err(Error::MissingUrlAndCommand);
        }
        Ok(())
    }

    pub fn log_level(&self) -> Option<&'static str> {
        if self.quiet || self.verbose <= 0 {
            return None;
        }

        match self.verbose {
            1 => Some("error"),
            2 => Some("warn"),
            3 => Some("info"),
            4 => Some("debug"),
            _ => Some("trace"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "screaming_snake_case")]
pub enum Method {
    HEAD(MethodData),
    GET(MethodData),
    PUT(MethodData),
    POST(MethodData),
    PATCH(MethodData),
    DELETE(MethodData),
}

#[derive(StructOpt, Debug)]
pub struct MethodData {
    pub url: String,

    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

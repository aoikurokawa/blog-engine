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

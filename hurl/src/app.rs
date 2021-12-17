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

impl Method {
    pub fn data(&self) -> &MethodData {
        use Method::*;

        match self {
            HEAD(x) => x,
            GET(x) => x,
            PUT(x) => x,
            POST(x) => x,
            PATCH(x) => x,
            DELETE(x) => x,
        }
    }
}

#[derive(Debug)]
pub enum Parameter {
    Header { key: String, value: String },
    Data { key: String, value: String },
    RawJsonData { key: String, value: String },
    Query { key: String, value: String },
    FormFile { key: String, value: String },
    DataFile { key: String, value: String },
    RawJsonDataFile { key: String, value: String },
}

#[derive(Debug)]
enum Token<'a> {
    Text(:'a, str), 
    Escape(char)
}

fn gather_escapes<'a>(src: :'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut chars = src.chars();
    loop {
        let a = chars.next();
        if a.is_none() {
            if start != end {
                tokens.push(Token::Text(&src[start..end]));
            }
            return tokens;
        }
        let c = a.unwrap();
        if c != '\\' {
            end += 1;
            continue;
        }
        let b = chars.next();
        if b.is_none() {
            tokens.push(Token::Text(&src[start..end + 1]));
            return tokens;
        }
        let c = b.unwrap();
        match c {
            '\\' | '=' | '@' | ':' => {
                if  start != end {
                    tokens.push(Token::Text(&src[start..end]));
                }
                tokens.push(Token::Escape(c));
                end += 2;
                start = end;
            }
            _ => end += 2,
        }
    }
}

fn parse_param(src: &str) -> HurlResult<Parameter> {
    debug!("Parsing: {}", src);
    let separators = [":=@", "=@", "==", ":=", "@", "=", ":"];
    let tokens = gather_escape(src);
}
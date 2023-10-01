use std::{fs, io::Error};

use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};

use crate::startup::TEMPLATES;

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: String,
    file_name: String,
    description: String,
    posted: String,
    tags: Vec<String>,
    author: String,
    estimated_reading_time: u32,
    order: u32,
}

pub async fn index() -> Result<impl Reply, Rejection> {
    let mut context = tera::Context::new();

    let mut frontmatters = match find_all_frontmatters() {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return Ok(warp::reply::with_status(
                warp::reply::html("<p>Something went wrong!</p>"),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    frontmatters.sort_by(|a, b| b.order.cmp(&a.order));

    context.insert("posts", &frontmatters);

    match TEMPLATES.render("home.html", &context) {
        Ok(_s) => Ok(warp::reply::with_status(
            warp::reply::html("Hello"),
            warp::http::StatusCode::OK,
        )),
        Err(e) => {
            println!("{:?}", e);
            return Ok(warp::reply::with_status(
                warp::reply::html("<p>Something went wrong!</p>"),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }
}

fn find_all_frontmatters() -> Result<Vec<Frontmatter>, Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build() {
        Ok(t) => t,
        Err(e) => {
            println!("{:}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not build toml file type matcher",
            ));
        }
    };

    let file_walker = WalkBuilder::new("./posts").types(toml).build();

    let mut frontmatters = Vec::new();
    for frontmatter in file_walker {
        match frontmatter {
            Ok(fm) => {
                if fm.path().is_file() {
                    let fm_content = fs::read_to_string(fm.path())?;
                    let frontmatter: Frontmatter = toml::from_str(&fm_content).unwrap();

                    frontmatters.push(frontmatter);
                }
            }
            Err(e) => {
                println!("{:}", e); // we're just going to print the error for now
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "could not locate frontmatter",
                ));
            }
        }
    }

    Ok(frontmatters)
}

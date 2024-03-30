use std::{fs, io::Error};

use pulldown_cmark::{html, Options, Parser};
use warp::Rejection;

use crate::startup::TEMPLATES;

use super::home_handler::Frontmatter;

pub async fn post(post_name: String) -> Result<warp::http::Response<String>, Rejection> {
    let mut context = tera::Context::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let markdown_input = match extract_markdown(&post_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            let err_response = warp::http::Response::builder()
                .status(warp::http::StatusCode::NOT_FOUND)
                .header("content-type", "text/html")
                .body(String::from("<p>Could not find post - sorry!</p>"))
                .unwrap();
            return Ok(err_response);
        }
    };

    let frontmatter = match extract_frontmatter(&post_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            let err_response = warp::http::Response::builder()
                .status(warp::http::StatusCode::NOT_FOUND)
                .header("content-type", "text/html")
                .body(String::from("<p>Could not find post - sorry!</p>"))
                .unwrap();
            return Ok(err_response);
        }
    };

    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

    match TEMPLATES.render("post.html", &context) {
        Ok(s) => {
            let response = warp::http::Response::builder()
                .status(warp::http::StatusCode::OK)
                .header("content-type", "text/html")
                .body(s)
                .unwrap();
            Ok(response)
        }
        Err(e) => {
            println!("{:?}", e);
            let err_response = warp::http::Response::builder()
                .status(warp::http::StatusCode::NOT_FOUND)
                .header("content-type", "text/html")
                .body(String::from("<p>Could not find post - sorry!</p>"))
                .unwrap();
            Ok(err_response)
        }
    }
}

fn extract_markdown(post_name: &str) -> Result<String, Error> {
    let markdown = match fs::read_to_string(format!("./posts/{}/post.md", post_name)) {
        Ok(markdown) => markdown,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    Ok(markdown)
}

fn extract_frontmatter(post_name: &str) -> Result<Frontmatter, Error> {
    let frontmatter_input =
        match fs::read_to_string(format!("./posts/{}/post_frontmatter.toml", post_name)) {
            Ok(s) => s,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not find post frontmatter",
            ));
        }
    };

    Ok(frontmatter)
}

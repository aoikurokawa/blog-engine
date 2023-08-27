use std::{fs, io::Error};

use actix_web::{get, web, HttpResponse, Responder};
use pulldown_cmark::{html, Options, Parser};

use super::home_handler::Frontmatter;

#[get("/posts/{post_name}")]
pub async fn post(tmpl: web::Data<tera::Tera>, post_name: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();
    let mut options = Options::empty(); // used as part of pulldown_cmark for setting flags to enable extra features - we're not going to use any of those, hence the `empty();`
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let markdown_input = match extract_markdown(&post_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    let frontmatter = match extract_frontmatter(&post_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

    match tmpl.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
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

use gistcard::{get_media_tags, parse_gist_id};
use http::{header, StatusCode};
use log::debug;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use simplelog::{Config, Level, LevelFilter, SimpleLogger};
use std::error::Error;
use urlencoding::decode;

const LOG_CONFIG: Config = Config {
    time: Some(Level::Debug),
    level: Some(Level::Debug),
    target: None,
    location: None,
    time_format: Some("%T"),
};

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let id = decode(&req.uri().query().unwrap_or("").replace("id=", "")).map_err(|err| {
        debug!("Unable to decode gist id: {:?}", err);

        NowError::new("Unable to decode gist id. Please try again")
    })?;
    let gist_id = parse_gist_id(&id)
        .map_err(|_| NowError::new("Unable to parse that gist url/id. Please try again"))?;
    let meta_tags = get_media_tags(&gist_id).map_err(|_| {
        NowError::new("Unable to scrape the meta tags from the gist. Please try again")
    })?;
    let html = format!("<!DOCTYPE html><html><head>{}<meta name=\"twitter:image:src\" content=\"https://gistcard.now.sh/img/{}.png\" /></head><body><script>window.location.replace('https://gist.github.com/{}')</script></body></html>", meta_tags.join(""), gist_id, gist_id);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(html)
        .expect("Internal Server Error"))
}

fn main() -> Result<(), Box<dyn Error>> {
    match SimpleLogger::init(LevelFilter::Debug, LOG_CONFIG) {
        Ok(_) => {}
        Err(error) => eprintln!("Error setting up SimpleLogger: {:?}", error),
    };

    Ok(lambda!(handler))
}

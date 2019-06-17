use html5ever::rcdom::{NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, Attribute, ParseOpts};
use http::header;
use log::debug;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub enum GistError {
    BadGateway,
    NotFound,
    InvalidId,
    ImageError,
}

#[derive(Debug, Deserialize)]
pub struct GistFile {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Gist {
    pub url: String,
    pub html_url: String,
    pub description: Option<String>,
    pub user: Option<String>,
    pub files: HashMap<String, GistFile>,
}

pub fn parse_gist_id(id: &str) -> Result<String, GistError> {
    debug!("parsing incoming gist id: {}", id);

    let url_regex =
        Regex::new("^(?:https?://gist.github.com/(?:[A-z0-9-]*/)?)?(?P<id>[A-z0-9]+)$").unwrap();
    let gist_id = url_regex
        .captures(id)
        .ok_or_else(|| {
            debug!("Regex was unable to match the input id");

            GistError::InvalidId
        })?
        .name("id")
        .ok_or_else(|| {
            debug!("Regex did not return any matches for the capture group");

            GistError::InvalidId
        })?
        .as_str();

    Ok(gist_id.into())
}

pub fn get_gist(id: &str) -> Result<Gist, GistError> {
    debug!("fetching gist data as JSON");

    let gist_id = parse_gist_id(id)?;
    let client = Client::new();
    let res: Gist = client
        .get(format!("https://api.github.com/gists/{}", gist_id).as_str())
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .and_then(|mut res| res.json())
        .map_err(|err| {
            debug!("Error retrieving the gist: {:?}", err);

            GistError::BadGateway
        })?;

    Ok(res)
}

pub fn get_gist_markup(id: &str) -> Result<String, GistError> {
    debug!("fetching github gist markup");

    let gist_id = parse_gist_id(id)?;
    let client = Client::new();

    client
        .get(format!("https://gist.github.com/{}", gist_id).as_str())
        .send()
        .and_then(|mut res| res.text())
        .map_err(|err| {
            debug!("Error fetching gist markup: {:?}", err);

            GistError::BadGateway
        })
}

fn attr(name: &str, attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        if attr.name.local.as_ref() == name {
            return Some(attr.value.to_string());
        }
    }
    None
}

fn extract_media_prop(name: &str, attrs: &Vec<Attribute>) -> Option<(String, String)> {
    attr(name, attrs).and_then(|property| {
        if property.starts_with("og:")
            || (property.starts_with("twitter:") && !property.starts_with("twitter:image:src"))
        {
            attr("content", attrs).map(|content| (property, content))
        } else {
            None
        }
    })
}

pub fn get_media_tags(id: &str) -> Result<Vec<String>, GistError> {
    debug!("collecting meta tags");

    let markup = get_gist_markup(id)?;
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut markup.as_bytes())
        .unwrap();
    let document = &dom.document;
    let html = &document.children.borrow()[0];
    let head = &html.children.borrow()[0];
    let mut meta = Vec::new();

    for child in head.children.borrow().iter() {
        match &child.data {
            NodeData::Element { name, attrs, .. } => match name.local.as_ref() {
                "meta" => {
                    match extract_media_prop("name", &attrs.borrow()) {
                        Some((key, content)) => meta.push((key, content)),
                        None => {}
                    };
                    match extract_media_prop("property", &attrs.borrow()) {
                        Some((key, content)) => meta.push((key, content)),
                        None => {}
                    };
                }
                _ => {}
            },
            _ => {}
        }
    }

    let meta_tags = meta
        .iter()
        .map(|(key, content)| match key.starts_with("twitter:") {
            true => format!("<meta name=\"{}\" content=\"{}\" />", key, content),
            false => format!("<meta property=\"{}\" content=\"{}\" />", key, content),
        })
        .collect();

    Ok(meta_tags)
}

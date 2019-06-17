use base64;
use gistcard::{get_gist, Gist, GistError, GistFile};
use http::{header, StatusCode};
use log::debug;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use simplelog::{Config, Level, LevelFilter, SimpleLogger};
use std::error::Error;
use urlencoding;

const LOG_CONFIG: Config = Config {
    time: Some(Level::Debug),
    level: Some(Level::Debug),
    target: None,
    location: None,
    time_format: Some("%T"),
};

#[derive(Debug, Serialize)]
struct Highlight {
    background: String,
    text: String,
    variable: String,
    attribute: String,
    definition: String,
    keyword: String,
    operator: String,
    property: String,
    number: String,
    string: String,
    comment: String,
    meta: String,
    tag: String,
}

#[derive(Debug, Serialize)]
struct CarbonImage {
    #[serde(rename = "paddingVertical")]
    padding_vertical: String,
    #[serde(rename = "paddingHorizontal")]
    padding_horizontal: String,
    #[serde(rename = "marginVertical")]
    margin_vertical: String,
    #[serde(rename = "marginHorizontal")]
    margin_horizontal: String,
    #[serde(rename = "backgroundImage")]
    background_image: Option<String>,
    #[serde(rename = "backgroundImageSelection")]
    background_image_selection: Option<String>,
    #[serde(rename = "backgroundMode")]
    background_mode: String,
    #[serde(rename = "backgroundColor")]
    background_color: String,
    #[serde(rename = "dropShadow")]
    drop_shadow: bool,
    #[serde(rename = "dropShadowOffsetY")]
    drop_shadow_offset_y: String,
    #[serde(rename = "dropShadowBlurRadius")]
    drop_shadow_blur_radius: String,
    theme: String,
    #[serde(rename = "windowTheme")]
    window_theme: String,
    language: String,
    #[serde(rename = "fontFamily")]
    font_family: String,
    #[serde(rename = "fontSize")]
    font_size: String,
    #[serde(rename = "lineHeight")]
    line_height: String,
    #[serde(rename = "windowControls")]
    window_controls: bool,
    #[serde(rename = "widthAdjustment")]
    width_adjustment: bool,
    #[serde(rename = "lineNumbers")]
    line_numbers: bool,
    #[serde(rename = "exportSize")]
    export_size: String,
    watermark: bool,
    #[serde(rename = "squaredImage")]
    squared_image: bool,
    code: String,
    loading: bool,
    preset: String,
    highlights: Highlight,
}

impl CarbonImage {
    fn default() -> CarbonImage {
        CarbonImage {
            padding_vertical: "56px".into(),
            padding_horizontal: "56px".into(),
            margin_vertical: "45px".into(),
            margin_horizontal: "45px".into(),
            background_image: None,
            background_image_selection: None,
            background_mode: "color".into(),
            background_color: "rgba(189,210,157,1)".into(),
            drop_shadow: true,
            drop_shadow_offset_y: "20px".into(),
            drop_shadow_blur_radius: "68px".into(),
            theme: "dracula".into(),
            window_theme: "none".into(),
            language: "auto".into(),
            font_family: "hack".into(),
            font_size: "14px".into(),
            line_height: "133%".into(),
            window_controls: false,
            width_adjustment: false,
            line_numbers: false,
            export_size: "2x".into(),
            watermark: false,
            squared_image: false,
            code: "console.log(\"hello world\");".into(),
            loading: false,
            preset: "preset:4".into(),
            highlights: Highlight {
                background: "#151718".into(),
                text: "#CFD2D1".into(),
                variable: "#a074c4".into(),
                attribute: "#9fca56".into(),
                definition: "#55b5db".into(),
                keyword: "#e6cd69".into(),
                operator: "#9fca56".into(),
                property: "#a074c4".into(),
                number: "#cd3f45".into(),
                string: "#55b5db".into(),
                comment: "#41535b".into(),
                meta: "#55b5db".into(),
                tag: "#55b5db".into(),
            },
        }
    }

    fn with_code(code: &str) -> CarbonImage {
        CarbonImage {
            code: code.into(),
            ..CarbonImage::default()
        }
    }
}

fn decode_image(blob: &str) -> Result<Vec<u8>, GistError> {
    let base64_data = blob.replace("data:image/png;base64,", "");

    base64::decode(&base64_data).map_err(|err| {
        debug!("Error decoding image data: {:?}", err);

        GistError::ImageError
    })
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let client = Client::new();
    let id = req.uri().query().unwrap_or("").replace("id=", "");
    let Gist { files, .. } = get_gist(&id).map_err(|err| match err {
        GistError::InvalidId => NowError::new("An invalid ID was provided. Please try again"),
        GistError::BadGateway => {
            NowError::new("For some reason the GitHub API isn't responding. Please try again later")
        }
        GistError::NotFound => NowError::new("That gist can't be found. Is it private?"),
        _ => NowError::new("Something went terribly wrong. Please try again"),
    })?;
    let GistFile { content, .. } = files.iter().map(|(_, v)| v).collect::<Vec<&GistFile>>()[0];
    let state = CarbonImage::with_code(content);
    let encoded_state = base64::encode(
        urlencoding::encode(serde_json::to_string(&state).unwrap().as_str()).as_str(),
    );
    let res = client
        .post("https://carbon.now.sh/api/image")
        .json(&json!({ "state": encoded_state }))
        .send()
        .and_then(|mut res| res.text())
        .map(|data| decode_image(&data))
        .map_err(|err| {
            debug!("Unable to create the image: {:?}", err);

            GistError::BadGateway
        });

    match res {
        Ok(Ok(value)) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "image/png")
            .body(value)
            .expect("Internal Server Error")),
        _ => {
            debug!("Error creating the image from carbon");

            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body("Error creating the code image".as_bytes().to_vec())
                .expect("Internal Server Error"))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match SimpleLogger::init(LevelFilter::Debug, LOG_CONFIG) {
        Ok(_) => {}
        Err(error) => eprintln!("Error setting up SimpleLogger: {:?}", error),
    };

    Ok(lambda!(handler))
}

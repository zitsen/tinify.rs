extern crate curl;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

use curl::easy::Easy;
use reqwest::RequestBuilder;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, stdout, Read, Write};
use std::ops::Deref;
use std::path::Path;

mod serde_url;
mod data;

use data::Data;
use serde_url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ApiError {
    error: String,
    message: String,
}

#[test]
fn test_api_error_serde() {
    let err = ApiError {
        error: "A".into(),
        message: "B".into(),
    };
    let json = serde_json::to_string(&err).unwrap();
    assert_eq!(json, r#"{"error":"A","message":"B"}"#.to_string());
    let new_err = serde_json::from_str(&json).unwrap();
    assert_eq!(err, new_err);
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.error, self.message)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TinifyError {
        File(err: io::Error) {
            from()
            description("tinify io error")
            cause(err)
        }
        Curl(err: curl::Error) {
            from()
            cause(err)
        }
        Request(err: reqwest::Error) {
            from()
            cause(err)
        }
        Serde(err: serde_json::Error) {
            from()
            cause(err)
        }
        Api(err: ApiError) {
            from()
            description("API error")
            display(me) -> ("{}", me)
        }
    }
}

pub struct Tinify {
    ua: reqwest::Client,
}

pub struct Shrinked {
    location: Url,
    compression_count: usize,
    data: Data,
}

pub struct Image {
    compression_count: usize,
    image_width: usize,
    image_height: usize,
    content_type: String,
    content_length: usize,
    data: Data,
}

pub struct Stored {
    compression_count: usize,
    image_width: usize,
    image_height: usize,
    location: Url,
}

/// Tinify client struct
///
/// ```text
/// Tinify.new()
///     .shrink()?
///     .resize("fit", 18, 18)?
///     .preserve()?
///     .save_to_s3("example-bucket/my-images/optimized.jpg")?
///     .save("file.png")?;
/// ```
impl Tinify {
    pub fn new() -> Tinify {
        let mut client = reqwest::Client::builder();
        if let Ok(proxy) = env::var("https_proxy") {
            client.proxy(reqwest::Proxy::https(&proxy).expect("proxy"));
        }
        client.timeout(::std::time::Duration::new(300, 0));
        Tinify {
            ua: client.build().expect("build client"),
        }
    }

    /// Compressing images from file or url
    pub fn shrink<T: AsRef<str>>(mut self, data: T) -> Result<Self, TinifyError> {
        let data = data.as_ref();
        if let Ok(url) = Url::parse(data) {
            debug!("{} is an URL", url);
            self.shrink_from_url(url)
        } else {
            self.shrink_from_file(Path::new(data))
        }
    }

    fn prepare_shrink(&mut self) -> RequestBuilder {
        static API_ENDPOINT: &'static str = "https://api.tinify.com/shrink";
        let password = env::var("TINIFY_KEY")
            .expect("TINIFY_KEY must exists")
            .as_str()
            .to_string();
        let mut req = self.ua.post(API_ENDPOINT);
        req.basic_auth("api", Some(password));

        req
    }
    fn shrink_from_url(mut self, url: Url) -> Result<Self, TinifyError> {
        debug!("Shrink from image url: {}", url);
        Ok(self)
    }

    fn shrink_from_file(mut self, path: &Path) -> Result<Self, TinifyError> {
        debug!("Shrink from file: {}", path.display());
        let mut file = File::open(path)?;

        let mut res = self.prepare_shrink().body(file).send()?;

        use reqwest::StatusCode;
        match res.status() {
            StatusCode::Ok => {
                info!("Get compressed image");
            }
            StatusCode::Created => {
                let dd: Data = res.json()?;
                debug!("{:?}", dd);
                info!("Compressing complete");
            }
            StatusCode::Unauthorized => {
                let err: ApiError = res.json()?;
                debug!("{:?}", err);
                return Err(err.into());
            }
            code if code >= StatusCode::BadRequest => {
                let err: ApiError = res.json()?;
                debug!("{:?}", err);
                return Err(err.into());
            }
            _ => unimplemented!(),
        }
        Ok(self)
    }
}

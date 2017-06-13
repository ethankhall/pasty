use std::io::Read;
use std::io;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::fmt;

use hyper;
use hyper::client::Client;
use hyper::header;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;

use serde_json;
use serde_json::Value;

#[derive(Debug)]
pub enum UploadError {
    IOError(io::Error),
    TlsError(String),
    HyperError(hyper::error::Error),
    ApiError(StatusCode),
    ParseError(serde_json::error::Error),
}

impl From<io::Error> for UploadError {
    fn from(e: io::Error) -> UploadError {
        UploadError::IOError(e)
    }
}

impl From<hyper::error::Error> for UploadError {
    fn from(e: hyper::error::Error) -> UploadError {
        UploadError::HyperError(e)
    }
}

impl From<serde_json::error::Error> for UploadError {
    fn from(e: serde_json::error::Error) -> UploadError {
        UploadError::ParseError(e)
    }
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message: String = match *self {
            UploadError::IOError(ref e) => e.to_string(),
            UploadError::HyperError(ref e) => e.to_string(),
            UploadError::ParseError(ref e) => e.to_string(),
            UploadError::ApiError(code) => format!("Server responded with status code {}", code),
            UploadError::TlsError(ref e) => e.clone(),
        };
        write!(f, "{}", message)
    }
}

impl Error for UploadError {
    fn description(&self) -> &str {
        match *self {
            UploadError::IOError(ref e) => e.description(),
            UploadError::HyperError(ref e) => e.description(),
            UploadError::ParseError(ref e) => e.description(),
            UploadError::TlsError(ref e) => e.as_str(),
            UploadError::ApiError(_) => {
                "The server responded with a status code that was not successful."
            }
        }
    }
}

const USER_AGENT: &'static str = "pasty CLI (https://github.com/joek13/pasty)";

pub mod hastebin {
    use uploader::*;
    #[derive(Deserialize, Serialize)]
    struct Response {
        key: String,
    }
    /// Uploads something to Hastebin.
    /// # Errors
    /// Errors if reading fails, it contains invalid UTF-8, or anything
    /// else goes wrong during uploading (i.e. network issues, rate-limits, etc.)
    pub fn upload<T: Read>(source: &mut T) -> Result<String, UploadError> {
        let mut contents = String::new();
        source.read_to_string(&mut contents)?;

        //actually upload the file
        let ssl = NativeTlsClient::new()
            .map_err(|e| UploadError::TlsError(e.to_string()))?;
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let mut res = client
            .post("https://hastebin.com/documents")
            .body(contents.as_str())
            .header(header::UserAgent(USER_AGENT.to_owned()))
            .send()?;

        if res.status.is_success() {
            let mut response_body = String::new();
            res.read_to_string(&mut response_body)?;
            let r: Response = serde_json::from_str(response_body.as_str())?;
            Ok(format!("https://hastebin.com/{}", r.key))
        } else {
            return Err(UploadError::ApiError(res.status));
        }
    }
    ///Uploads a file.
    ///See hastebin::upload for errors and more.
    pub fn upload_file<P: AsRef<Path>>(path: P) -> Result<String, UploadError> {
        let mut f = File::open(path)?;
        upload(&mut f)
    }
}

pub mod github {
    use uploader::*;
    use std::ffi::OsStr;

    /// Uploads something to Github.
    /// Returns the Url.
    /// # Errors
    /// Errors if anything goes wrong while uploading.
    pub fn upload<T: Read>(source: &mut T, name: String) -> Result<String, UploadError> {
        let mut contents = String::new();
        source.read_to_string(&mut contents)?;

        let ssl = NativeTlsClient::new()
            .map_err(|e| UploadError::TlsError(e.to_string()))?;
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let body = json!({
           "public": true,
           "description": "",
           "files": {
               name: {
                   "content": contents
               }
           }
        })
                .to_string();

        let mut res = client
            .post("https://api.github.com/gists")
            .header(header::UserAgent(USER_AGENT.to_owned()))
            .body(body.as_str())
            .send()?;

        let mut response = String::new();
        res.read_to_string(&mut response)?;
        if res.status.is_success() {
            let response: Value = serde_json::from_str(&response)?;
            if let Some(url) = response.get("html_url") {
                if let Some(url) = url.as_str() {
                    Ok(url.to_owned())
                } else {
                    Err(UploadError::ApiError(res.status))
                }
            } else {
                Err(UploadError::ApiError(res.status))
            }
        } else {
            Err(UploadError::ApiError(res.status))
        }
    }
    pub fn upload_file<P: AsRef<Path>>(path: P,
                                       name: Option<String>)
                                       -> Result<String, UploadError> {
        let name = match name {
            Some(x) => x,
            None => {
                path.as_ref()
                    .file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .into_owned()
            }
        };

        let mut f = File::open(path)?;
        upload(&mut f, name)
    }
}

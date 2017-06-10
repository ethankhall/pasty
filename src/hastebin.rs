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
use hyper_native_tls::NativeTlsClient;

#[derive(Debug)]
pub enum UploadError
{
   IOError(io::Error),
   TlsError(String),
   HyperError(hyper::error::Error)
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message: String = match *self {
            UploadError::IOError(ref e) => e.to_string(),
            UploadError::HyperError(ref e) => e.to_string(),
            UploadError::TlsError(ref e) => e.clone()
        };
        write!(f, "{}", message)
    }
}

impl Error for UploadError {
    fn description(&self) -> &str {
        match *self {
            UploadError::IOError(ref e) => e.description(),
            UploadError::HyperError(ref e) => e.description(),
            UploadError::TlsError(ref e) => e.as_str()
        }
    }
}
/// Uploads something to Hastebin.
/// # Errors
/// Errors if reading fails, it contains invalid UTF-8, or anything
/// else goes wrong during uploading (i.e. network issues, rate-limits, etc.)
pub fn upload<T: Read>(source: &mut T) -> Result<String, UploadError> {
    let mut contents = String::new();
    source.read_to_string(&mut contents)
        .map_err(|e| UploadError::IOError(e))?;
    
    //actually upload the file
    let ssl = NativeTlsClient::new()
        .map_err(|e| UploadError::TlsError(e.to_string()))?;
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut res = client.post("https://hastebin.com/documents")
        .body(contents.as_str())
        .header(header::UserAgent("Hastebin CLI (https://github.com/joek13/hastebin-client)".to_owned()))
        .send()
        .map_err(|e| UploadError::HyperError(e))?;

    let mut response_body = String::new();
    res.read_to_string(&mut response_body)
        .map_err(|e| UploadError::IOError(e))?;

    println!("{}", response_body);

    Ok("we good".to_owned())
}
///Uploads a file.
///See hastebin::upload for errors and more.
pub fn upload_file<P: AsRef<Path>> (path: P) -> Result<String, UploadError> {
    let mut f = File::open(path)
        .map_err(|e| UploadError::IOError(e))?;
    upload(&mut f)
}

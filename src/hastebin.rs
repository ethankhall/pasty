use std::io::Read;
use std::io;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UploadError
{
   IOError(io::Error)
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message: String = match *self {
            UploadError::IOError(ref e) => e.to_string()
        };
        write!(f, "{}", message)
    }
}

impl Error for UploadError {
    fn description(&self) -> &str {
        match *self {
            UploadError::IOError(ref e) => e.description()
        }
    }
}

pub fn upload<T: Read>(source: &mut T) -> Result<String, UploadError> {
    let mut contents = String::new();
    source.read_to_string(&mut contents)
        .map_err(|e| UploadError::IOError(e))?;
    Ok("we good".to_owned())
}
pub fn upload_file<P: AsRef<Path>> (path: P) -> Result<String, UploadError> {
    let mut f = File::open(path)
        .map_err(|e| UploadError::IOError(e))?;
    upload(&mut f)
}

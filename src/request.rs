extern crate reqwest;

use error::Error;

type RequestResult<T> = Result<T, Error>;

pub fn request() -> RequestResult<String> {
    let mut res =
        reqwest::get("http://sample.com/")?;
    if !res.status().is_success() {
        let err = Error::InvalidStatusCode { code: res.status() };
        return Err(err);
    }
    return Ok(res.text()?);
}

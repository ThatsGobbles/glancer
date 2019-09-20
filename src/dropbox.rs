use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    ListingFetch(reqwest::Error),
    ListingParse(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ListingFetch(ref err) => write!(f, "error when listing files: {}", err),
            Self::ListingParse(ref err) => write!(f, "error when deserializing payload: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::ListingFetch(ref err) => Some(err),
            Self::ListingParse(ref err) => Some(err),
        }
    }
}

#[derive(Deserialize)]
pub struct ImageListing {
    entries: Vec<ImageMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct ImageMetadata {
    path_lower: String,
    name: String,
}

pub struct HTTPTempLink(String);

pub fn get_image_listing() -> Result<ImageListing, Error> {
    let client = Client::new();
    let mut response = client
        .post("https://api.dropboxapi.com/2/files/list_folder")
        .header("Authorization", "Bearer KHopaZZMKCIAAAAAAAAH5ojsxHMYzWlrvd_6wCTguwkRbQCPNTt6NSL-C5YpDNUX")
        .header("Content-Type", "application/json")
        .body(r#"{ "path": "" }"#)
        .send()
        .map_err(Error::ListingFetch)?
    ;

    let http_list_response: ImageListing = response.json().map_err(Error::ListingParse)?;

    Ok(http_list_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image_listing() {
        let response = get_image_listing().unwrap();
        println!("{:?}", response.entries);
    }
}

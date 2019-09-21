use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    ListingFetch(reqwest::Error),
    ListingParse(reqwest::Error),
    TempLinkFetch(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ListingFetch(ref err) => write!(f, "error when fetching file listing: {}", err),
            Self::ListingParse(ref err) => write!(f, "error when parsing file listing: {}", err),
            Self::TempLinkFetch(ref err) => write!(f, "error when fetching temporary link: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::ListingFetch(ref err) => Some(err),
            Self::ListingParse(ref err) => Some(err),
            Self::TempLinkFetch(ref err) => Some(err),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ImageListing {
    entries: Vec<ImageMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct ImageMetadata {
    path_lower: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageTempLink {
    link: String,
}

pub struct Dropbox;

impl Dropbox {
    pub fn get_image_listing(token: &str) -> Result<ImageListing, Error> {
        let client = Client::new();
        let mut response = client
            .post("https://api.dropboxapi.com/2/files/list_folder")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(r#"{ "path": "" }"#)
            .send()
            .map_err(Error::ListingFetch)?
        ;

        response.json().map_err(Error::ListingParse)
    }

    pub fn get_temporary_link(path: &str, token: &str) -> Result<ImageTempLink, Error> {
        let client = Client::new();
        let mut response = client
            .post("https://api.dropboxapi.com/2/files/get_temporary_link")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(format!(r#"{{ "path": "{}" }}"#, path))
            .send()
            .map_err(Error::TempLinkFetch)?
        ;

        response.json().map_err(Error::ListingParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str  = "KHopaZZMKCIAAAAAAAAH5ojsxHMYzWlrvd_6wCTguwkRbQCPNTt6NSL-C5YpDNUX";

    #[test]
    fn test_get_image_listing() {
        let response = Dropbox::get_image_listing(&TOKEN).unwrap();
        println!("{:?}", response.entries);
    }

    #[test]
    fn test_get_temporary_link() {
        const TEST_PATH: &str = "/robert-horvick-1r4upyipcfm-unsplash.jpg";
        let response = Dropbox::get_temporary_link(&TEST_PATH, &TOKEN).unwrap();
        println!("{}", response.link);
    }
}

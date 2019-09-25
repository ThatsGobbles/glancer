use reqwest::Client;
use serde::Deserialize;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Error {
    ListingFetch(reqwest::Error),
    ListingParse(reqwest::Error),
    TempLinkFetch(reqwest::Error),
    EmptyListing,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ListingFetch(ref err) => write!(f, "error when fetching file listing: {}", err),
            Self::ListingParse(ref err) => write!(f, "error when parsing file listing: {}", err),
            Self::TempLinkFetch(ref err) => write!(f, "error when fetching temporary link: {}", err),
            Self::EmptyListing => write!(f, "file listing is empty"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::ListingFetch(ref err) => Some(err),
            Self::ListingParse(ref err) => Some(err),
            Self::TempLinkFetch(ref err) => Some(err),
            Self::EmptyListing => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ImageListing {
    pub entries: Vec<ImageMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct ImageMetadata {
    pub path_lower: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageTempLink {
    pub link: String,
}

pub struct Dropbox;

impl Dropbox {
    pub fn get_image_listing(token: &str) -> Result<Vec<String>, Error> {
        let client = Client::new();
        let mut response = client
            .post("https://api.dropboxapi.com/2/files/list_folder")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(r#"{ "path": "" }"#)
            .send()
            .map_err(Error::ListingFetch)?
        ;

        response.json::<ImageListing>().map_err(Error::ListingParse).map(|il| il.entries.into_iter().map(|e| e.path_lower).collect())
    }

    pub fn get_temporary_link(path: &str, token: &str) -> Result<String, Error> {
        let client = Client::new();
        let mut response = client
            .post("https://api.dropboxapi.com/2/files/get_temporary_link")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(format!(r#"{{ "path": "{}" }}"#, path))
            .send()
            .map_err(Error::TempLinkFetch)?
        ;

        response.json::<ImageTempLink>().map_err(Error::ListingParse).map(|itl| itl.link)
    }

    pub fn get_random_temporary_link(token: &str) -> Result<String, Error> {
        let listing = Self::get_image_listing(&token)?;
        let mut rng = rand::thread_rng();
        let choice = listing.choose(&mut rng).ok_or(Error::EmptyListing)?;

        Self::get_temporary_link(&choice, &token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str  = "KHopaZZMKCIAAAAAAAAH5ojsxHMYzWlrvd_6wCTguwkRbQCPNTt6NSL-C5YpDNUX";

    #[test]
    fn test_get_image_listing() {
        let response = Dropbox::get_image_listing(&TOKEN).unwrap();
        println!("{:?}", response);
    }

    #[test]
    fn test_get_temporary_link() {
        const TEST_PATH: &str = "/robert-horvick-1r4upyipcfm-unsplash.jpg";
        let temp_link = Dropbox::get_temporary_link(&TEST_PATH, &TOKEN).unwrap();
        println!("{}", temp_link);
    }

    #[test]
    fn test_get_random_temporary_link() {
        let temp_link = Dropbox::get_random_temporary_link(&TOKEN).unwrap();
        println!("{}", temp_link);
    }
}

use reqwest::Client;
use reqwest::Response;

pub struct HTTPListResponse(Vec<ImageMetadata>);

pub struct ImageMetadata {
    path: String,
    name: String,
}

pub struct HTTPTempLink(String);

pub fn list_images() -> Response {
    let client = Client::new();
    let response = {
        client
            .post("https://api.dropboxapi.com/2/files/list_folder")
            .header("Authorization", "Bearer KHopaZZMKCIAAAAAAAAH5ojsxHMYzWlrvd_6wCTguwkRbQCPNTt6NSL-C5YpDNUX")
            .header("Content-Type", "application/json")
            // .header("Dropbox-Api-Select-User", "")
            .body(r#"{ "path": "" }"#)
            .send()
            .unwrap()
    };

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_images() {
        let mut response = list_images();
        println!("STATUS: {}", response.status());
        println!("TEXT: {:?}", response.text().unwrap());
        // let json_str: String = response.json().unwrap();
        // println!("{}", json_str);
    }
}

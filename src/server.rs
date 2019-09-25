use rouille::Response;

use crate::template::Dropbox as DropboxTemplate;
use crate::dropbox::Dropbox;

pub fn start_server() {
    rouille::start_server("0.0.0.0:9001", move |_request| {
        let random_image_link = Dropbox::get_random_temporary_link("KHopaZZMKCIAAAAAAAAH5ojsxHMYzWlrvd_6wCTguwkRbQCPNTt6NSL-C5YpDNUX").unwrap();
        let templated = DropboxTemplate { img_url: &random_image_link };
        Response::html(templated.to_string())
    });
}

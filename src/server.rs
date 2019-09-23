use rouille::Response;

use crate::template::Hello;

pub fn start_server() {
    rouille::start_server("0.0.0.0:9001", move |_request| {
        let templated = Hello { name: "Marko" };
        Response::html(templated.to_string())
    });
}

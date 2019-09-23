mod config;
mod dropbox;

use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:9001", move |_request| {
        Response::text("Hello!")
    });
}

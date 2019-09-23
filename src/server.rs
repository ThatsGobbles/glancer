use rouille::Response;

pub fn start_server() {
    rouille::start_server("0.0.0.0:9001", move |_request| {
        Response::text("Hello!")
    });
}

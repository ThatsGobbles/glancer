mod config;
mod dropbox;
mod server;
mod template;

fn main() {
    server::start_server();
}

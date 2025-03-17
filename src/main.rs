use {{crate_name}}::server::Server;
use {{crate_name}}::settings::Initializer;

#[tokio::main]
async fn main() {
    Initializer::new().init();
    Server::serve().await;
}

use {{project_name}}::server::Server;
use {{project_name}}::settings::Initializer;

#[tokio::main]
async fn main() {
    Initializer::new().init();
    Server::serve().await;
}

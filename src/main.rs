use tokio;
mod udp;
mod relay;
mod resolver;

#[tokio::main]
async fn main() {
    let relay_configs = realm::parse_arguments();
    relay::start_relay(relay_configs).await;
}

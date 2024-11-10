use tganalytics::client::client;
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let api_id = std::env::var("API-ID").unwrap().parse().unwrap();
    let api_hash = std::env::var("API-HASH").unwrap();
    let session_file = std::env::var("SESSION-FILE").unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(client(api_id, &api_hash, &session_file))
}

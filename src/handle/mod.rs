mod message;
use grammers_client::{Client, Update};
use message::get_message;

pub async fn handle_update(
    update: Update,
    client: Client,
) -> Result<(), Box<dyn std::error::Error>> {
    match update {
        Update::NewMessage(message) if !message.outgoing() => get_message(message, client).await?,
        _ => {}
    };
    Ok(())
}

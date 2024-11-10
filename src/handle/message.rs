use grammers_client::{types::Message, Client};

pub async fn get_message(
    message: Message,
    client: Client,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

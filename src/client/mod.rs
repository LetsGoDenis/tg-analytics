use crate::handle::handle_update;
use futures_util::future::{select, Either};
use grammers_client::{Client, Config};
use grammers_session::Session;
use std::{
    io::{self, BufRead, Write},
    pin::pin,
};

pub async fn client(
    api_id: i32,
    api_hash: &str,
    session_file: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = auth(api_id, api_hash, session_file)
        .await
        .expect("Wasn't able to create a client");
    loop {
        let update = {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let upd = pin!(async { client.next_update().await });
            match select(exit, upd).await {
                Either::Left(_) => None,
                Either::Right((u, _)) => Some(u),
            }
        };
        let update = match update {
            None => break,              // Ending if None
            Some(Ok(update)) => update, // Correct update
            Some(Err(e)) => {
                eprintln!("Error fetching update: {:?}", e);
                continue;
            }
        };
        let handle = client.clone();
        tokio::task::spawn(async move {
            handle_update(update, handle)
                .await
                .expect("Unable to handle an incoming update")
        });
    }
    Ok(())
}
async fn auth(
    api_id: i32,
    api_hash: &str,
    session_file: &str,
) -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::connect(Config {
        session: Session::load_file_or_create(session_file.to_string())
            .expect("Unable to find session file."),
        api_id: api_id,
        api_hash: api_hash.to_string(),
        params: Default::default(),
    })
    .await?;
    let mut sing_out = false;
    if !client.is_authorized().await? {
        println!("Singing in...");
        let phone = prompt("Enter your phone number (international format) without +: ")?;
        let token = client.request_login_code(&phone).await?;
        let code = prompt("Enter the code that you have received: ")?;
        let signed_in = client.sign_in(&token, &code).await;
        match signed_in {
            Err(grammers_client::SignInError::PasswordRequired(password_token)) => {
                let hint = password_token.hint().unwrap_or("None");
                let message = format!("Enter the password (hint {}): ", &hint);
                let password = prompt(&message.as_str())?;

                client
                    .check_password(password_token, password.trim())
                    .await?;
            }
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        println!("Signed in!");

        match client.session().save_to_file(session_file) {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "NOTE: failed to save the session, will sign out when done: {}",
                    e
                );
                sing_out = true;
            }
        }
    }

    if sing_out {
        // TODO revisit examples and get rid of "handle references" (also, this panics)
        drop(client.sign_out_disconnect().await);
    }

    Ok(client)
}
fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout
        .write_all(message.as_bytes())
        .expect("Couldn't perform write_all");
    stdout.flush().expect("Couldn't perform flush");
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Couldn't perform stdin");
    Ok(line)
}

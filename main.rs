use grammers_client::{Client, Config};
use grammers_session::Session;
use std::{path::Path, time::Duration};
use tokio::time::sleep;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_id: i32 = env::var("API_ID")?.parse()?;
    let api_hash = env::var("API_HASH")?;
    let phone = env::var("PHONE")?;
    let chat_id: i64 = env::var("CHAT_ID")?.parse()?;

    let session_path = Path::new("session");
    let mut client = Client::connect(Config {
        session: Session::load_file_or_create(session_path)?,
        api_id,
        api_hash: api_hash.clone(),
    }).await?;

    if !client.is_authorized().await? {
        let token = client.request_code(&phone, None).await?;
        println!("Введите код из Telegram:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        client.sign_in(&phone, &token, code.trim()).await?;
        client.session().save_to_file(session_path)?;
    }

    println!("✅ Userbot запущен — отправка каждые 4 часа.");

    loop {
        client
            .send_message(chat_id, "фарма")
            .await?;
        println!("💬 Сообщение отправлено!");
        sleep(Duration::from_secs(4 * 60 * 60)).await;
    }
  }

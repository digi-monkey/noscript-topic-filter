use nostr_sdk::prelude::*;
use std::{fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("../.secret")?;
    let mut key = String::new();
    file.read_to_string(&mut key).unwrap();
    let my_keys = Keys::from_sk_str(key.as_str())?;

    let pubkey: String = my_keys.public_key().to_string();
    println!("PubKey: {}", pubkey);

    let client = Client::new(&my_keys);
    client.add_relay("ws://localhost:8080").await?;

    client.connect().await;

    // Send test event
    client.publish_text_note("hello, my test note!", []).await?;

    Ok(())
}

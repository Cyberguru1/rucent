use log;
use rucent::client::{Client, Config};
use rucent::options::with_limit;
use simple_logger::SimpleLogger;
use tokio;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_level("rucent", log::LevelFilter::Trace)
        .init()
        .unwrap();

    let mut config = Config::default();

    config.addr = Some("http://127.0.0.1:8000/api".to_string());
    config.key = Some("fa7ce149-b279-4870-af59-ad7ce78ef11a".to_string());

    let client = Client::new(config);

    let channel = "test_channel".to_string();

    // Publish to test channel

    match client
        .publish(channel.clone(), r#"{"input":"test"}"#, &[])
        .await
    {
        Ok(reply) => log::info!("Publish successful: {:?}", reply),
        Err(e) => log::error!("Publish failed: {:?}", e),
    }

    //. Get history of size 20
    match client.history(channel.clone(), &[with_limit(20)]).await {
        Ok(reply) => log::info!("History fetch successful: {:?}", reply),
        Err(e) => log::error!("History fetch failed: {:?}", e),
    }

    // Get Presense

    match client.presence(channel.clone()).await {
        Ok(reply) => log::info!("Presence fetch successful: {:?}", reply),
        Err(e) => log::error!("Presence fetch failed: {:?}", e),
    }

    // Get Presense Stats
    match client.presence_stats(channel.clone()).await {
        Ok(reply) => log::info!("Presence Stats fetch successful: {:?}", reply),
        Err(e) => log::error!("Presence Stats fetch failed: {:?}", e),
    }

    // Get all channels

    match client.channels(&[]).await {
        Ok(reply) => log::info!("Channels fetch successful: {:?}", reply),
        Err(e) => log::error!("Channels fetch failed: {:?}", e),
    }

    let mut channels = Vec::new();

    for i in 0..10 {
        channels.push(format!("test_channel_{}", i));
    }

    log::info!("{channels:#?}");

    let broadcast_result = client
        .broadcast(channels, r#"{"date": "2024-12-28"}"#, &[])
        .await;
    log::info!("{:#?}", broadcast_result);

}

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio::{task, time};
mod settings;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Environment
    dotenvy::dotenv().ok();
    let username = std::env::var("MQTT_USERNAME")?;
    let passwd = std::env::var("MQTT_PASSWORD")?;
    println!("MQTT_USERNAME: {}", &username);
    println!("MQTT_PASSWORD: {}", &passwd);

    // Settings
    let settings = settings::load();
    println!("{:?}", settings);

    let mut mqttoptions = MqttOptions::new("RqTT", settings.mq_host);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_credentials(&username, passwd.into_bytes());

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe(settings.mq_topic, QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();

            time::sleep(Duration::from_millis(100)).await;
        }
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}

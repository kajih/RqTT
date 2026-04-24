use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio::{task, time};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org");
    mqttoptions.set_keep_alive(5);

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
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
}

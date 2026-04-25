use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub mq_host: String,
    pub mq_port: u16,
    pub mq_topic: String,
}

pub fn load() -> Settings {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("failed to load config")
        .try_deserialize()
        .expect("invalid config")
}
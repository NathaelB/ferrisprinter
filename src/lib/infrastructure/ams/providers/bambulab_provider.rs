use paho_mqtt::Message;
use serde_json::json;
use tracing::info;

use crate::{
    domain::ams::{models::AmsError, ports::provider_ams_service::ProviderAmsService},
    infrastructure::mqtt::mqtt_client::{MqttClient, MqttError},
};

#[derive(Debug, Clone)]
pub struct BambuLabProviderAmsService {
    broker_url: String,
}

impl BambuLabProviderAmsService {
    pub fn new(broker_url: String) -> Self {
        Self { broker_url }
    }

    async fn try_create_mqtt_client(
        &self,
        username: &str,
        password: &str,
    ) -> Result<MqttClient, MqttError> {
        MqttClient::new(&self.broker_url, username, password, "").await
    }
}

impl ProviderAmsService for BambuLabProviderAmsService {
    async fn refresh_rfid(
        &self,
        tray_id: String,
        device_id: String,
        username: String,
        password: String,
    ) -> Result<(), AmsError> {
        let mqtt_client = self
            .try_create_mqtt_client(&username, &password)
            .await
            .unwrap();

        let client = mqtt_client.client;
        let topic = format!("device/{}/request", device_id);

        info!("Publishing message to topic: {}", topic);

        let command_param = format!("M620 R{} \n", tray_id);
        let payload = json!({
          "print": {
            "command": "gcode_line",
            "param": command_param,
            "sequence_id": "0",
          }
        });
        let payload_str = payload.to_string();

        let message = Message::new(topic, payload_str, 1);

        client.publish(message).await.unwrap();
        Ok(())
    }
}

use std::{sync::Arc, time::Duration};

use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, SslOptionsBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqttError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    #[error("Publish failed: {0}")]
    PublishError(String),
    #[error("Subscription failed: {0}")]
    SubscriptionError(String),
    #[error("General MQTT error: {0}")]
    GeneralError(String),
}

pub struct MqttClient {
    pub client: Arc<AsyncClient>,
}

impl MqttClient {
    pub async fn new(
        broker_url: &str,
        username: &str,
        password: &str,
        client_id: &str,
    ) -> Result<Self, MqttError> {
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(broker_url)
            .client_id(client_id)
            .finalize();

        let client = AsyncClient::new(create_opts).map_err(|e| {
            MqttError::ConnectionError(format!("Error creating MQTT client: {}", e))
        })?;

        let conn_opts = ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(60))
            .clean_session(true)
            .user_name(username)
            .password(password)
            .ssl_options(SslOptionsBuilder::new().finalize())
            .finalize();

        client.connect(conn_opts).await.map_err(|e| {
            MqttError::ConnectionError(format!("Failed to connect to broker: {}", e))
        })?;

        Ok(Self {
            client: Arc::new(client),
        })
    }
}

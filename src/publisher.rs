//! eqmq publisher interface.

use crate::config;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions};
use lapin::publisher_confirm::Confirmation;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use tracing::info;

/// Starts producing messages to the broker.
pub async fn run(cfg: &config::Publisher) -> anyhow::Result<()> {
    let conn = Connection::connect(&cfg.endpoint, ConnectionProperties::default()).await?;
    info!("CONNECTED");

    let channel = conn.create_channel().await?;
    info!(state=?conn.status().state());

    channel
        .exchange_declare(
            config::EXCHANGE,
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!("Declared exchange '{}'", config::EXCHANGE);
    info!(state=?conn.status().state());

    channel
        .queue_bind(
            config::QUEUE,
            config::EXCHANGE,
            config::ROUTE,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!("Bound queue '{}'", config::QUEUE);
    info!(state=?conn.status().state());

    info!("will produce");
    for _ in 0..100 {
        let payload = b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        let confirm = channel
            .basic_publish(
                config::EXCHANGE,
                config::ROUTE,
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        info!(state=?conn.status().state());
    }

    Ok(())
}

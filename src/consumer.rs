//! eqmq consumer interface.

use crate::config;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties};
use tracing::info;

/// Starts consuming messages from the broker.
pub async fn run(cfg: &config::Consumer) -> anyhow::Result<()> {
    let conn = Connection::connect(&cfg.endpoint, ConnectionProperties::default()).await?;
    info!("CONNECTED");

    let channel = conn.create_channel().await?;
    info!(state=?conn.status().state());

    let queue = channel
        .queue_declare(
            config::QUEUE,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!(state=?conn.status().state());
    info!(?queue, "Declared");

    info!("will consume");
    let mut consumer = channel
        .basic_consume(
            config::QUEUE,
            config::ROUTE,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!(state=?conn.status().state());

    use futures_lite::StreamExt as _;
    while let Some(delivery) = consumer.next().await {
        info!(message=?delivery, "received message");
        if let Ok(delivery) = delivery {
            delivery.1.ack(BasicAckOptions::default()).await?;
        }
    }

    Ok(())
}

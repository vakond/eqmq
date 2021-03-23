//! eqmq main module.

#![forbid(unsafe_code)]
#![deny(warnings)]

mod cli;
mod config;
mod consumer;
mod publisher;

fn main() {
    if let Err(err) = execute(cli::application()) {
        eprintln!("Error: {:#}", err);
    }
}

/// Dispatches CLI commands.
fn execute(app: cli::Application) -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    match app.cmd {
        cli::Command::Run => {
            let config_file = app.config.unwrap_or("eqmq.json".into());
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(run(&config_file))?;
        }
    }

    tracing::info!("Done.");
    Ok(())
}

/// Runs both workers.
async fn run(config_file: &std::path::Path) -> anyhow::Result<()> {
    let cfg = config::EqMq::load(config_file);
    tokio::try_join!(publisher::run(&cfg.publisher), consumer::run(&cfg.consumer))?;
    Ok(())
}

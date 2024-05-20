#![allow(
    clippy::multiple_crate_versions,
    clippy::unwrap_used,
    clippy::expect_used
)]

use bob_fusion_scheduler::{main::prelude::*, service::SchedulerServer};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = cli::Config::try_from(cli::Args::parse())
        .change_context(AppError::InitializationError)
        .attach_printable("Couldn't get config file.")?;

    SchedulerServer::from_config(config)?.start().await
}

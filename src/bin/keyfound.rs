
use anyhow::Result;
use std::path::Path;
use log::info;
use kfd::{
    config::{Cli, KfdConfig},
    ApiServer,
};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();

    info!("Using config file {}", cli.config_file);

    let kfd_config = KfdConfig::try_from(Path::new(&cli.config_file))?;
    let api_server = ApiServer::new(kfd_config).await?;

    api_server.serve().await?;
    Ok(())
}

use crate::args::{Config, FromFile};
use clap::{crate_authors, crate_version, Parser};
use error_stack::Report;
use error_stack::ResultExt;
use std::path::PathBuf;
use thiserror::Error;

lazy_static::lazy_static! {
    static ref VERSION: String = {
        format!(
            concat!("BOBFUSION VERSION: {}\n",
            "BUILT AT: {}\n",
            "COMMIT HASH: {}\n",
            "GIT BRANCH/TAG: {}\n"),
            crate_version!(),
            build_time::build_time_utc!(),
            option_env!("BOBFUSION_GIT_HASH").unwrap_or("-"),
            option_env!("BOBFUSION_BUILD_BRANCH_TAG").unwrap_or("-"),
        )
    };
}

// Cli Args
#[derive(Debug, Parser, Clone)]
#[command(author = crate_authors!())]
#[command(version = VERSION.trim(), about, long_about)]
#[group(id = "configs", required = true, multiple = false)]
pub struct Args {
    /// If set, passes default configuration to the server
    #[clap(short, long)]
    default: bool,

    /// Server configuration file
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,
}

impl TryFrom<Args> for Config {
    type Error = Report<Error>;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if value.default {
            Ok(Self::default())
        } else if let Some(config) = value.config_file {
            Self::from_file(config).change_context(Error::Config)
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("couldn't get logger configuration")]
    Logger,
    #[error("couldn't get server configuration")]
    Config,
}


use clap::Parser;

use crate::version_five_tx_log;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
/// Process backup files
pub struct EtlCli {
    #[clap(subcommand)]
    pub subcommand: Option<EtlSubcommands>,
}

impl EtlCli {
    pub fn run(&self) -> anyhow::Result<()> {
      match &self.subcommand {
        Some(EtlSubcommands::VersionFive(opts)) => {
          opts.run()
        },
        _ => todo!(),
        }
      Ok(())
    }
}

#[derive(clap::Subcommand)]
pub enum EtlSubcommands {
    VersionFive(V5Opts),
}


#[derive(clap::Args)]
pub struct V5Opts {
  #[clap(short, long)]
  /// hello world
  debug: bool
}

impl V5Opts {
  pub fn run(&self) {
    version_five_tx_log::debug_parse_manifest()
  }
}

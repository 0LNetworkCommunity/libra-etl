
use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
/// Submit a transaction to the blockchain.
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
    println!("hello world");
  }
}

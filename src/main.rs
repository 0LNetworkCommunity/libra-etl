
use libra_etl::cli::EtlCli;
use clap::Parser;

fn main() -> anyhow::Result<()>{
  EtlCli::parse().run()
}

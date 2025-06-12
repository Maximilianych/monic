mod config;

use anyhow::Ok;
use clap::Parser;

use config::Config;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::from_file(&args.config).unwrap();

    println!("Config:\n{:#?}", config);
    Ok(())
}

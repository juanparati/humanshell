use clap::{Arg, Command};
use crate::hs::HS;

mod config;
mod system_info;
mod hs;
mod providers;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("hs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Human to Shell translator")
        .arg(
            Arg::new("expression")
                .help("Human expression to translate")
                .index(1),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .help("Initialize configuration")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("init") {
        config::prompt_for_config();
        return Ok(());
    }

    let hs = HS::new()?;

    if let Some(expression) = matches.get_one::<String>("expression") {
        hs.run_cli_mode(expression).await?;
    } else {
        hs.run_interactive_mode().await?;
    }

    Ok(())
}

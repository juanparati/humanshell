use clap::{Arg, Command};
use crate::hs::HS;

mod config;
mod system_info;
mod hs;
mod providers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("hs")
        .version("1.0.0")
        .about("Human to Shell translator")
        .arg(
            Arg::new("expression")
                .help("Human expression to translate")
                .index(1),
        )
        .get_matches();

    let hs = HS::new()?;

    if let Some(expression) = matches.get_one::<String>("expression") {
        hs.run_interactive(expression).await?;
    } else {
        hs.run_shell_mode().await?;
    }

    Ok(())
}

use ao_core::{AoCore, TypeLogs, build_logger};
use std::io::{self, Write};

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_logger(TypeLogs::Info);

    info!("Ao local CLI interface");

    let mut ao = AoCore::new();

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim_end();

        match ao.execute(input).await? {
            Some(result) if result == "exit" => break,
            Some(result) => info!("Ao: {result}"),
            None => {}
        }
    }

    Ok(())
}

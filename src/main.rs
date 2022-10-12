use std::env;
use minigrep::Config;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;
    config.run()?;
    Ok(())
}

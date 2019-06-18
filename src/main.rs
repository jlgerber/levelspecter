use levelspecter::{LevelSpec, LevelSpecterError};
use std::env;

fn main() -> Result<(), LevelSpecterError> {
    let args = env::args();
    if args.len() < 2 {
        eprintln!("levelspecter <levelspec>");
        std::process::exit(1);
    }
    let args = args.collect::<Vec<_>>();
    let levelspec = LevelSpec::new(&args[1])?;
    println!("{:?}", levelspec);
    Ok(())
}

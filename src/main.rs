use std::process;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1])?;
    } else {
        rlox::run_prompt()?;
    }

    Ok(())
}

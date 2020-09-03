use std::io;
use std::{env, process};

use interpreters::Runner;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let runner = Runner::new();

    if args.len() > 2 {
        println!("--Usage: eksc [script]--");
        process::exit(64);
    } else if args.len() == 2 {
        runner.run_file(&args[1])?;
    } else {
        runner.run_prompt().ok();
        println!("Is Error: {}", *runner.had_error.borrow());
    }
    Ok(())
}

use std::fs;
use std::io::{self, BufRead, Write};

pub fn run_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        stdout.flush()?;

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // EOF
        }

        let line = line.trim_end_matches(&['\n', '\r'][..]);
        run(line);
    }

    Ok(())
}

pub fn run(source: &str) {
    println!("{}", source);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic_on_empty() {
        run("");
    }

    #[test]
    fn run_does_not_panic_on_simple_source() {
        run("print 123;");
    }
}

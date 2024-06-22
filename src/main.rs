use std::io::{self, Write};

fn main() {
    let stdout: io::Stdout = io::stdout();
    let mut std_handler: io::BufWriter<io::Stdout> = io::BufWriter::new(stdout);
    writeln!(std_handler, "Hello world").expect("Error");
}


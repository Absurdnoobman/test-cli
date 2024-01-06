use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut std_handler = io::BufWriter::new(stdout);
    writeln!(std_handler, "Hello world").expect("Error");
}

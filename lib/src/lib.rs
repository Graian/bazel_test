use std::fs;
use std::io::{self, Write};

pub fn copy_and_append(input_file: &str, output_file: &str) -> io::Result<()> {
    fs::copy(input_file, output_file)?;

    let output = format!("[*MT:I] Copied from {:?}", input_file);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)?;
        
    writeln!(file, "\n{:?}", output)
}

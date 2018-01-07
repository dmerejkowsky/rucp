use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{self, Write};
use std::fs::File;

pub const BUFFER_SIZE: usize = 100 * 1024;


pub fn copy(source: &str, destination: &str) -> io::Result<()> {
    println!("{} -> {}", source, destination);
    let src_path = File::open(source)?;
    let mut buf_reader = BufReader::new(src_path);
    let dest_path = File::create(destination)?;
    let mut buf_writer = BufWriter::new(dest_path);
    let mut total = 0;
    let mut buffer = vec![0; BUFFER_SIZE];
    loop {
        let num_read = buf_reader.read(&mut buffer)?;
        if num_read == 0 {
            break;
        }
        buf_writer.write(&buffer[0..num_read])?;
        total += num_read;
        print!("{}\r", total)
    }
    Ok(())
}

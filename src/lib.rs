extern crate tempdir;

const BUFFER_SIZE: usize = 100 * 1024;

use std::fs::File;
use std::io::{self, Write};
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

#[derive(Debug)]
struct TransferRequest<'a> {
    sources: Vec<&'a str>,
    destination: &'a str,
}

fn parse_args(args: &[String]) -> Result<TransferRequest, String> {
    let mut res = TransferRequest{sources: vec![], destination: ""};
    let arg_count = args.len();
    if arg_count < 2 {
        return Err(format!("Usage: {} SRC [SRC ...] DEST", args[0]))
    }
    for i in 1..arg_count-1 {
        res.sources.push(&args[i]);
    }
    res.destination = &args[arg_count-1];
    Ok(res)
}

fn copy(source: &str, destination: &str) -> io::Result<()> {
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

fn do_transfer(request: &TransferRequest) -> io::Result<()> {
    for source in &request.sources {
        copy(source, request.destination)?
    }
    Ok(())
}

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = parse_args(&args)?;
    let transfer_outcome = do_transfer(&request);
    match transfer_outcome {
        Err(err) => return Err(err.to_string()),
        Ok(_) => return Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn parse_args_into_sources_and_dest() {
        let args = vec![String::from("rucp"), String::from("src.txt"), String::from("dest.txt")];
        let transfer_request = parse_args(&args).expect("invalid input");
        assert_eq!(transfer_request.sources, vec!["src.txt"]);
        assert_eq!(transfer_request.destination, "dest.txt");
    }

    #[test]
    fn can_copy_binary_files() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_dir = tmp_dir.path();
        let src_path = tmp_dir.join("src.txt");
        let src_path_name = src_path.to_str().expect("encoding issue");
        let dest_path = tmp_dir.join("dest.txt");
        let dest_path_name = dest_path.to_str().expect("encoding issue");
        let mut src_file = File::create(&src_path).expect("failed to create file");
        let contents = vec![0xba; 10 * BUFFER_SIZE + 123];
        src_file.write_all(&contents).expect("could no write to test file");
        let request = TransferRequest{
            sources: vec![src_path_name],
            destination: dest_path_name,
        };
        do_transfer(&request).expect("should have worked");
        let mut dest_file = File::open(&dest_path).expect("failed to open dest");
        let dest_size :usize = std::fs::metadata(&dest_path).expect("failed to stat dest").len() as usize;
        assert_eq!(dest_size, contents.len());
        let mut actual = vec![0; dest_size];
        dest_file.read(&mut actual).expect("could not read dest");
        assert_eq!(actual, contents);
    }

}

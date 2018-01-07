use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

pub const BUFFER_SIZE: usize = 100 * 1024;


pub fn copy(source: &Path, destination: &Path) -> io::Result<()> {
    println!("{} -> {}", source.to_str().unwrap(), destination.to_str().unwrap());
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::path::{Path,PathBuf};
    use tempdir::TempDir;

    fn setup_test(tmp_path: &Path, contents: &Vec<u8>) -> (PathBuf, PathBuf) {
        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest.txt");
        let mut src_file = File::create(&src_path).expect("failed to create file");
        src_file.write_all(&contents).expect("could no write to test file");
        (src_path, dest_path)
    }

    fn check_copy(dest_path: &PathBuf, contents: &Vec<u8>) {
        let mut dest_file = File::open(&dest_path).expect("failed to open dest");
        let dest_size :usize = fs::metadata(&dest_path).expect("failed to stat dest").len() as usize;
        assert_eq!(dest_size, contents.len());
        let mut actual = vec![0; dest_size];
        dest_file.read(&mut actual).expect("could not read dest");
        assert_eq!(actual, *contents);
    }

    #[test]
    fn copy() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let contents: Vec<u8> = vec![0xba; 10 * BUFFER_SIZE + 123];
        let (src_path, dest_path) = setup_test(tmp_dir.path(), &contents);
        let res = copy(&src_path, &dest_path);
        assert!(res.is_ok());
        check_copy(&dest_path, &contents);
    }
}

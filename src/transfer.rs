use std::io;
use super::copy;

#[derive(Debug)]
pub struct TransferRequest<'a> {
    pub sources: Vec<&'a str>,
    pub destination: &'a str,
}


pub fn do_transfer(request: &TransferRequest) -> io::Result<()> {
    for source in &request.sources {
        copy::copy(source, request.destination)?
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use self::copy::BUFFER_SIZE;
    use std::path::{Path,PathBuf};
    use std::fs::File;
    use std::fs;
    use std::io::{Read,Write};
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
    fn can_copy_binary_files() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let contents : Vec<u8> = vec![0xba; 10 * BUFFER_SIZE + 123];
        let (src_path, dest_path) = setup_test(tmp_dir.path(), &contents);
        let src_str = src_path.to_str().expect("encoding issue");
        let dest_str = dest_path.to_str().expect("encoding issue");
        let request = TransferRequest{
            sources: vec![src_str],
            destination: dest_str
        };
        do_transfer(&request).expect("should have worked");
        check_copy(&dest_path, &contents);
    }
}

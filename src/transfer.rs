use std::io;
use std::path::Path;
use super::copy;

#[derive(Debug)]
pub struct TransferRequest<'a> {
    pub sources: Vec<&'a str>,
    pub destination: &'a str,
}

// FIXME: it should not be possible to build a ValidRequest from scratch ...
pub struct ValidRequest<'a> {
    pub sources: Vec<&'a str>,
    pub destination: &'a str,
}

pub fn validate(request: TransferRequest) -> Result<ValidRequest, String> {
    for source in &request.sources {
        let path = Path::new(source);
        if !path.exists() {
            return Err(format!("{} does not exist", source));
        }
    }
    let dest_path = Path::new(request.destination);
    if (request.sources.len() > 1) && (!dest_path.is_dir()) {
        return Err(format!("{} should be a directory", request.destination));
    }
    Ok(
        ValidRequest{
            sources: request.sources,
            destination: request.destination
        }
    )
}

pub fn do_transfer(request: &ValidRequest) -> io::Result<()> {
    for source in &request.sources {
        copy::copy(source, request.destination)?
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::fs;
    use tempdir::TempDir;
    use std::path::PathBuf;

    fn build_request<'a>(paths: &'a Vec<&PathBuf>) -> TransferRequest<'a> {
        let mut res = TransferRequest{sources: vec![], destination: ""};
        for (i, path) in paths.iter().enumerate() {
            let as_str = path.to_str().expect("could not encode path name");
            if i != paths.len() - 1 {
                res.sources.push(as_str);
            } else {
                res.destination = as_str;
            }
        }
        res
    }

    #[test]
    fn validate_file_file() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest.txt");

        File::create(&src_path).expect("failed to create src file");
        File::create(&dest_path).expect("failed to create dest file");

        let paths = vec![&src_path, &dest_path];
        let request = build_request(&paths);
        let valid_request = validate(request);
        assert!(valid_request.is_ok());

    }

    #[test]
    fn validate_source_exist() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest.txt");
        File::create(&dest_path).expect("failed to create dest file");

        let paths = vec![&src_path, &dest_path];
        let request = build_request(&paths);
        let error = validate(request).err().unwrap();
        assert_eq!(error, format!("{} does not exist", src_path.to_str().unwrap()));
    }

    #[test]
    fn validate_dest_is_dir_when_several_sources() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let one_path = tmp_path.join("one.txt");
        let two_path = tmp_path.join("two.txt");
        let three_path = tmp_path.join("three.txt");

        File::create(&one_path).expect("failed to create one.txt file");
        File::create(&two_path).expect("failed to create two.txt file");
        File::create(&three_path).expect("failed to create dest.txt file");


        let paths = vec![&one_path, &two_path, &three_path];
        let request = build_request(&paths);
        let error = validate(request).err().unwrap();
        assert_eq!(error, format!("{} should be a directory", three_path.to_str().unwrap()));
    }

    #[test]
    fn validate_files_dir() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let one_path = tmp_path.join("one.txt");
        let two_path = tmp_path.join("two.txt");
        let dest_path = tmp_path.join("dest");

        File::create(&one_path).expect("failed to create one.txt file");
        File::create(&two_path).expect("failed to create two.txt file");
        fs::create_dir(&dest_path).expect("faile to create dest dir");


        let paths = vec![&one_path, &two_path, &dest_path];
        let request = build_request(&paths);
        let valid_request = validate(request);
        assert!(valid_request.is_ok());
    }
}

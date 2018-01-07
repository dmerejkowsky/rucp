use super::copy;
use std;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

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

#[derive(Debug,PartialEq)]
enum TransferStep {
    MkDir(PathBuf),
    Cp(PathBuf, PathBuf),
}

pub struct Transfer {
    steps: Vec<TransferStep>,
}

fn get_file_name(path: &Path) -> Result<&std::ffi::OsStr, String> {
    let maybe_str = path.to_str();
    let as_str;
    match maybe_str {
        None => return Err(format!("Could not convert {:?} as string", path)),
        Some(s) => as_str = s
    }
    let res = path.file_name();
    match res {
        None => Err(format!("{} has no name", as_str)),
        Some(name) => Ok(name),
    }
}

impl<'a> ValidRequest<'a> {
    /*
     * file - file -> copy(file, file)
     * files - dir -> copy (file, dir/$(baseanme file)
     *  dir - dir -> mkdir, copy, mkdir, copy ...
     */
    pub fn compute_transfer(&self) -> Result<Transfer, String> {
        if self.sources.len() == 1 {
            self.compute_transfer_one_source()
        } else {
            self.compute_transfer_several_sources()
        }
    }

    fn compute_transfer_one_source(&self) -> Result<Transfer, String> {
        let src_path = Path::new(self.sources[0]).to_path_buf();
        let dest_path = Path::new(self.destination).to_path_buf();
        let full_dest_path;
        if dest_path.is_dir() {
            let file_name = get_file_name(&src_path)?;
            full_dest_path = dest_path.join(file_name);
        } else {
            full_dest_path = dest_path;
        }
        let step = TransferStep::Cp(src_path, full_dest_path);
        Ok(Transfer{steps: vec![step]})
    }

    fn compute_transfer_several_sources(&self) -> Result<Transfer, String> {
        let mut steps = vec![];
        let dest_path = Path::new(self.destination).to_path_buf();
        for source in &self.sources {
            let src_path = Path::new(source).to_path_buf();
            let file_name = get_file_name(&src_path)?;
            let full_dest_path = dest_path.clone().join(file_name);
            let step = TransferStep::Cp(src_path.clone(), full_dest_path);
            steps.push(step);
        }
        Ok(Transfer{steps: steps})
    }
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

pub fn do_transfer(transfer: Transfer) -> io::Result<()> {
    for step in transfer.steps {
        match step {
            TransferStep::Cp(source, destination) =>
                copy::copy(&source, &destination)?,
            TransferStep::MkDir(dest) =>
                fs::create_dir(dest)?,
        }
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

    fn get_transfer<'a>(paths: &'a Vec<&PathBuf>) -> Transfer {
        let request = build_request(&paths);
        let valid_request = validate(request).unwrap();
        let transfer = valid_request.compute_transfer();
        let transfer = transfer.expect("compute_transfer failed");
        return transfer;
    }

    fn assert_error<'a>(paths: &'a Vec<&PathBuf>, path: &'a PathBuf, error: &str) {
        let request = build_request(&paths);
        let res = validate(request);
        let actual_error = res.err().unwrap();
        let expected_error = format!("{} {}", path.to_str().unwrap(), error);
        assert_eq!(actual_error, expected_error);
    }


    #[test]
    fn compute_file_file() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest.txt");

        File::create(&src_path).expect("failed to create src file");
        File::create(&dest_path).expect("failed to create dest file");

        let paths = vec![&src_path, &dest_path];
        let transfer = get_transfer(&paths);

        assert_eq!(transfer.steps.len(), 1);
        let actual_step = &transfer.steps[0];
        let expected = TransferStep::Cp(
            src_path.clone(),
            dest_path.clone()
        );
        assert_eq!(actual_step, &expected);
    }

    #[test]
    fn compute_file_directory() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest");

        File::create(&src_path).expect("failed to create src file");
        fs::create_dir(&dest_path).expect("failed to create dest dir");

        let paths = vec![&src_path, &dest_path];
        let transfer = get_transfer(&paths);
        assert_eq!(transfer.steps.len(), 1);

        let full_dest_path = tmp_path.join("dest/src.txt");
        let actual_step = &transfer.steps[0];
        let expected = TransferStep::Cp(
            src_path.clone(),
            full_dest_path.clone()
        );
        assert_eq!(actual_step, &expected);
    }
    #[test]
    fn source_does_not_exist() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src.txt");
        let dest_path = tmp_path.join("dest.txt");
        File::create(&dest_path).expect("failed to create dest file");

        let paths = vec![&src_path, &dest_path];

        assert_error(&paths, &src_path, "does not exist");
    }

    #[test]
    fn several_sources_but_dest_not_a_directory() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let one_path = tmp_path.join("one.txt");
        let two_path = tmp_path.join("two.txt");
        let three_path = tmp_path.join("three.txt");

        File::create(&one_path).expect("failed to create one.txt file");
        File::create(&two_path).expect("failed to create two.txt file");
        File::create(&three_path).expect("failed to create dest.txt file");


        let paths = vec![&one_path, &two_path, &three_path];

        assert_error(&paths, &three_path, "should be a directory");
    }

    #[test]
    fn serveral_sources() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let one_path = tmp_path.join("one.txt");
        let two_path = tmp_path.join("two.txt");
        let dest_path = tmp_path.join("dest");

        File::create(&one_path).expect("failed to create one.txt file");
        File::create(&two_path).expect("failed to create two.txt file");
        fs::create_dir(&dest_path).expect("faile to create dest dir");


        let paths = vec![&one_path, &two_path, &dest_path];
        let transfer = get_transfer(&paths);


        let actual_steps = &transfer.steps;
        let expected_steps = vec![
            TransferStep::Cp(one_path.clone(), dest_path.join("one.txt")),
            TransferStep::Cp(two_path.clone(), dest_path.join("two.txt")),
        ];

        assert_eq!(actual_steps, &expected_steps);
    }

    /*
    #[test]
    fn subdirs() {
        let tmp_dir = TempDir::new("test-rucp").expect("failed to create temp dir");
        let tmp_path = tmp_dir.path();

        let src_path = tmp_path.join("src");
        fs::create_dir(&src_path).expect("failed to create src dir");

        let sub_path = src_path.join("sub");
        fs::create_dir(&sub_path).expect("failed to create sub path");

        let a_path = sub_path.join("a.txt");

        let dest_path = tmp_path.join("dest");
        fs::create_dir(&dest_path).expect("failed to create dest path");

        let paths = vec![&src_path, &dest_path];
        let transfer = get_transfer(&paths);


        let actual_steps = &transfer.steps;

        let expected_steps = vec![
            TransferStep::MkDir(dest_path.clone().join("sub")),
            TransferStep::Cp(a_path.clone(), dest_path.join("sub/a.txt")),
        ];

        assert_eq!(actual_steps, &expected_steps);
    }
    */

}

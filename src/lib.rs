extern crate tempdir;

mod args;
mod transfer;
mod copy;


pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = args::parse_args(&args)?;
    let transfer_outcome = transfer::do_transfer(&request);
    match transfer_outcome {
        Err(err) => return Err(err.to_string()),
        Ok(_) => return Ok(())
    }
}

#[cfg(test)]
mod test {



}

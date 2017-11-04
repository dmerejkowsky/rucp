#[derive(Debug)]
struct TransferRequest<'a> {
    sources: Vec<&'a str>,
    destination: &'a str,
}

impl<'a> TransferRequest<'a> {
    pub fn new(args: &[String]) -> Result<TransferRequest, String> {
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
}

fn do_transfer(request: &TransferRequest) -> Result<(), String> {
    for source in &request.sources {
        println!("{} -> {}", source, request.destination)
    }
    Ok(())
}

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = TransferRequest::new(&args)?;
    do_transfer(&request)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sources_and_dest() {
        let input = vec![String::from("rucp"), String::from("src.txt"), String::from("dest.txt")];
        let transfer_request = TransferRequest::new(&input).expect("invalid input");
        assert_eq!(transfer_request.sources, vec!["src.txt"]);
        assert_eq!(transfer_request.destination, "dest.txt");
    }

}
